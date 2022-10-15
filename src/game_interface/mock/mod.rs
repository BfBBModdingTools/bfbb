//! A mock backend for [`GameInterface`](super::GameInterface)

use self::mock_vars::MockBackend;

use super::{GameInterface, GameInterfaceProvider, InterfaceResult};

pub mod mock_vars;

/// Provider for [`GameInterface::DolphinBackend`]
#[derive(Default)]
pub struct MockInterface {
    interface: GameInterface<MockBackend>,
}
impl GameInterfaceProvider for MockInterface {
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
