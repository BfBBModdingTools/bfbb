use std::error::Error;

use bfbb::game_interface::{
    dolphin::Dolphin,
    game_var::{GameVar, GameVarFamily, GameVarMut},
    mock::mock_vars::MockVarFamily,
    GameInterface,
};

/// While it's unlikely that you'd need to use two separate [`GameInterface`](GameInterface)s at the same time,
/// this example shows how you might write logic that is generic over a `GameInterface`'s backend.
///
/// Run this example with dolphin open and BfBB running to see the example output.
fn main() -> Result<(), Box<dyn Error>> {
    // Setup a dolphin interface
    let mut dolphin = Dolphin::default();
    let dol_interface = dolphin.get_interface()?;

    // Set a known starting value differing from our other interface
    dol_interface.spatula_count.set(3)?;

    // We don't yet have a real second interface so we'll use a mock.
    // Pretend this is an implementation for an Xemu-based GameInterface
    let mut xemu_interface = GameInterface::<MockVarFamily>::new();

    println!("Dolphin:");
    takes_generic_and_sets_spatula_count(dol_interface, 16)?;
    println!("Xemu:");
    takes_generic_and_sets_spatula_count(&mut xemu_interface, 20)?;

    println!("\nAfter:");
    println!(
        "\tDolphin has {} spatulas",
        dol_interface.spatula_count.get()?
    );
    println!(
        "\tXemu has {} spatulas",
        xemu_interface.spatula_count.get()?
    );
    Ok(())
}

fn takes_generic_and_sets_spatula_count<I: GameVarFamily>(
    interface: &mut GameInterface<I>,
    new_count: u32,
) -> Result<(), Box<dyn Error>> {
    println!(
        "\tYou have {} spatulas, updating count to {new_count}",
        interface.spatula_count.get()?
    );
    interface.spatula_count.set(new_count)?;
    Ok(())
}
