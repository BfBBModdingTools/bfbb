//! Provides traits necessary to implement or use a [`GameInterface`](super::GameInterface)
//!
//! # Examples
//! ```
//! use bfbb::game_interface::game_var::{GameVar, GameVarMut, InterfaceBackend};
//! use bfbb::game_interface::{GameInterface, InterfaceResult};
//!  
//! fn do_thing<V: InterfaceBackend> (interface: &mut GameInterface<V>) -> InterfaceResult<()> {
//!     // Here you can access game variables or perform actions on the interface.
//!     interface.spatula_count.get()?;
//!     interface.start_new_game()
//! }
//! ```

use bytemuck::CheckedBitPattern;

use crate::endian::EndianAware;

use super::InterfaceResult;

/// Allows accessing the value of a variable within BfBB
pub trait GameVar {
    /// The underlying type of this variable.
    type Target;
    /// Attempt to get value of this variable.
    ///
    /// # Errors
    ///
    /// May return an [`InterfaceError`](super::InterfaceError) if the value can not be read.
    fn get(&self) -> InterfaceResult<Self::Target>;
}

/// Allows mutating a variable within BfBB
pub trait GameVarMut: GameVar {
    /// Attempt to set the value of this variable.
    ///
    /// # Errors
    ///
    /// May return an [`InterfaceError`](super::InterfaceError) if the value can not be read.
    fn set(&mut self, value: Self::Target) -> InterfaceResult<()>;
}

/// A trait with type constructors for [`GameVar`]s that are generic over an implementation (backend).
///
/// For [`GameInterface`](super::GameInterface) implementations, this trait should be implemented on a marker struct
/// and provide its [`GameVar`] implementation to the type constructors. (See [`DolphinBackend`](crate::game_interface::dolphin::dolphin_var::DolphinBackend)).
///
/// For users, you will need to use this trait to write code that is generic over any `GameInterface` implementation.
///
/// # Examples
/// ```
/// use bfbb::game_interface::game_var::InterfaceBackend;
/// use bfbb::game_interface::GameInterface;
///
/// fn takes_generic<V: InterfaceBackend>(interface: &mut GameInterface<V>) {
///     todo!();
/// }
/// ```
pub trait InterfaceBackend {
    /// Type constructor for an immutable [`GameVar`]
    type Var<T: CheckedBitPattern + EndianAware>: GameVar<Target = T>;
    /// Type constructor for a mutable [`GameVar`]
    type Mut<T: CheckedBitPattern + EndianAware>: GameVarMut<Target = T>;
}
