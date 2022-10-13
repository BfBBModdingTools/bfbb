//! Allows for performing actions on or reading information about a running instance of BfBB.

use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

use thiserror::Error;

use crate::{
    game_state::{GameMode, GameOstrich, GameState},
    Level, Spatula,
};

use self::game_var::{GameVar, GameVarFamily, GameVarMut};

pub mod dolphin;
pub mod game_var;
pub mod mock;

/// Error type for failed [`GameInterface`] actions.
///
/// This list is non-exhaustive and may grow over time.
#[derive(Copy, Clone, Debug, Error)]
#[non_exhaustive]
pub enum InterfaceError {
    /// Error for when the interface isn't hooked to the game.
    #[error("Interface became unhooked")]
    Unhooked,
    /// Error for when an action fails for any other reason.
    #[error("Interface operation failed")]
    Other,
}

impl From<std::io::Error> for InterfaceError {
    fn from(e: std::io::Error) -> Self {
        // For now, treat any error other than InvalidData as being unhooked
        if e.kind() == std::io::ErrorKind::InvalidData {
            return Self::Other;
        }
        Self::Unhooked
    }
}

/// Result type for [`GameInterface`] actions.
pub type InterfaceResult<T> = std::result::Result<T, InterfaceError>;
/// Interact with BfBB in an abstract way.
///
//// This struct allows accessing variables existing with a running instance of *Battle for Bikini Bottom*
/// and performing actions on that instance in a way that is generic over a backend (e.g Dolphin and Xemu)
///
/// NOTE: To a see a list of supported backends see the `Implementors` list of [`GameVarFamily`]
///
/// This is the key struct of the [`game_interface`](self) module and enables interacting with the game.
#[non_exhaustive]
pub struct GameInterface<F: GameVarFamily> {
    /// True while on loading screens
    pub is_loading: F::Var<bool>,
    /// Location of the [`GameState`] enum.
    pub game_state: F::Mut<GameState>,
    /// Location of the [`GameState`] enum.
    pub game_mode: F::Mut<GameMode>,
    /// Location of the [`GameOstrich`] enum.
    ///
    /// Not recommended to mutate this, but the option is available if you wish, it's probably not what you want to do.
    pub game_ostrich: F::Mut<GameOstrich>,
    /// Array for whether a new game should start with powers enabled. First element is the bubble-bowl and the second is the cruise-bubble.
    pub powers: PowerUps<F>,
    /// Location of the ID for the current scene. Can be converted to a [`Level`](crate::Level) via [`TryFrom`].
    ///
    /// # Examples
    /// ```
    /// use std::error::Error;
    ///
    /// use bfbb::game_interface::GameInterface;
    /// use bfbb::game_interface::game_var::{GameVar, GameVarFamily};
    /// use bfbb::Level;
    ///
    /// fn get_current_level<V: GameVarFamily>(
    ///     interface: &mut GameInterface<V>,
    /// ) -> Result<Level, Box<dyn Error>> {
    ///     Ok(interface.scene_id.get()?.try_into()?)
    /// }
    /// ```
    pub scene_id: F::Var<[u8; 4]>,
    /// Location of the spatula counter
    pub spatula_count: F::Mut<u32>,
    /// [`Tasks`]
    pub tasks: Tasks<F>,

    // TODO: This value is on the heap, it shouldn't be global like this
    lab_door_cost: F::Mut<u32>,
}

