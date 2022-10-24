//! A mock backend for [`GameInterface`](super::GameInterface)

use std::ops::{Deref, DerefMut};

use self::mock_vars::MockBackend;

use super::{GameInterface, InterfaceProvider, InterfaceResult};

pub mod mock_vars;

/// Provider for [`GameInterface<DolphinBackend>`]
///
/// Implements [`Deref`] and [`DerefMut`] for [`GameInterface<MockBackend>`] as there is no hooking
/// behavior to be abstracted away in this implementation. This allows convenient access of game-state
/// when writing tests.
#[derive(Default)]
pub struct MockInterface {
    interface: GameInterface<MockBackend>,
}
impl InterfaceProvider for MockInterface {
    type Backend = MockBackend;

    fn do_with_interface<T>(
        &mut self,
        fun: impl FnOnce(&mut GameInterface<Self::Backend>) -> InterfaceResult<T>,
    ) -> InterfaceResult<T> {
        fun(&mut self.interface)
    }

    fn is_available(&mut self) -> bool {
        true
    }
}

impl Deref for MockInterface {
    type Target = GameInterface<MockBackend>;

    fn deref(&self) -> &Self::Target {
        &self.interface
    }
}
impl DerefMut for MockInterface {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.interface
    }
}
