//! Provides traits necessary to implement or use a [`GameInterface`](super::GameInterface)
//!
//! # Examples
//! ```
//! use bfbb::game_interface::game_var::{GameVar, GameVarMut, GameVarFamily};
//! use bfbb::game_interface::{GameInterface, InterfaceResult};
//!  
//! fn do_thing<V: GameVarFamily> (interface: &mut GameInterface<V>) -> InterfaceResult<()> {
//!     // Here you can access game variables or perform actions on the interface.
//!     interface.spatula_count.get()?;
//!     interface.start_new_game()
//! }
//! ```

use bytemuck::CheckedBitPattern;

use crate::endian::EndianAware;

use super::InterfaceResult;

/// Allows accessing the value of variable within BfBB
pub trait GameVar {
    /// The underlying type of this variable.
    type T;
    /// Attempt to get value of this variable.
    fn get(&self) -> InterfaceResult<Self::T>;
}

/// Allows mutating a variable within BfBB
pub trait GameVarMut: GameVar {
    /// Attempt to set the value of this variable.
    fn set(&mut self, value: Self::T) -> InterfaceResult<()>;
}

/// A trait to create type constructors for [`GameVar`]s that are generic over an implementation
///
/// For [`GameInterface`](super::GameInterface) implementations, this trait should be implemented on a marker struct
/// and provide its [`GameVar`] implementation to the type constructors. (See [`DolphinVarFamily`](crate::game_interface::dolphin::dolphin_var::DolphinVarFamily)).
///
/// For users, you will need to use this trait to write code that is generic over any `GameInterface` implementation.
///
/// # Examples
/// ```
/// use bfbb::game_interface::game_var::GameVarFamily;
/// use bfbb::game_interface::GameInterface;
///
/// fn takes_generic<V: GameVarFamily>(interface: &mut GameInterface<V>) {
///     todo!();
/// }
/// ```
pub trait GameVarFamily {
    /// Type constructor for an immutable GameVar
    type Var<T: CheckedBitPattern + EndianAware>: GameVar<T = T>;
    /// Type constructor for a mutable GameVar
    type Mut<T: CheckedBitPattern + EndianAware>: GameVarMut<T = T>;
}
