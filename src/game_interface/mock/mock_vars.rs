//! Implementation of a mock backend for [`GameInterface`]
//!
//! This may be useful for testing logic against some known state.

use bytemuck::CheckedBitPattern;
use strum::IntoEnumIterator;

use crate::{
    endian::EndianAware,
    game_interface::{
        game_var::{GameVar, GameVarMut, InterfaceBackend},
        GameInterface, PowerUps, Task, Tasks,
    },
    game_state::{GameMode, GameOstrich, GameState},
    Level, Spatula,
};

/// A mock implementation for [`InterfaceBackend`]
pub enum MockBackend {}
impl InterfaceBackend for MockBackend {
    type Var<T: CheckedBitPattern + EndianAware> = MockVar<T>;
    type Mut<T: CheckedBitPattern + EndianAware> = MockVar<T>;
}

impl Default for GameInterface<MockBackend> {
    fn default() -> Self {
        Self {
            is_loading: MockVar::default(),
            game_state: MockVar::new(GameState::Play),
            game_mode: MockVar::new(GameMode::Game),
            game_ostrich: MockVar::new(GameOstrich::InScene),
            powers: PowerUps::new(),
            scene_id: MockVar::new(Level::SpongebobHouse.into()),
            spatula_count: MockVar::default(),
            tasks: Tasks::new(),
            lab_door_cost: MockVar::new(74),
        }
    }
}

impl Tasks<MockBackend> {
    fn new() -> Self {
        Self {
            arr: Spatula::iter()
                .map(|s| {
                    (
                        s,
                        Task {
                            menu_count: MockVar::default(),
                            flags: Some(MockVar::default()),
                            state: Some(MockVar::default()),
                        },
                    )
                })
                .collect(),
        }
    }
}

impl PowerUps<MockBackend> {
    fn new() -> Self {
        Self {
            bubble_bowl: MockVar::default(),
            cruise_bubble: MockVar::default(),
            initial_bubble_bowl: MockVar::default(),
            initial_cruise_bubble: MockVar::default(),
        }
    }
}

/// A mock implementation for [`GameVar`] and [`GameVarMut`]
#[derive(Default)]
pub struct MockVar<T> {
    /// The mocked value. Accessing the value here directly allows setting up a scenario for a test,
    /// including the ability to mutate [`GameVar`]s that normally are immuatble.
    pub value: T,
}

impl<T> MockVar<T> {
    fn new(value: T) -> Self {
        Self { value }
    }
}

impl<T: CheckedBitPattern + EndianAware> GameVar for MockVar<T> {
    type Target = T;

    fn get(&self) -> crate::game_interface::InterfaceResult<Self::Target> {
        Ok(self.value)
    }
}

impl<T: CheckedBitPattern + EndianAware> GameVarMut for MockVar<T> {
    fn set(&mut self, value: Self::Target) -> crate::game_interface::InterfaceResult<()> {
        self.value = value;
        Ok(())
    }
}