impl<F: GameVarFamily> GameInterface<F> {
    /// Will start a new game when called. Only works when the player is on the main menu and not in the demo cutscene.
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError`] if the implementation is unable to access the game.
    pub fn start_new_game(&mut self) -> InterfaceResult<()> {
        self.game_mode.set(GameMode::Game)
    }

    /// Get the level that the player is currently in
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError::Other`] sometimes when the game is loading due to the scene pointer being null
    pub fn get_current_level(&self) -> InterfaceResult<Level> {
        // TODO: It would be great to eventually allow self.scene_id to be of type `F::Var<Level>` directly and be able
        //       to blanket impl `GameVar<T=Level>` for all backends by using an actual `GameVar<T=[u8;4]>` internally.
        self.scene_id
            .get()?
            .try_into()
            .map_err(|_| InterfaceError::Other)
    }

    /// Marks a task as available (Silver). This will not update an already unlocked task.
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError`] if the implementation is unable to access the game.
    pub fn unlock_task(&mut self, spatula: Spatula) -> InterfaceResult<()> {
        let task = &mut self.tasks[spatula];
        let curr = task.menu_count.get()?;
        if curr != 2 {
            task.menu_count.set(1)?;
        }
        Ok(())
    }

    /// Marks a spatula as "completed" in the pause menu. This has the effect of giving the player access to the task warp.
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError`] if the implementation is unable to access the game.
    pub fn mark_task_complete(&mut self, spatula: Spatula) -> InterfaceResult<()> {
        self.tasks[spatula].menu_count.set(2)
    }

    /// True when `spatula` is shown as gold in the pause menu.
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError`] if the implementation is unable to access the game.
    pub fn is_task_complete(&self, spatula: Spatula) -> InterfaceResult<bool> {
        Ok(self.tasks[spatula].menu_count.get()? == 2)
    }

    /// Collect a spatula in the world. This only removes the entity, it will not complete the task or increment the spatula
    /// counter.
    ///
    /// *NOTE*: Will always return `Ok(false)` when `spatula` is located in a level other than `current_level` or when
    /// `spatula` is [Kah-Rah-Tae](Spatula::KahRahTae) or [The Small Shall Rule... Or Not](Spatula::TheSmallShallRuleOrNot)
    /// without writing memory.
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError`] if the implementation is unable to access the game.
    pub fn collect_spatula(
        &mut self,
        spatula: Spatula,
        current_level: Option<Level>,
    ) -> InterfaceResult<()> {
        if current_level != Some(spatula.get_level()) {
            return Ok(());
        }

        let task = &mut self.tasks[spatula];
        let (flags, state) = match (&mut task.flags, &mut task.state) {
            (Some(flags), Some(state)) => (flags, state),
            _ => return Ok(()),
        };

        let mut new_flags = flags.get()?;
        new_flags &= !1; // Disable the entity

        // Set some model flags
        let mut new_state = state.get()?;
        new_state |= 8;
        new_state &= !4;
        new_state &= !2;

        flags.set(new_flags)?;
        state.set(new_state)?;
        Ok(())
    }

    /// True when `spatula`'s collected animation is playing
    ///
    /// *NOTE*: Will always return `Ok(false)` when `spatula` is located in a level other than `current_level` or when
    /// `spatula` is [Kah-Rah-Tae](Spatula::KahRahTae) or [The Small Shall Rule... Or Not](Spatula::TheSmallShallRuleOrNot)
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError`] if the implementation is unable to access the game.
    pub fn is_spatula_being_collected(
        &self,
        spatula: Spatula,
        current_level: Option<Level>,
    ) -> InterfaceResult<bool> {
        if current_level != Some(spatula.get_level()) {
            return Ok(false);
        }

        let state = match &self.tasks[spatula].state {
            Some(x) => x,
            None => return Ok(false),
        };

        Ok(state.get()? & 4 != 0)
    }

    /// Changes the number of spatulas required to enter the Chum Bucket Lab.
    ///
    /// *NOTE*: This function requires that the current level is the Chum Bucket an will therefore always return `Ok(())`
    /// if `current_level` is not `Some(Level::ChumBucket)`
    ///
    /// # Errors
    ///
    /// Will return an [`InterfaceError`] if the implementation is unable to access the game.
    pub fn set_lab_door(
        &mut self,
        value: u32,
        current_level: Option<Level>,
    ) -> InterfaceResult<()> {
        if current_level != Some(Level::ChumBucket) {
            return Ok(());
        }

        // The game uses a greater than check so we need to subtract by one
        let cost = value - 1;
        self.lab_door_cost.set(cost)?;
        Ok(())
    }
}

/// A collection of [`Task`]s. Can be indexed by [`Spatula`]
///
/// # Examples
///
/// ```
/// use bfbb::game_interface::game_var::{GameVar, GameVarMut, GameVarFamily};
/// use bfbb::game_interface::{InterfaceResult, Tasks};
/// use bfbb::Spatula;
///
/// fn unlock_pinapple<V: GameVarFamily>(tasks: &mut Tasks<V>) -> InterfaceResult<()> {
///     tasks[Spatula::OnTopOfThePineapple].menu_count.set(1)
/// }
/// ```
pub struct Tasks<F: GameVarFamily> {
    pub(crate) arr: HashMap<Spatula, Task<F>>,
}

