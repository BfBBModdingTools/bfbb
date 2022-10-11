use log::{debug, error, trace};
use process_memory::{Memory, ProcessHandle, TryIntoProcessHandle};
use sysinfo::{ProcessExt, System, SystemExt};
use thiserror::Error;

use crate::game_interface::{GameInterface, InterfaceError, InterfaceResult};
use crate::game_state::{GameMode, GameOstrich, GameState};
use crate::{Level, Spatula};

use super::data_member::{CheckedMemory, DataMember};

const REGION_SIZE: usize = 0x2000000;

#[cfg(target_os = "linux")]
const PROCESS_NAME: &str = "dolphin-emu";
#[cfg(target_os = "windows")]
const PROCESS_NAME: &str = "Dolphin";

/// Error type for failures to interact with Dolphin's memory.
#[derive(Debug, Error)]
pub enum Error {
    /// The BfBB memory-region could not be found.
    #[error("The emulated memory region could not be found. Make sure Dolphin is running and the game is open.")]
    RegionNotFound,
    /// An error occurred while reading/writing to Dolphin's memory
    #[error("Dolphin memory could not be accessed.")]
    IO,
}

impl From<std::io::Error> for Error {
    fn from(_: std::io::Error) -> Self {
        Self::IO
    }
}

/// A [GameInterface](crate::game_interface::GameInterface) implemented for Dolphin running on the same local machine.
///
/// # Example Usage
/// ```
/// use bfbb::game_interface::{ GameInterface, dolphin::DolphinInterface };
/// use bfbb::Spatula;
///
/// let mut game = DolphinInterface::default();
/// if game.hook().is_ok() {
///     game.mark_task_complete(Spatula::SpongebobsCloset);
/// }
/// ```
#[derive(Default)]
pub struct DolphinInterface {
    system: System,
    base_address: Option<usize>,
    handle: Option<ProcessHandle>,
}

impl DolphinInterface {
    /// Attempt to hook Dolphin
    ///
    /// Dolphin is considered "hooked" when it's process is found and the region of memory used
    /// for emulating the GameCube's memory is located. This method will always attempt to hook
    /// Dolphin when called, even if already hooked.
    pub fn hook(&mut self) -> Result<(), Error> {
        // TODO: Differentiate errors between Dolphin not being found and Dolphin being found, but the game not being open.
        // We might be rehooking, so we should "unhook" to prevent ending up in an invalid state if rehooking fails
        self.handle = None;
        self.base_address = None;

        self.system.refresh_processes();

        let procs = self.system.processes_by_name(PROCESS_NAME);
        if let Some((pid, addr)) = procs
            .into_iter()
            .map(|p| {
                let pid = p.pid();
                trace!("{} found with pid {pid}", p.name());
                (pid, get_emulated_base_address(pid))
            })
            .find_map(|(pid, addr)| addr.map(|addr| (pid, addr)))
        {
            debug!("Found emulated memory region at {addr:#X}");
            self.base_address = Some(addr);

            // Convert sysinfo Pid (wrapper type) to process_memory Pid (platform specific alias)
            #[cfg(target_os = "windows")]
            // Work around for sysinfo crate using usize on windows for Pids
            let pid = <sysinfo::Pid as Into<usize>>::into(pid) as u32;

            #[allow(clippy::useless_conversion)]
            // This isn't uselss on *nix
            let pid: process_memory::Pid = pid.into();
            self.handle = Some(pid.try_into_process_handle()?);
            return Ok(());
        }

        Err(Error::RegionNotFound)
    }
}

const LOADING_ADDRESS: usize = 0x803CB7B0;
const GAME_STATE_ADDRESS: usize = 0x803CAB43;
const GAME_MODE_ADDRESS: usize = 0x803CB8AB;
const GAME_OSTRICH_ADDRESS: usize = 0x803CB8AF;
const POWERS_ADDRESS: usize = 0x803C0F17;
const SCENE_PTR_ADDRESS: usize = 0x803C2518;
const SPATULA_COUNT_ADDRESS: usize = 0x803C205C;
const SWORLD_BASE: usize = 0x802F63C8;
const LAB_DOOR_ADDRESS: usize = 0x804F6CB8;

// TODO: Cache DataMembers; they contain a Vec so it isn't the best idea to be making new ones
//       every time we interact with the game.
impl GameInterface for DolphinInterface {
    fn is_loading(&self) -> InterfaceResult<bool> {
        let ptr = self.get_ptr(vec![LOADING_ADDRESS])?;
        Ok(u32::from_be(ptr.checked_read()?) != 0)
    }

    fn start_new_game(&self) -> InterfaceResult<()> {
        let ptr = self.get_ptr(vec![GAME_MODE_ADDRESS])?;
        ptr.write(&GameMode::Game)?;
        Ok(())
    }

