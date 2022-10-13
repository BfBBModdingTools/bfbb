use strum_macros::{EnumCount, EnumIter};

use crate::Level;

/// Convenient definition for every spatula in the game
///
/// # Menu coordinates:
/// `Spatula` has support for interpeting a `(usize, usize)` value as a coordinate from the in-game menu,
/// where the first coordinate is the level-index and the second coordinate is the spatula-index,
/// and convert it to a `Spatula` value (or vice-versa).
///
/// ```
/// use bfbb::Spatula;
///
/// let coord = (0,3);
/// let spatula = Spatula::try_from(coord).expect("'(0,3)' should be Spongebob's Closet");
/// assert_eq!(spatula, Spatula::SpongebobsCloset);
///
/// let closet = Spatula::SpongebobsCloset;
/// let coord: (usize, usize) = closet.into();
/// assert_eq!(coord, (0,3))
/// ```

#[derive(EnumIter, EnumCount, Hash, Debug, Clone, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Spatula {
    // Bikini Bottom
    #[doc(hidden)]
    OnTopOfThePineapple,
    #[doc(hidden)]
    OnTopOfShadyShoals,
    #[doc(hidden)]
    OnTopOfTheChumBucket,
    #[doc(hidden)]
    SpongebobsCloset,
    #[doc(hidden)]
    AnnoySquidward,
    #[doc(hidden)]
    AmbushAtTheTreeDome,
    #[doc(hidden)]
    InfestationAtTheKrustyKrab,
    #[doc(hidden)]
    AWallJumpInTheBucket,

    // Jellyfish Fields
    #[doc(hidden)]
    TopOfTheHill,
    #[doc(hidden)]
    CowaBungee,
    #[doc(hidden)]
    Spelunking,
    #[doc(hidden)]
    PatricksDilemma,
    #[doc(hidden)]
    NavigateTheCanyonsAndMesas,
    #[doc(hidden)]
    DrainTheLake,
    #[doc(hidden)]
    SlideLeap,
    #[doc(hidden)]
    DefeatKingJellyfish,

    // Downtown Bikini Bottom
    #[doc(hidden)]
    EndOfTheRoad,
    #[doc(hidden)]
    LearnSandysMoves,
    #[doc(hidden)]
    TikisGoBoom,
    #[doc(hidden)]
    AcrossTheRooftops,
    #[doc(hidden)]
    SwinginSandy,
    #[doc(hidden)]
    AmbushInTheLighthouse,
    #[doc(hidden)]
    ExtremeBungee,
    #[doc(hidden)]
    ComeBackWithTheCruiseBubble,

    // Goo Lagoon
    #[doc(hidden)]
    KingOfTheCastle,
    #[doc(hidden)]
    ConnectTheTowers,
    #[doc(hidden)]
    SaveTheChildren,
    #[doc(hidden)]
    OverTheMoat,
    #[doc(hidden)]
    ThroughTheSeaCaves,
    #[doc(hidden)]
    CleanOutTheBumperBoats,
    #[doc(hidden)]
    SlipAndSlideUnderThePier,
    #[doc(hidden)]
    TowerBungee,

    // Poseidome
    #[doc(hidden)]
    RumbleAtThePoseidome,

    // Rock Bottom
    #[doc(hidden)]
    GetToTheMuseum,
    #[doc(hidden)]
    SlipSlidingAway,
    #[doc(hidden)]
    ReturnTheMuseumsArt,
    #[doc(hidden)]
    SwingalongSpatula,
    #[doc(hidden)]
    PlunderingRobotsInTheMuseum,
    #[doc(hidden)]
    AcrossTheTrenchOfDarkness,
    #[doc(hidden)]
    LasersAreFunAndGoodForYou,
    #[doc(hidden)]
    HowInTarnationDoYouGetThere,

    // Mermalair
    #[doc(hidden)]
    TopOfTheEntranceAreaML,
    #[doc(hidden)]
    TopOfTheComputerArea,
    #[doc(hidden)]
    ShutDownTheSecuritySystem,
    #[doc(hidden)]
    TheFunnelMachines,
    #[doc(hidden)]
    TheSpinningTowersOfPower,
    #[doc(hidden)]
    TopOfTheSecurityTunnel,
    #[doc(hidden)]
    CompleteTheRollingBallRoom,
    #[doc(hidden)]
    DefeatPrawn,

    // Sand Mountain
    #[doc(hidden)]
    FrostyBungee,
    #[doc(hidden)]
    TopOfTheLodge,
    #[doc(hidden)]
    DefeatRobotsOnGuppyMound,
    #[doc(hidden)]
    BeatMrsPuffsTime,
    #[doc(hidden)]
    DefeatRobotsOnFlounderHill,
    #[doc(hidden)]
    BeatBubbleBuddysTime,
    #[doc(hidden)]
    DefeatRobotsOnSandMountain,
    #[doc(hidden)]
    BeatLarrysTime,

    // Industrial Park
    #[doc(hidden)]
    RoboPatrickAhoy,

    // Kelp Forest
    #[doc(hidden)]
    ThroughTheWoods,
    #[doc(hidden)]
    FindAllTheLostCampers,
    #[doc(hidden)]
    TikiRoundup,
    #[doc(hidden)]
    DownInTheSwamp,
    #[doc(hidden)]
    ThroughTheKelpCaves,
    #[doc(hidden)]
    PowerCrystalCrisis,
    #[doc(hidden)]
    KelpVineSlide,
    #[doc(hidden)]
    BeatMermaidMansTime,

    // Flying Dutchman's Graveyard
    #[doc(hidden)]
    TopOfTheEntranceAreaFDG,
    #[doc(hidden)]
    APathThroughTheGoo,
    #[doc(hidden)]
    GooTankerAhoy,
    #[doc(hidden)]
    TopOfTheStackOfShips,
    #[doc(hidden)]
    ShipwreckBungee,
    #[doc(hidden)]
    DestroyTheRobotShip,
    #[doc(hidden)]
    GetAloftThereMatey,
    #[doc(hidden)]
    DefeatTheFlyingDutchman,

    // SpongeBob's Dream
    #[doc(hidden)]
    AcrossTheDreamscape,
    #[doc(hidden)]
    FollowTheBouncingBall,
    #[doc(hidden)]
    SlidingTexasStyle,
    #[doc(hidden)]
    SwingersAhoy,
    #[doc(hidden)]
    MusicIsInTheEarOfTheBeholder,
    #[doc(hidden)]
    KrabbyPattyPlatforms,
    #[doc(hidden)]
    SuperBounce,
    #[doc(hidden)]
    HereYouGo,

    // Chum Bucket Lab
    #[doc(hidden)]
    KahRahTae,
    #[doc(hidden)]
    TheSmallShallRuleOrNot,
}