/// Contains [`GameVar`]s for a [`Spatula`]'s pause-menu counter and game object state.
#[non_exhaustive]
pub struct Task<F: GameVarFamily> {
    /// The `count` field of this task's `_xCounter` struct.
    ///
    /// Notable values are:
    /// - `0` => Task is "locked", will be a question mark in the menu.
    /// - `1` => Task is "incomplete", will be a silver spatula in the menu.
    /// - `2` => Task is "complete", will be a golden spatula in the menu.
    /// - `3` => Task is also silver in the menu, may have some semantics not currently understood.
    /// - `_` => No icon will appear for this task in the menu, just an empty bubble. You can not warp to it and
    ///          attempting to will put the menu into an invalid state until a different unlocked task is selected.
    pub menu_count: F::Mut<i16>,
    /// A bitfield of flags for a spatula entity. The first bit determines if the entity is enabled or not.
    pub flags: Option<F::Mut<u8>>,
    /// Another bitfield for a spatula entity.
    pub state: Option<F::Mut<u32>>,
}

impl<V: GameVarFamily> Index<Spatula> for Tasks<V> {
    type Output = Task<V>;

    fn index(&self, index: Spatula) -> &Self::Output {
        &self.arr[&index]
    }
}

impl<T: GameVarFamily> IndexMut<Spatula> for Tasks<T> {
    fn index_mut(&mut self, index: Spatula) -> &mut Self::Output {
        self.arr.get_mut(&index).unwrap()
    }
}

/// [`GameVar`]s related to the bubble bowl and cruise-missile
#[non_exhaustive]
pub struct PowerUps<F: GameVarFamily> {
    /// Whether the bubble bowl is currently unlocked
    pub bubble_bowl: F::Mut<bool>,
    /// Whether the cruise bubble is currently unlocked
    pub cruise_bubble: F::Mut<bool>,
    /// Whether new games should begin with the bubble bowl unlocked.
    pub initial_bubble_bowl: F::Mut<bool>,
    /// Whether new games should begin with the cruise missile
    pub initial_cruise_bubble: F::Mut<bool>,
}

impl<F: GameVarFamily> PowerUps<F> {
    /// Set whether a new game should start with powers or not (New Game+)
    ///
    /// # Errors
    ///
    /// Will return a [`InterfaceError`] if the implementation is unable to access the game.
    pub fn start_with_powers(&mut self, value: bool) -> InterfaceResult<()> {
        self.initial_bubble_bowl.set(value)?;
        self.initial_cruise_bubble.set(value)
    }

    /// Unlock the Bubble Bowl and Cruise Bubble for the current game.
    ///
    /// # Errors
    ///
    /// Will return a [`InterfaceError`] if the implementation is unable to access the game.
    pub fn unlock_powers(&mut self) -> InterfaceResult<()> {
        self.bubble_bowl.set(true)?;
        self.cruise_bubble.set(true)
    }
}