    fn unlock_powers(&self) -> InterfaceResult<()> {
        let ptr = self.get_ptr::<[u8; 2]>(vec![POWERS_ADDRESS])?;
        ptr.write(&[1, 1])?;
        Ok(())
    }

    fn get_current_game_mode(&self) -> InterfaceResult<GameMode> {
        let ptr = self.get_ptr(vec![GAME_MODE_ADDRESS])?;
        Ok(ptr.checked_read()?)
    }

    fn set_game_mode(&self, new_mode: GameMode) -> InterfaceResult<()> {
        let ptr = self.get_ptr(vec![GAME_MODE_ADDRESS])?;
        ptr.write(&new_mode)?;
        Ok(())
    }

    fn get_current_game_ostrich(&self) -> InterfaceResult<GameOstrich> {
        let ptr = self.get_ptr(vec![GAME_OSTRICH_ADDRESS])?;
        Ok(ptr.checked_read()?)
    }

    fn get_current_game_state(&self) -> InterfaceResult<GameState> {
        let ptr = self.get_ptr(vec![GAME_STATE_ADDRESS])?;
        Ok(ptr.checked_read()?)
    }

    fn set_game_state(&self, new_state: GameState) -> InterfaceResult<()> {
        let ptr = self.get_ptr(vec![GAME_STATE_ADDRESS])?;
        ptr.write(&new_state)?;
        Ok(())
    }

    fn get_current_level(&self) -> InterfaceResult<Level> {
        let ptr = self.get_ptr::<[u8; 4]>(vec![SCENE_PTR_ADDRESS, 0])?;

        // This address is a multicharacter literal, so we don't need to convert endianness
        ptr.checked_read()?
            .try_into()
            .map_err(|_| InterfaceError::Other)
    }

    fn get_spatula_count(&self) -> InterfaceResult<u32> {
        let ptr = self.get_ptr(vec![SPATULA_COUNT_ADDRESS])?;

        Ok(u32::from_be(ptr.checked_read()?))
    }

    fn set_spatula_count(&self, value: u32) -> InterfaceResult<()> {
        let ptr = self.get_ptr(vec![SPATULA_COUNT_ADDRESS])?;
        ptr.write(&value.to_be())?;
        Ok(())
    }

    fn unlock_task(&self, spatula: Spatula) -> InterfaceResult<()> {
        let (world_idx, idx) = spatula.into();

        // TODO: reduce magic numbers
        let mut base = SWORLD_BASE;
        base += world_idx as usize * 0x24C;
        base += 0xC;
        base += idx as usize * 0x48;
        base += 0x14;

        let ptr = self.get_ptr(vec![base, 0x14])?;

        let curr = i16::from_be(ptr.checked_read()?);
        if curr != 2 {
            ptr.write(&1i16.to_be())?;
        }
        Ok(())
    }

    fn mark_task_complete(&self, spatula: Spatula) -> InterfaceResult<()> {
        let (world_idx, idx) = spatula.into();

        // TODO: reduce magic numbers
        let mut base = SWORLD_BASE;
        base += world_idx as usize * 0x24C;
        base += 0xC;
        base += idx as usize * 0x48;
        base += 0x14;

        let ptr = self.get_ptr(vec![base, 0x14])?;
        ptr.write(&2i16.to_be())?;
        Ok(())
    }

    fn is_task_complete(&self, spatula: Spatula) -> InterfaceResult<bool> {
        let (world_idx, idx) = spatula.into();

        // TODO: reduce magic numbers
        let mut base = SWORLD_BASE;
        base += world_idx as usize * 0x24C;
        base += 0xC;
        base += idx as usize * 0x48;
        base += 0x14;

        let ptr = self.get_ptr(vec![base, 0x14])?;
        Ok(i16::from_be(ptr.checked_read()?) == 2)
    }

    fn collect_spatula(
        &self,
        spatula: Spatula,
        current_level: Option<Level>,
    ) -> InterfaceResult<()> {
        if current_level != Some(spatula.get_level()) {
            return Ok(());
        }

        // TODO: reduce magic numbers
        let offset = match spatula.get_offset() {
            Some(offset) => offset * 4,
            None => return Ok(()),
        };

        // This prevents writing to incorrect memory for KahRahTae and SmallShallRule
        if offset == 0 {
            return Ok(());
        }

        let ptr_flags = self.get_ptr::<u8>(vec![SCENE_PTR_ADDRESS, 0x78, offset, 0x18])?;
        let ptr_state = self.get_ptr::<u32>(vec![SCENE_PTR_ADDRESS, 0x78, offset, 0x16C])?;

        let mut flags = ptr_flags.checked_read()?;
        flags &= !1; // Disable the entity

        // Set some model flags
        let mut state = u32::from_be(ptr_state.checked_read()?);
        state |= 8;
        state &= !4;
        state &= !2;

        ptr_flags.write(&flags.to_be())?;
        ptr_state.write(&state.to_be())?;
        Ok(())
    }

