use bfbb::game_interface::{
    dolphin::DolphinInterface,
    game_var::{GameVar, GameVarMut, InterfaceBackend},
    mock::MockInterface,
    GameInterface, InterfaceProvider, InterfaceResult,
};

/// While it's unlikely that you'd need to use two separate [`GameInterface`](GameInterface)s at the same time,
/// this example shows how you might write logic that is generic over a `GameInterface`'s backend.
///
/// Run this example with dolphin open and BfBB running to see the example output.
fn main() -> InterfaceResult<()> {
    // Setup a dolphin interface
    let mut dolphin = DolphinInterface::default();
    // Setup a mock interface in place of a hypothetical interface for Xemu
    let mut xemu = MockInterface::default();

    // Run the same logic using two different backends.
    // In practice you would select an available backend at runtime and only use it.
    println!("Dolphin:");
    dolphin.do_with_interface(takes_generic_and_adds_spatulas)?;
    println!("Xemu:");
    xemu.do_with_interface(takes_generic_and_adds_spatulas)
}

/// Since this function has the signature expected by `InterfaceProvider::do_with_interface`
/// we can just pass it directly.
fn takes_generic_and_adds_spatulas<I: InterfaceBackend>(
    interface: &mut GameInterface<I>,
) -> InterfaceResult<()> {
    let count = interface.spatula_count.get()?;
    println!("\tYou have {count} spatulas, adding 10 now",);
    interface.spatula_count.set(count + 10)
}