impl Spatula {
    /// In-game, spatula objects are stored within an array of entities.
    /// `get_offset` gets the index of a spatula within that entity array.
    ///
    /// **NOTE:** These indexes are only valid while the spatula's level is loaded.
    ///
    /// **NOTE:** This list is currently only validated for the Gamecube version of the game.
    pub fn get_offset(&self) -> Option<usize> {
        #[allow(clippy::match_same_arms)]
        match *self {
            Self::OnTopOfThePineapple => Some(0xa8),
            Self::OnTopOfShadyShoals => Some(0xcf),
            Self::OnTopOfTheChumBucket => Some(0xd0),
            Self::SpongebobsCloset => Some(0x5d),
            Self::AnnoySquidward => Some(0x26),
            Self::AmbushAtTheTreeDome => Some(0x3a),
            Self::InfestationAtTheKrustyKrab => Some(0xce),
            Self::AWallJumpInTheBucket => Some(0x2a),
            Self::TopOfTheHill => Some(0xc8),
            Self::CowaBungee => Some(0xc9),
            Self::Spelunking => Some(0xd8),
            Self::PatricksDilemma => Some(0xd7),
            Self::NavigateTheCanyonsAndMesas => Some(0xfa),
            Self::DrainTheLake => Some(0xea),
            Self::SlideLeap => Some(0x58),
            Self::DefeatKingJellyfish => Some(0x128),
            Self::EndOfTheRoad => Some(0xba),
            Self::LearnSandysMoves => Some(0xb9),
            Self::TikisGoBoom => Some(0x111),
            Self::AcrossTheRooftops => Some(0xab),
            Self::SwinginSandy => Some(0xac),
            Self::AmbushInTheLighthouse => Some(0x53),
            Self::ExtremeBungee => Some(0x99),
            Self::ComeBackWithTheCruiseBubble => Some(0x9a),
            Self::KingOfTheCastle => Some(0x12a),
            Self::ConnectTheTowers => Some(0x154),
            Self::SaveTheChildren => Some(0x153),
            Self::OverTheMoat => Some(0x12b),
            Self::ThroughTheSeaCaves => Some(0x5c),
            Self::CleanOutTheBumperBoats => Some(0xff),
            Self::SlipAndSlideUnderThePier => Some(0xfd),
            Self::TowerBungee => Some(0xfe),
            Self::RumbleAtThePoseidome => Some(0x28),
            Self::GetToTheMuseum => Some(0xff),
            Self::SlipSlidingAway => Some(0xfe),
            Self::ReturnTheMuseumsArt => Some(0x105),
            Self::SwingalongSpatula => Some(0x107),
            Self::PlunderingRobotsInTheMuseum => Some(0x76),
            Self::AcrossTheTrenchOfDarkness => Some(0xa5),
            Self::LasersAreFunAndGoodForYou => Some(0xa4),
            Self::HowInTarnationDoYouGetThere => Some(0xa3),
            Self::TopOfTheEntranceAreaML => Some(0x72),
            Self::TopOfTheComputerArea => Some(0x6a),
            Self::ShutDownTheSecuritySystem => Some(0x6b),
            Self::TheFunnelMachines => Some(0x68),
            Self::TheSpinningTowersOfPower => Some(0x69),
            Self::TopOfTheSecurityTunnel => Some(0x9a),
            Self::CompleteTheRollingBallRoom => Some(0x45),
            Self::DefeatPrawn => Some(0x39),
            Self::FrostyBungee => Some(0x5d),
            Self::TopOfTheLodge => Some(0x5e),
            Self::DefeatRobotsOnGuppyMound => Some(0x91),
            Self::BeatMrsPuffsTime => Some(0x92),
            Self::DefeatRobotsOnFlounderHill => Some(0xa8),
            Self::BeatBubbleBuddysTime => Some(0xa9),
            Self::DefeatRobotsOnSandMountain => Some(0xcd),
            Self::BeatLarrysTime => Some(0xcc),
            Self::RoboPatrickAhoy => Some(0x28),
            Self::ThroughTheWoods => Some(0x94),
            Self::FindAllTheLostCampers => Some(0x8d),
            Self::TikiRoundup => Some(0x83),
            Self::DownInTheSwamp => Some(0x84),
            Self::ThroughTheKelpCaves => Some(0x5a),
            Self::PowerCrystalCrisis => Some(0x53),
            Self::KelpVineSlide => Some(0x53),
            Self::BeatMermaidMansTime => Some(0x54),
            Self::TopOfTheEntranceAreaFDG => Some(0x70),
            Self::APathThroughTheGoo => Some(0x71),
            Self::GooTankerAhoy => Some(0x6f),
            Self::TopOfTheStackOfShips => Some(0x86),
            Self::ShipwreckBungee => Some(0x87),
            Self::DestroyTheRobotShip => Some(0x5f),
            Self::GetAloftThereMatey => Some(0x60),
            Self::DefeatTheFlyingDutchman => Some(0x35),
            Self::AcrossTheDreamscape => Some(0x5e),
            Self::FollowTheBouncingBall => Some(0x5f),
            Self::SlidingTexasStyle => Some(0xa1),
            Self::SwingersAhoy => Some(0xa3),
            Self::MusicIsInTheEarOfTheBeholder => Some(0x22e),
            Self::KrabbyPattyPlatforms => Some(0x7f),
            Self::SuperBounce => Some(0x6e),
            Self::HereYouGo => Some(0x32),
            Self::KahRahTae => None,
            Self::TheSmallShallRuleOrNot => None,
        }
    }

