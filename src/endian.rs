use crate::game_state::{GameMode, GameOstrich, GameState};

pub trait EndianAware {
    const NEEDS_SWAP: bool;
}

macro_rules! impl_aware {
    ( false: $( $f:ty ),*; true: $( $t:ty ),* )=> {
        $( impl_aware!(false, $f); )*
        $( impl_aware!(true,  $t); )*
    };
    ( $needs_swap:expr, $typ:ty) => {
        impl EndianAware for $typ {
            const NEEDS_SWAP: bool = $needs_swap;
        }
    };
}

impl_aware!(false: bool, u8, GameMode, GameState, GameOstrich; true: i16, u32);
impl<const N: usize> EndianAware for [u8; N] {
    const NEEDS_SWAP: bool = false;
}
