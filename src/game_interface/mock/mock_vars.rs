//! Implemenation of mock backend for [`GameInterface`]
//!
//! This may be useful for testing logic against some known state.

use std::collections::HashMap;

use bytemuck::CheckedBitPattern;
use strum::IntoEnumIterator;

use crate::{
    game_interface::{
        dolphin::EndianAware,
        game_var::{GameVar, GameVarFamily, GameVarMut},
        GameInterface, Task, Tasks,
    },
    game_state::{GameMode, GameOstrich, GameState},
    Spatula,
};

/// A mock implemenation for [`GameVarFamily`]
pub struct MockVarFamily;
impl GameVarFamily for MockVarFamily {
    type Var<T: CheckedBitPattern + EndianAware> = MockVar<T>;
    type Mut<T: CheckedBitPattern + EndianAware> = MockVar<T>;
}

impl Default for GameInterface<MockVarFamily> {
    fn default() -> Self {
        Self {
            is_loading: MockVar::default(),
            game_state: MockVar::new(GameState::FirstTime),
            game_mode: MockVar::new(GameMode::Boot),
            game_ostrich: MockVar::new(GameOstrich::InScene),
            intial_powers: MockVar::default(),
            scene_id: MockVar::default(),
            spatula_count: MockVar::default(),
            tasks: Tasks::<MockVarFamily>::new(),
            lab_door_cost: MockVar::default(),
        }
    }
}

impl Tasks<MockVarFamily> {
    fn new() -> Self {
        Self {
            arr: HashMap::from_iter(Spatula::iter().map(|s| {
                (
                    s,
                    Task {
                        menu_count: MockVar::default(),
                        flags: Some(MockVar::default()),
                        state: Some(MockVar::default()),
                    },
                )
            })),
        }
    }
}

/// A mock implementation for [`GameVar`] and [`GameVarMut`]
#[derive(Default)]
pub struct MockVar<T> {
    value: T,
}

impl<T> MockVar<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: CheckedBitPattern + EndianAware> GameVar for MockVar<T> {
    type T = T;

    fn get(&self) -> crate::game_interface::InterfaceResult<Self::T> {
        Ok(self.value)
    }
}

impl<T: CheckedBitPattern + EndianAware> GameVarMut for MockVar<T> {
    fn set(&mut self, value: Self::T) -> crate::game_interface::InterfaceResult<()> {
        Ok(self.value = value)
    }
}