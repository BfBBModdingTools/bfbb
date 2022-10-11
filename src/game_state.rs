//! Enums that represent various parts of the game's current state
//!
//! See also: [decomp](https://github.com/bfbbdecomp/bfbb/blob/master/src/Game/zGameState.h)

use bytemuck::CheckedBitPattern;
use strum_macros::EnumCount;

/// While unsure what the name actually means, this enum is primarily useful for checking if
/// we're in a scene or not.
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumCount, CheckedBitPattern)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(u8)]
pub enum GameOstrich {
    /// Used when a loading screen is active
    Loading = 0,

    /// Used when an FMV is playing
    PlayingMovie = 1,

    /// Used when a scene is loaded and active
    InScene = 2,
}

/// More macro-level game state.
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumCount, CheckedBitPattern)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(u8)]
pub enum GameMode {
    /// Active when the game first starts, until the Logo splash-screens appear
    Boot = 0,

    /// Logo splash-screen FMVs
    Intro = 1,

    /// On main menu
    Title = 2,

    /// Unknown, appears unused
    Start = 3,

    /// Load Save-Game Screen, transitioning to this while on the main menu will open the load game menu.
    Load = 4,

    /// Unknown, appears unused
    Options = 5,

    /// Used for the Save-Game menu, transitioning to this will usually open the save menu
    Save = 6,

    /// In pause menu
    Pause = 7,

    /// Unknown, appears unused
    Stall = 8,

    /// Unknown, appears unused
    WorldMap = 9,

    /// Unknown, Likely left-over from Scooby-Doo Night of 100 Frights
    MonsterGallery = 10,

    /// Unknown, not used by the concept art gallery in the Theater.
    ConceptArtGallery = 11,

    /// Active while playing in-game and unpaused. Transitioning to this state while on the main-menu
    /// will cause a new game to start.
    Game = 12,
}

/// Used mostly for tracking states during gameplay.
#[derive(Copy, Clone, Debug, PartialEq, Eq, EnumCount, CheckedBitPattern)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[repr(u8)]
pub enum GameState {
    /// Used while on the title screen.
    FirstTime = 0,
    /// Used during general play.
    Play = 1,
    /// Transitioning to this state will reload the current level as if SpongeBob has died.
    LoseChance = 2,
    /// Appears unused. Transitioning to this state will trigger a fade-out to black followed by a return
    /// to the main menu.
    GameOver = 3,
    /// Unknown, Appears unused
    GameStats = 4,
    /// Used when loading a new level. Manually transitioning to this state seems to always crash the game.
    SceneSwitch = 5,
    /// Appears unused. Transitioning to this state looks to have the same effect as [`Exit`](Self::Exit)
    Dead = 6,
    /// Transitioning to this state will cause the game to return to the main menu.
    Exit = 7,
}