    /// Returns the level this spatula is in.
    ///
    /// ```
    /// use bfbb::{Level, Spatula};
    ///
    /// let closet = Spatula::SpongebobsCloset;
    /// let level = closet.get_level();
    /// assert_eq!(level, Level::SpongebobHouse);
    /// ```
    pub fn get_level(&self) -> crate::Level {
        #[allow(clippy::match_same_arms)]
        match *self {
            Self::OnTopOfThePineapple => Level::BikiniBottom,
            Self::OnTopOfShadyShoals => Level::BikiniBottom,
            Self::OnTopOfTheChumBucket => Level::BikiniBottom,
            Self::SpongebobsCloset => Level::SpongebobHouse,
            Self::AnnoySquidward => Level::SquidwardHouse,
            Self::AmbushAtTheTreeDome => Level::Treedome,
            Self::InfestationAtTheKrustyKrab => Level::BikiniBottom,
            Self::AWallJumpInTheBucket => Level::ChumBucket,
            Self::TopOfTheHill => Level::JellyfishRock,
            Self::CowaBungee => Level::JellyfishRock,
            Self::Spelunking => Level::JellyfishCaves,
            Self::PatricksDilemma => Level::JellyfishCaves,
            Self::NavigateTheCanyonsAndMesas => Level::JellyfishLake,
            Self::DrainTheLake => Level::JellyfishLake,
            Self::SlideLeap => Level::JellyfishMountain,
            Self::DefeatKingJellyfish => Level::JellyfishRock,
            Self::EndOfTheRoad => Level::DowntownStreets,
            Self::LearnSandysMoves => Level::DowntownStreets,
            Self::TikisGoBoom => Level::DowntownStreets,
            Self::AcrossTheRooftops => Level::DowntownRooftops,
            Self::SwinginSandy => Level::DowntownRooftops,
            Self::AmbushInTheLighthouse => Level::DowntownLighthouse,
            Self::ExtremeBungee => Level::DowntownSeaNeedle,
            Self::ComeBackWithTheCruiseBubble => Level::DowntownSeaNeedle,
            Self::KingOfTheCastle => Level::GooLagoonBeach,
            Self::ConnectTheTowers => Level::GooLagoonBeach,
            Self::SaveTheChildren => Level::GooLagoonBeach,
            Self::OverTheMoat => Level::GooLagoonBeach,
            Self::ThroughTheSeaCaves => Level::GooLagoonCaves,
            Self::CleanOutTheBumperBoats => Level::GooLagoonPier,
            Self::SlipAndSlideUnderThePier => Level::GooLagoonPier,
            Self::TowerBungee => Level::GooLagoonPier,
            Self::RumbleAtThePoseidome => Level::Poseidome,
            Self::GetToTheMuseum => Level::RockBottomDowntown,
            Self::SlipSlidingAway => Level::RockBottomDowntown,
            Self::ReturnTheMuseumsArt => Level::RockBottomDowntown,
            Self::SwingalongSpatula => Level::RockBottomDowntown,
            Self::PlunderingRobotsInTheMuseum => Level::RockBottomMuseum,
            Self::AcrossTheTrenchOfDarkness => Level::RockBottomTrench,
            Self::LasersAreFunAndGoodForYou => Level::RockBottomTrench,
            Self::HowInTarnationDoYouGetThere => Level::RockBottomTrench,
            Self::TopOfTheEntranceAreaML => Level::MermalairEntranceArea,
            Self::TopOfTheComputerArea => Level::MermalairMainChamber,
            Self::ShutDownTheSecuritySystem => Level::MermalairMainChamber,
            Self::TheFunnelMachines => Level::MermalairMainChamber,
            Self::TheSpinningTowersOfPower => Level::MermalairMainChamber,
            Self::TopOfTheSecurityTunnel => Level::MermalairSecurityTunnel,
            Self::CompleteTheRollingBallRoom => Level::MermalairBallroom,
            Self::DefeatPrawn => Level::MermalairVillianContainment,
            Self::FrostyBungee => Level::SandMountainHub,
            Self::TopOfTheLodge => Level::SandMountainHub,
            Self::DefeatRobotsOnGuppyMound => Level::SandMountainSlide1,
            Self::BeatMrsPuffsTime => Level::SandMountainSlide1,
            Self::DefeatRobotsOnFlounderHill => Level::SandMountainSlide2,
            Self::BeatBubbleBuddysTime => Level::SandMountainSlide2,
            Self::DefeatRobotsOnSandMountain => Level::SandMountainSlide3,
            Self::BeatLarrysTime => Level::SandMountainSlide3,
            Self::RoboPatrickAhoy => Level::IndustrialPark,
            Self::ThroughTheWoods => Level::KelpForest,
            Self::FindAllTheLostCampers => Level::KelpForest,
            Self::TikiRoundup => Level::KelpSwamps,
            Self::DownInTheSwamp => Level::KelpSwamps,
            Self::ThroughTheKelpCaves => Level::KelpCaves,
            Self::PowerCrystalCrisis => Level::KelpCaves,
            Self::KelpVineSlide => Level::KelpVines,
            Self::BeatMermaidMansTime => Level::KelpVines,
            Self::TopOfTheEntranceAreaFDG => Level::GraveyardLake,
            Self::APathThroughTheGoo => Level::GraveyardLake,
            Self::GooTankerAhoy => Level::GraveyardLake,
            Self::TopOfTheStackOfShips => Level::GraveyardShipwreck,
            Self::ShipwreckBungee => Level::GraveyardShipwreck,
            Self::DestroyTheRobotShip => Level::GraveyardShip,
            Self::GetAloftThereMatey => Level::GraveyardShip,
            Self::DefeatTheFlyingDutchman => Level::GraveyardBoss,
            Self::AcrossTheDreamscape => Level::SpongebobsDream,
            Self::FollowTheBouncingBall => Level::SpongebobsDream,
            Self::SlidingTexasStyle => Level::SandysDream,
            Self::SwingersAhoy => Level::SandysDream,
            Self::MusicIsInTheEarOfTheBeholder => Level::SquidwardsDream,
            Self::KrabbyPattyPlatforms => Level::KrabsDream,
            Self::SuperBounce => Level::SpongebobsDream,
            Self::HereYouGo => Level::PatricksDream,
            Self::KahRahTae => Level::ChumBucketLab,
            Self::TheSmallShallRuleOrNot => Level::ChumBucketBrain,
        }
    }
}