    fn is_spatula_being_collected(
        &self,
        spatula: Spatula,
        current_level: Option<Level>,
    ) -> InterfaceResult<bool> {
        if current_level != Some(spatula.get_level()) {
            return Ok(false);
        }

        // TODO: reduce magic numbers
        let offset = match spatula.get_offset() {
            Some(offset) => offset * 4,
            None => return Ok(false),
        };

        let ptr = self.get_ptr(vec![SCENE_PTR_ADDRESS, 0x78, offset, 0x16C])?;

        Ok(u32::from_be(ptr.checked_read()?) & 4 != 0)
    }

    fn set_lab_door(&self, value: u32, current_level: Option<Level>) -> InterfaceResult<()> {
        if current_level != Some(Level::ChumBucket) {
            return Ok(());
        }

        let ptr = self.get_ptr(vec![LAB_DOOR_ADDRESS])?;

        // The game uses a greater than check so we need to subtract by one
        let cost = value - 1;
        ptr.write(&cost.to_be())?;
        Ok(())
    }
}

impl DolphinInterface {
    fn get_ptr<T>(&self, offsets: Vec<usize>) -> InterfaceResult<DataMember<T>> {
        Ok(DataMember::new_offset(
            self.handle.ok_or(InterfaceError::Unhooked)?,
            self.base_address.ok_or(InterfaceError::Unhooked)?,
            offsets,
        ))
    }
}

#[cfg(target_os = "linux")]
fn get_emulated_base_address(pid: sysinfo::Pid) -> Option<usize> {
    use proc_maps::get_process_maps;
    let maps = match get_process_maps(pid.into()) {
        Err(e) => {
            error!("Could not get dolphin process maps\n{e:?}");
            return None;
        }
        Ok(maps) => maps,
    };

    // Multiple maps exist that fit our criteria who only differ by address.
    // Perhaps by chance, the last match appears to always have the correct address.
    let map = maps.iter().rev().find(|m| {
        m.size() == REGION_SIZE
            && m.filename()
                .map(|f| f.to_string_lossy().contains("dolphin-emu"))
                .unwrap_or(false)
    });
    map.map(|m| m.start())
}

#[cfg(target_os = "windows")]
fn get_emulated_base_address(pid: sysinfo::Pid) -> Option<usize> {
    use winapi::um::handleapi::CloseHandle;
    use winapi::um::memoryapi::VirtualQueryEx;
    use winapi::um::processthreadsapi::OpenProcess;
    use winapi::um::psapi::{QueryWorkingSetEx, PSAPI_WORKING_SET_EX_INFORMATION};
    use winapi::um::winnt::{
        MEMORY_BASIC_INFORMATION, MEM_MAPPED, PROCESS_QUERY_INFORMATION, PROCESS_VM_OPERATION,
        PROCESS_VM_READ, PROCESS_VM_WRITE,
    };

    unsafe {
        let handle = OpenProcess(
            PROCESS_QUERY_INFORMATION | PROCESS_VM_OPERATION | PROCESS_VM_READ | PROCESS_VM_WRITE,
            0,
            <sysinfo::Pid as Into<usize>>::into(pid) as u32,
        );
        if handle.is_null() {
            // TODO use GetLastError for error feedback
            error!("Failed to open process handle for dolphin (pid {pid})");
            return None;
        }

        // Begin memory scan for Dolphin's emulated memory region
        // We are looking for a MEM_MAPPED region of size 0x2000000
        let mut mem_info = MEMORY_BASIC_INFORMATION::default();
        let mut mem = std::ptr::null();
        while VirtualQueryEx(
            handle,
            mem,
            &mut mem_info,
            std::mem::size_of::<MEMORY_BASIC_INFORMATION>(),
        ) == std::mem::size_of::<MEMORY_BASIC_INFORMATION>()
        {
            if mem_info.RegionSize == REGION_SIZE && mem_info.Type == MEM_MAPPED {
                let mut ws_info = PSAPI_WORKING_SET_EX_INFORMATION {
                    VirtualAddress: mem_info.BaseAddress,
                    ..Default::default()
                };
                if QueryWorkingSetEx(
                    handle,
                    &mut ws_info as *mut _ as *mut std::ffi::c_void,
                    std::mem::size_of::<PSAPI_WORKING_SET_EX_INFORMATION>()
                        .try_into()
                        .unwrap(),
                ) != 0
                    && ws_info.VirtualAttributes.Valid() != 0
                {
                    return Some(mem_info.BaseAddress as usize);
                }
            }

            mem = mem.add(mem_info.RegionSize);
        }

        CloseHandle(handle);
    }

    None
}
