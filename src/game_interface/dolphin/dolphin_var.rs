//! Implementation of Dolphin backend for [`GameInterface`]

use bytemuck::CheckedBitPattern;
use process_memory::{Memory, ProcessHandle};
use strum::IntoEnumIterator;

use crate::{
    endian::EndianAware,
    game_interface::{
        game_var::{GameVar, GameVarMut, InterfaceBackend},
        GameInterface, InterfaceResult, PowerUps, Task, Tasks,
    },
    Spatula,
};

use super::data_member::{CheckedMemory, DataMember};

/// Dolphin implementation for [`InterfaceBackend`]
pub enum DolphinBackend {}
impl InterfaceBackend for DolphinBackend {
    type Var<T: CheckedBitPattern + EndianAware> = DolphinVar<T>;
    type Mut<T: CheckedBitPattern + EndianAware> = DolphinVar<T>;
}

/// Dolphin implementation for [`GameVar`] and [`GameVarMut`]
pub struct DolphinVar<T> {
    ptr: DataMember<T>,
}

impl<T> DolphinVar<T> {
    /// Create a new [`GameVar`] given a location in the game's memory-space.
    ///
    /// You can use this as an "escape-hatch" to access variables not implemented in the library.
    /// In such a case, please consider also making a PR to integrate that variable into the library if it
    /// may feasibly be useful more broadly.
    ///
    /// Since this takes an arbitrary pointer-path and the game's memory is backed by regular system memory owned by Dolphin,
    /// you can technically use this to modify more than just game data. For example, you could theoretically patch/re-write
    /// instructions in the game's executable code, though this use-case is currently not prioritized and may pose other issues,
    /// such as not having any method to ensure a patch is made before the game starts executing (Consider if a Dolphin AR code will work for
    /// your use-case instead.).
    pub fn new(addr: impl Into<Vec<usize>>, base_addr: usize, handle: ProcessHandle) -> Self {
        Self {
            ptr: DataMember::new_offset(handle, base_addr, addr.into()),
        }
    }
}

impl<T: EndianAware + CheckedBitPattern> GameVar for DolphinVar<T> {
    type Target = T;
    fn get(&self) -> InterfaceResult<T> {
        Ok(self.ptr.checked_read()?)
    }
}
impl<T: EndianAware + CheckedBitPattern> GameVarMut for DolphinVar<T> {
    fn set(&mut self, value: T) -> InterfaceResult<()> {
        Ok(self.ptr.write(&value)?)
    }
}

const LOADING_ADDRESS: usize = 0x803C_B7B3;
const GAME_STATE_ADDRESS: usize = 0x803C_AB43;
const GAME_MODE_ADDRESS: usize = 0x803C_B8AB;
const GAME_OSTRICH_ADDRESS: usize = 0x803C_B8AF;
const SCENE_PTR_ADDRESS: usize = 0x803C_2518;
const SPATULA_COUNT_ADDRESS: usize = 0x803C_205C;
const LAB_DOOR_ADDRESS: usize = 0x804F_6CB8;

impl GameInterface<DolphinBackend> {
    pub(crate) fn new(base_addr: usize, handle: ProcessHandle) -> Self {
        Self {
            is_loading: DolphinVar::new([LOADING_ADDRESS], base_addr, handle),
            game_state: DolphinVar::new([GAME_STATE_ADDRESS], base_addr, handle),
            game_mode: DolphinVar::new([GAME_MODE_ADDRESS], base_addr, handle),
            game_ostrich: DolphinVar::new([GAME_OSTRICH_ADDRESS], base_addr, handle),
            powers: PowerUps::new(base_addr, handle),
            scene_id: DolphinVar::new([SCENE_PTR_ADDRESS, 0], base_addr, handle),
            spatula_count: DolphinVar::new([SPATULA_COUNT_ADDRESS], base_addr, handle),
            tasks: Tasks::new(base_addr, handle),
            lab_door_cost: DolphinVar::new([LAB_DOOR_ADDRESS], base_addr, handle),
        }
    }
}

const SWORLD_BASE: usize = 0x802F_63C8;
impl Tasks<DolphinBackend> {
    fn new(base_addr: usize, handle: ProcessHandle) -> Self {
        const SIZE_OF_MENU_WORLD: usize = 0x24C;
        const SIZE_OF_MENU_TASK: usize = 0x48;

        let arr = Spatula::iter()
            .map(|s| {
                // Calcuate address of the _xCounter for this task in the menu
                let (world, idx) = s.into();
                let counter_addr =
                    SWORLD_BASE + world * SIZE_OF_MENU_WORLD + 0xC + idx * SIZE_OF_MENU_TASK + 0x14;

                let offset = s.get_offset().map(|x| x * std::mem::size_of::<u32>());

                (
                    s,
                    Task {
                        menu_count: DolphinVar::new([counter_addr, 0x14], base_addr, handle),
                        flags: offset.map(|x| {
                            DolphinVar::new([SCENE_PTR_ADDRESS, 0x78, x, 0x18], base_addr, handle)
                        }),
                        state: offset.map(|x| {
                            DolphinVar::new([SCENE_PTR_ADDRESS, 0x78, x, 0x16C], base_addr, handle)
                        }),
                    },
                )
            })
            .collect();
        Self { arr }
    }
}

const POWERS_ADDRESS: usize = 0x803C_0F15;
impl PowerUps<DolphinBackend> {
    fn new(base_addr: usize, handle: ProcessHandle) -> Self {
        Self {
            bubble_bowl: DolphinVar::new([POWERS_ADDRESS], base_addr, handle),
            cruise_bubble: DolphinVar::new([POWERS_ADDRESS + 1], base_addr, handle),
            initial_bubble_bowl: DolphinVar::new([POWERS_ADDRESS + 2], base_addr, handle),
            initial_cruise_bubble: DolphinVar::new([POWERS_ADDRESS + 3], base_addr, handle),
        }
    }
}