impl TryFrom<(usize, usize)> for Spatula {
    type Error = &'static str;

    fn try_from(value: (usize, usize)) -> Result<Self, Self::Error> {
        match value {
            (0, 0) => Ok(Self::OnTopOfThePineapple),
            (0, 1) => Ok(Self::OnTopOfShadyShoals),
            (0, 2) => Ok(Self::OnTopOfTheChumBucket),
            (0, 3) => Ok(Self::SpongebobsCloset),
            (0, 4) => Ok(Self::AnnoySquidward),
            (0, 5) => Ok(Self::AmbushAtTheTreeDome),
            (0, 6) => Ok(Self::InfestationAtTheKrustyKrab),
            (0, 7) => Ok(Self::AWallJumpInTheBucket),

            // Jellyfish Fields
            (1, 0) => Ok(Self::TopOfTheHill),
            (1, 1) => Ok(Self::CowaBungee),
            (1, 2) => Ok(Self::Spelunking),
            (1, 3) => Ok(Self::PatricksDilemma),
            (1, 4) => Ok(Self::NavigateTheCanyonsAndMesas),
            (1, 5) => Ok(Self::DrainTheLake),
            (1, 6) => Ok(Self::SlideLeap),
            (1, 7) => Ok(Self::DefeatKingJellyfish),

            // Downtown Bikini Bottom
            (2, 0) => Ok(Self::EndOfTheRoad),
            (2, 1) => Ok(Self::LearnSandysMoves),
            (2, 2) => Ok(Self::TikisGoBoom),
            (2, 3) => Ok(Self::AcrossTheRooftops),
            (2, 4) => Ok(Self::SwinginSandy),
            (2, 5) => Ok(Self::AmbushInTheLighthouse),
            (2, 6) => Ok(Self::ExtremeBungee),
            (2, 7) => Ok(Self::ComeBackWithTheCruiseBubble),

            // Goo Lagoon
            (3, 0) => Ok(Self::KingOfTheCastle),
            (3, 1) => Ok(Self::ConnectTheTowers),
            (3, 2) => Ok(Self::SaveTheChildren),
            (3, 3) => Ok(Self::OverTheMoat),
            (3, 4) => Ok(Self::ThroughTheSeaCaves),
            (3, 5) => Ok(Self::CleanOutTheBumperBoats),
            (3, 6) => Ok(Self::SlipAndSlideUnderThePier),
            (3, 7) => Ok(Self::TowerBungee),

            // Poseidome
            (4, 0) => Ok(Self::RumbleAtThePoseidome),

            // Rock Bottom
            (5, 0) => Ok(Self::GetToTheMuseum),
            (5, 1) => Ok(Self::SlipSlidingAway),
            (5, 2) => Ok(Self::ReturnTheMuseumsArt),
            (5, 3) => Ok(Self::SwingalongSpatula),
            (5, 4) => Ok(Self::PlunderingRobotsInTheMuseum),
            (5, 5) => Ok(Self::AcrossTheTrenchOfDarkness),
            (5, 6) => Ok(Self::LasersAreFunAndGoodForYou),
            (5, 7) => Ok(Self::HowInTarnationDoYouGetThere),

            // Mermalair
            (6, 0) => Ok(Self::TopOfTheEntranceAreaML),
            (6, 1) => Ok(Self::TopOfTheComputerArea),
            (6, 2) => Ok(Self::ShutDownTheSecuritySystem),
            (6, 3) => Ok(Self::TheFunnelMachines),
            (6, 4) => Ok(Self::TheSpinningTowersOfPower),
            (6, 5) => Ok(Self::TopOfTheSecurityTunnel),
            (6, 6) => Ok(Self::CompleteTheRollingBallRoom),
            (6, 7) => Ok(Self::DefeatPrawn),

            // Sand Mountain
            (7, 0) => Ok(Self::FrostyBungee),
            (7, 1) => Ok(Self::TopOfTheLodge),
            (7, 2) => Ok(Self::DefeatRobotsOnGuppyMound),
            (7, 3) => Ok(Self::BeatMrsPuffsTime),
            (7, 4) => Ok(Self::DefeatRobotsOnFlounderHill),
            (7, 5) => Ok(Self::BeatBubbleBuddysTime),
            (7, 6) => Ok(Self::DefeatRobotsOnSandMountain),
            (7, 7) => Ok(Self::BeatLarrysTime),

            // Industrial Park
            (8, 0) => Ok(Self::RoboPatrickAhoy),

            // Kelp Forest
            (9, 0) => Ok(Self::ThroughTheWoods),
            (9, 1) => Ok(Self::FindAllTheLostCampers),
            (9, 2) => Ok(Self::TikiRoundup),
            (9, 3) => Ok(Self::DownInTheSwamp),
            (9, 4) => Ok(Self::ThroughTheKelpCaves),
            (9, 5) => Ok(Self::PowerCrystalCrisis),
            (9, 6) => Ok(Self::KelpVineSlide),
            (9, 7) => Ok(Self::BeatMermaidMansTime),

            // Flying Dutchman's Graveyard
            (10, 0) => Ok(Self::TopOfTheEntranceAreaFDG),
            (10, 1) => Ok(Self::APathThroughTheGoo),
            (10, 2) => Ok(Self::GooTankerAhoy),
            (10, 3) => Ok(Self::TopOfTheStackOfShips),
            (10, 4) => Ok(Self::ShipwreckBungee),
            (10, 5) => Ok(Self::DestroyTheRobotShip),
            (10, 6) => Ok(Self::GetAloftThereMatey),
            (10, 7) => Ok(Self::DefeatTheFlyingDutchman),

            // SpongeBob's Dream
            (11, 0) => Ok(Self::AcrossTheDreamscape),
            (11, 1) => Ok(Self::FollowTheBouncingBall),
            (11, 2) => Ok(Self::SlidingTexasStyle),
            (11, 3) => Ok(Self::SwingersAhoy),
            (11, 4) => Ok(Self::MusicIsInTheEarOfTheBeholder),
            (11, 5) => Ok(Self::KrabbyPattyPlatforms),
            (11, 6) => Ok(Self::SuperBounce),
            (11, 7) => Ok(Self::HereYouGo),

            // Chum Bucket Lab,
            (12, 0) => Ok(Self::KahRahTae),
            (12, 1) => Ok(Self::TheSmallShallRuleOrNot),
            _ => Err("Menu coordinate did not correspond to a Spatula"),
        }
    }
}

