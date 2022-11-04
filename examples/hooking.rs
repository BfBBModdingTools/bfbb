use std::thread;
use std::time::Duration;

use bfbb::game_interface::dolphin::DolphinInterface;
use bfbb::game_interface::game_var::{GameVar, GameVarMut, InterfaceBackend};
use bfbb::game_interface::{GameInterface, InterfaceError, InterfaceProvider, InterfaceResult};
use bfbb::game_state::GameState;

fn main() -> InterfaceResult<()> {
    // First we need to setup our backend provider.
    // This simply sets up a object capable of hooking dolphin and providing a GameInterface,
    // but doesn't actually do anything yet.
    let mut dolphin = DolphinInterface::default();

    // The return type of `with_interface` automatically adjusts to any InterfaceResult, so you can
    // easily extract data from within the game. (Notice the `?` operator on the returned value of
    // `with_interace`, rather than within the closure.)
    let spatula_count = dolphin.do_with_interface(|interface| interface.spatula_count.get())?;
    println!("You have {spatula_count} spatulas.");

    loop {
        // `with_interface` automatically takes care of hooking the active Dolphin process, including
        // rehooking if a prior `InterfaceError::Unhooked` error is encountered within the provided function
        // This loop should continue to run indefinitely and automatically adjust to Dolphin being closed and reopened.
        let res = dolphin.do_with_interface(mod_logic);
        match res {
            Ok(()) => println!("GameState was successfully updated"),
            Err(InterfaceError::Unhooked) => println!("GameInterface became unhooked"),
            // A different error occurred, break and report it.
            Err(e) => println!("{e:?}"),
        }
        thread::sleep(Duration::from_secs(2));
    }
}

fn mod_logic<F: InterfaceBackend>(interface: &mut GameInterface<F>) -> InterfaceResult<()> {
    interface.game_state.set(GameState::Exit)
}