impl From<Spatula> for (usize, usize) {
    fn from(spatula: Spatula) -> Self {
        match spatula {
            // Bikini Bottom
            Spatula::OnTopOfThePineapple => (0, 0),
            Spatula::OnTopOfShadyShoals => (0, 1),
            Spatula::OnTopOfTheChumBucket => (0, 2),
            Spatula::SpongebobsCloset => (0, 3),
            Spatula::AnnoySquidward => (0, 4),
            Spatula::AmbushAtTheTreeDome => (0, 5),
            Spatula::InfestationAtTheKrustyKrab => (0, 6),
            Spatula::AWallJumpInTheBucket => (0, 7),

            // Jellyfish Fields
            Spatula::TopOfTheHill => (1, 0),
            Spatula::CowaBungee => (1, 1),
            Spatula::Spelunking => (1, 2),
            Spatula::PatricksDilemma => (1, 3),
            Spatula::NavigateTheCanyonsAndMesas => (1, 4),
            Spatula::DrainTheLake => (1, 5),
            Spatula::SlideLeap => (1, 6),
            Spatula::DefeatKingJellyfish => (1, 7),

            // Downtown Bikini Bottom
            Spatula::EndOfTheRoad => (2, 0),
            Spatula::LearnSandysMoves => (2, 1),
            Spatula::TikisGoBoom => (2, 2),
            Spatula::AcrossTheRooftops => (2, 3),
            Spatula::SwinginSandy => (2, 4),
            Spatula::AmbushInTheLighthouse => (2, 5),
            Spatula::ExtremeBungee => (2, 6),
            Spatula::ComeBackWithTheCruiseBubble => (2, 7),

            // Goo Lagoon
            Spatula::KingOfTheCastle => (3, 0),
            Spatula::ConnectTheTowers => (3, 1),
            Spatula::SaveTheChildren => (3, 2),
            Spatula::OverTheMoat => (3, 3),
            Spatula::ThroughTheSeaCaves => (3, 4),
            Spatula::CleanOutTheBumperBoats => (3, 5),
            Spatula::SlipAndSlideUnderThePier => (3, 6),
            Spatula::TowerBungee => (3, 7),

            // Poseidome
            Spatula::RumbleAtThePoseidome => (4, 0),

            // Rock Bottom
            Spatula::GetToTheMuseum => (5, 0),
            Spatula::SlipSlidingAway => (5, 1),
            Spatula::ReturnTheMuseumsArt => (5, 2),
            Spatula::SwingalongSpatula => (5, 3),
            Spatula::PlunderingRobotsInTheMuseum => (5, 4),
            Spatula::AcrossTheTrenchOfDarkness => (5, 5),
            Spatula::LasersAreFunAndGoodForYou => (5, 6),
            Spatula::HowInTarnationDoYouGetThere => (5, 7),

            // Mermalair
            Spatula::TopOfTheEntranceAreaML => (6, 0),
            Spatula::TopOfTheComputerArea => (6, 1),
            Spatula::ShutDownTheSecuritySystem => (6, 2),
            Spatula::TheFunnelMachines => (6, 3),
            Spatula::TheSpinningTowersOfPower => (6, 4),
            Spatula::TopOfTheSecurityTunnel => (6, 5),
            Spatula::CompleteTheRollingBallRoom => (6, 6),
            Spatula::DefeatPrawn => (6, 7),

            // Sand Mountain
            Spatula::FrostyBungee => (7, 0),
            Spatula::TopOfTheLodge => (7, 1),
            Spatula::DefeatRobotsOnGuppyMound => (7, 2),
            Spatula::BeatMrsPuffsTime => (7, 3),
            Spatula::DefeatRobotsOnFlounderHill => (7, 4),
            Spatula::BeatBubbleBuddysTime => (7, 5),
            Spatula::DefeatRobotsOnSandMountain => (7, 6),
            Spatula::BeatLarrysTime => (7, 7),

            // Industrial Park
            Spatula::RoboPatrickAhoy => (8, 0),

            // Kelp Forest
            Spatula::ThroughTheWoods => (9, 0),
            Spatula::FindAllTheLostCampers => (9, 1),
            Spatula::TikiRoundup => (9, 2),
            Spatula::DownInTheSwamp => (9, 3),
            Spatula::ThroughTheKelpCaves => (9, 4),
            Spatula::PowerCrystalCrisis => (9, 5),
            Spatula::KelpVineSlide => (9, 6),
            Spatula::BeatMermaidMansTime => (9, 7),

            // Flying Dutchman's Graveyard
            Spatula::TopOfTheEntranceAreaFDG => (10, 0),
            Spatula::APathThroughTheGoo => (10, 1),
            Spatula::GooTankerAhoy => (10, 2),
            Spatula::TopOfTheStackOfShips => (10, 3),
            Spatula::ShipwreckBungee => (10, 4),
            Spatula::DestroyTheRobotShip => (10, 5),
            Spatula::GetAloftThereMatey => (10, 6),
            Spatula::DefeatTheFlyingDutchman => (10, 7),

            // Spongebob's Dream
            Spatula::AcrossTheDreamscape => (11, 0),
            Spatula::FollowTheBouncingBall => (11, 1),
            Spatula::SlidingTexasStyle => (11, 2),
            Spatula::SwingersAhoy => (11, 3),
            Spatula::MusicIsInTheEarOfTheBeholder => (11, 4),
            Spatula::KrabbyPattyPlatforms => (11, 5),
            Spatula::SuperBounce => (11, 6),
            Spatula::HereYouGo => (11, 7),

            // Chum Bucket Lab
            Spatula::KahRahTae => (12, 0),
            Spatula::TheSmallShallRuleOrNot => (12, 1),
        }
    }
}
