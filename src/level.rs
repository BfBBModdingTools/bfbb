/// Convenient definition for every level in the game.
///
/// # Scene IDs
/// Battle for Bikini Bottom uses bytestrings encoded into a u32 to identify scenes.
/// Internally this was implemented with C/C++ multi-character constants (`u32 scene_id = 'HB01'`).
/// `Level` implements conversion to and from this format using a byte array (`[u8; 4]`).
///
/// # Examples
/// Using scene ids:
/// ```
/// use bfbb::Level;
///
/// // Attempting to get a Level from a scene_id
///
/// let scene_id = b"HB01";
/// let level = Level::try_from(*scene_id).expect("'HB01' should be the main hub level");
/// assert_eq!(level, Level::BikiniBottom);
///
/// // Converting a Level to it's scene_id
///
/// let hub = Level::BikiniBottom;
/// let scene_id: [u8; 4] = hub.into();
/// assert_eq!(b"HB01", &scene_id)
/// ```
///
/// Converting a Level to it's in-game name:
/// ```
/// use bfbb::Level;
///
/// let level = Level::SpongebobHouse;
/// println!("{level}"); // Prints "Spongebob's House"
/// assert_eq!(level.to_string().as_str(), "Spongebob's House");
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Level {
    #[doc(hidden)]
    MainMenu,
    #[doc(hidden)]
    IntroCutscene,

    #[doc(hidden)]
    BikiniBottom,
    #[doc(hidden)]
    SpongebobHouse,
    #[doc(hidden)]
    SquidwardHouse,
    #[doc(hidden)]
    PatrickHouse,
    #[doc(hidden)]
    ShadyShoals,
    #[doc(hidden)]
    PoliceStation,
    #[doc(hidden)]
    Treedome,
    #[doc(hidden)]
    KrustyKrab,
    #[doc(hidden)]
    ChumBucket,
    #[doc(hidden)]
    Theater,

    #[doc(hidden)]
    Poseidome,
    #[doc(hidden)]
    IndustrialPark,

    #[doc(hidden)]
    JellyfishRock,
    #[doc(hidden)]
    JellyfishCaves,
    #[doc(hidden)]
    JellyfishLake,
    #[doc(hidden)]
    JellyfishMountain,

    #[doc(hidden)]
    DowntownStreets,
    #[doc(hidden)]
    DowntownRooftops,
    #[doc(hidden)]
    DowntownLighthouse,
    #[doc(hidden)]
    DowntownSeaNeedle,

    #[doc(hidden)]
    GooLagoonBeach,
    #[doc(hidden)]
    GooLagoonCaves,
    #[doc(hidden)]
    GooLagoonPier,

    #[doc(hidden)]
    MermalairEntranceArea,
    #[doc(hidden)]
    MermalairMainChamber,
    #[doc(hidden)]
    MermalairSecurityTunnel,
    #[doc(hidden)]
    MermalairBallroom,
    #[doc(hidden)]
    MermalairVillianContainment,

    #[doc(hidden)]
    RockBottomDowntown,
    #[doc(hidden)]
    RockBottomMuseum,
    #[doc(hidden)]
    RockBottomTrench,

    #[doc(hidden)]
    SandMountainHub,
    #[doc(hidden)]
    SandMountainSlide1,
    #[doc(hidden)]
    SandMountainSlide2,
    #[doc(hidden)]
    SandMountainSlide3,

    #[doc(hidden)]
    KelpForest,
    #[doc(hidden)]
    KelpSwamps,
    #[doc(hidden)]
    KelpCaves,
    #[doc(hidden)]
    KelpVines,

    #[doc(hidden)]
    GraveyardLake,
    #[doc(hidden)]
    GraveyardShipwreck,
    #[doc(hidden)]
    GraveyardShip,
    #[doc(hidden)]
    GraveyardBoss,

    #[doc(hidden)]
    SpongebobsDream,
    #[doc(hidden)]
    SandysDream,
    #[doc(hidden)]
    SquidwardsDream,
    #[doc(hidden)]
    KrabsDream,
    #[doc(hidden)]
    PatricksDream,

    #[doc(hidden)]
    ChumBucketLab,
    #[doc(hidden)]
    ChumBucketBrain,

    #[doc(hidden)]
    SpongeballArena,
}

impl std::fmt::Display for Level {
    fn fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match *self {
            Level::MainMenu => write!(fmt, "Main Menu"),
            Level::IntroCutscene => write!(fmt, "Intro Cutscene"),
            Level::BikiniBottom => write!(fmt, "Bikini Bottom"),
            Level::SpongebobHouse => write!(fmt, "Spongebob's House"),
            Level::SquidwardHouse => write!(fmt, "Squidward's House"),
            Level::PatrickHouse => write!(fmt, "Patrick's House"),
            Level::ShadyShoals => write!(fmt, "Shady Shoals"),
            Level::PoliceStation => write!(fmt, "Police Station"),
            Level::Treedome => write!(fmt, "Treedome"),
            Level::KrustyKrab => write!(fmt, "Krusty Krab"),
            Level::ChumBucket => write!(fmt, "Chum Bucket"),
            Level::Theater => write!(fmt, "Theater"),
            Level::Poseidome => write!(fmt, "Poseidome"),
            Level::IndustrialPark => write!(fmt, "Industrial Park"),
            Level::JellyfishRock => write!(fmt, "Jellyfish Rock"),
            Level::JellyfishCaves => write!(fmt, "Jellyfish Caves"),
            Level::JellyfishLake => write!(fmt, "Jellyfish Lake"),
            Level::JellyfishMountain => write!(fmt, "Jellyfish Mountain"),
            Level::DowntownStreets => write!(fmt, "Downtown Streets"),
            Level::DowntownRooftops => write!(fmt, "Downtown Rooftops"),
            Level::DowntownLighthouse => write!(fmt, "Downtown Lighthouse"),
            Level::DowntownSeaNeedle => write!(fmt, "Downtown Sea Needle"),
            Level::GooLagoonBeach => write!(fmt, "Goo Lagoon Beach"),
            Level::GooLagoonCaves => write!(fmt, "Goo Lagoon Caves"),
            Level::GooLagoonPier => write!(fmt, "Goo Lagoon Pier"),
            Level::MermalairEntranceArea => write!(fmt, "Mermalair Entrance Area"),
            Level::MermalairMainChamber => write!(fmt, "Mermalair Main Chamber"),
            Level::MermalairSecurityTunnel => write!(fmt, "Mermalair Security Tunnel"),
            Level::MermalairBallroom => write!(fmt, "Mermalair Ballroom"),
            Level::MermalairVillianContainment => write!(fmt, "Mermalair Villian Containment"),
            Level::RockBottomDowntown => write!(fmt, "Rock Bottom Downtown"),
            Level::RockBottomMuseum => write!(fmt, "Rock Bottom Museum"),
            Level::RockBottomTrench => write!(fmt, "Rock Bottom Trench"),
            Level::SandMountainHub => write!(fmt, "Ski Lodge"),
            Level::SandMountainSlide1 => write!(fmt, "Guppy Mound"),
            Level::SandMountainSlide2 => write!(fmt, "Flounder Hill"),
            Level::SandMountainSlide3 => write!(fmt, "Sand Mountain"),
            Level::KelpForest => write!(fmt, "Kelp Forest"),
            Level::KelpSwamps => write!(fmt, "Kelp Swamps"),
            Level::KelpCaves => write!(fmt, "Kelp Caves"),
            Level::KelpVines => write!(fmt, "Kelp Vines"),
            Level::GraveyardLake => write!(fmt, "Graveyard Lake"),
            Level::GraveyardShipwreck => write!(fmt, "Graveyard of Ships"),
            Level::GraveyardShip => write!(fmt, "Dutchman's Ship"),
            Level::GraveyardBoss => write!(fmt, "Flying Dutchman Battle"),
            Level::SpongebobsDream => write!(fmt, "Spongebob's Dream"),
            Level::SandysDream => write!(fmt, "Sandy's Dream"),
            Level::SquidwardsDream => write!(fmt, "Squidward's Dream"),
            Level::KrabsDream => write!(fmt, "Krab's Dream"),
            Level::PatricksDream => write!(fmt, "Patrick's Dream"),
            Level::ChumBucketLab => write!(fmt, "Chum Bucket Lab"),
            Level::ChumBucketBrain => write!(fmt, "Chum Bucket Brain"),
            Level::SpongeballArena => write!(fmt, "Spongeball Arena"),
        }
    }
}

impl TryFrom<[u8; 4]> for Level {
    type Error = &'static str;

    fn try_from(scene_id: [u8; 4]) -> Result<Self, Self::Error> {
        match &scene_id {
            b"MNU3" => Ok(Level::MainMenu),
            b"HB00" => Ok(Level::IntroCutscene),
            b"HB01" => Ok(Level::BikiniBottom),
            b"HB02" => Ok(Level::SpongebobHouse),
            b"HB03" => Ok(Level::SquidwardHouse),
            b"HB04" => Ok(Level::PatrickHouse),
            b"HB06" => Ok(Level::ShadyShoals),
            b"HB09" => Ok(Level::PoliceStation),
            b"HB05" => Ok(Level::Treedome),
            b"HB07" => Ok(Level::KrustyKrab),
            b"HB08" => Ok(Level::ChumBucket),
            b"HB10" => Ok(Level::Theater),
            b"B101" => Ok(Level::Poseidome),
            b"B201" => Ok(Level::IndustrialPark),
            b"JF01" => Ok(Level::JellyfishRock),
            b"JF02" => Ok(Level::JellyfishCaves),
            b"JF03" => Ok(Level::JellyfishLake),
            b"JF04" => Ok(Level::JellyfishMountain),
            b"BB01" => Ok(Level::DowntownStreets),
            b"BB02" => Ok(Level::DowntownRooftops),
            b"BB03" => Ok(Level::DowntownLighthouse),
            b"BB04" => Ok(Level::DowntownSeaNeedle),
            b"GL01" => Ok(Level::GooLagoonBeach),
            b"GL02" => Ok(Level::GooLagoonCaves),
            b"GL03" => Ok(Level::GooLagoonPier),
            b"BC01" => Ok(Level::MermalairEntranceArea),
            b"BC02" => Ok(Level::MermalairMainChamber),
            b"BC03" => Ok(Level::MermalairSecurityTunnel),
            b"BC04" => Ok(Level::MermalairBallroom),
            b"BC05" => Ok(Level::MermalairVillianContainment),
            b"RB01" => Ok(Level::RockBottomDowntown),
            b"RB02" => Ok(Level::RockBottomMuseum),
            b"RB03" => Ok(Level::RockBottomTrench),
            b"SM01" => Ok(Level::SandMountainHub),
            b"SM02" => Ok(Level::SandMountainSlide1),
            b"SM03" => Ok(Level::SandMountainSlide2),
            b"SM04" => Ok(Level::SandMountainSlide3),
            b"KF01" => Ok(Level::KelpForest),
            b"KF02" => Ok(Level::KelpSwamps),
            b"KF04" => Ok(Level::KelpCaves),
            b"KF05" => Ok(Level::KelpVines),
            b"GY01" => Ok(Level::GraveyardLake),
            b"GY02" => Ok(Level::GraveyardShipwreck),
            b"GY03" => Ok(Level::GraveyardShip),
            b"GY04" => Ok(Level::GraveyardBoss),
            b"DB01" => Ok(Level::SpongebobsDream),
            b"DB02" => Ok(Level::SandysDream),
            b"DB03" => Ok(Level::SquidwardsDream),
            b"DB04" => Ok(Level::KrabsDream),
            b"DB06" => Ok(Level::PatricksDream),
            b"B302" => Ok(Level::ChumBucketLab),
            b"B303" => Ok(Level::ChumBucketBrain),
            b"PG12" => Ok(Level::SpongeballArena),
            _ => Err("Byte array did not correspond to a level."),
        }
    }
}

impl From<Level> for [u8; 4] {
    fn from(level: Level) -> [u8; 4] {
        *match level {
            Level::MainMenu => b"MNU3",
            Level::IntroCutscene => b"HB00",
            Level::BikiniBottom => b"HB01",
            Level::SpongebobHouse => b"HB02",
            Level::SquidwardHouse => b"HB03",
            Level::PatrickHouse => b"HB04",
            Level::ShadyShoals => b"HB06",
            Level::PoliceStation => b"HB09",
            Level::Treedome => b"HB05",
            Level::KrustyKrab => b"HB07",
            Level::ChumBucket => b"HB08",
            Level::Theater => b"HB10",
            Level::Poseidome => b"B101",
            Level::IndustrialPark => b"B201",
            Level::JellyfishRock => b"JF01",
            Level::JellyfishCaves => b"JF02",
            Level::JellyfishLake => b"JF03",
            Level::JellyfishMountain => b"JF04",
            Level::DowntownStreets => b"BB01",
            Level::DowntownRooftops => b"BB02",
            Level::DowntownLighthouse => b"BB03",
            Level::DowntownSeaNeedle => b"BB04",
            Level::GooLagoonBeach => b"GL01",
            Level::GooLagoonCaves => b"GL02",
            Level::GooLagoonPier => b"GL03",
            Level::MermalairEntranceArea => b"BC01",
            Level::MermalairMainChamber => b"BC02",
            Level::MermalairSecurityTunnel => b"BC03",
            Level::MermalairBallroom => b"BC04",
            Level::MermalairVillianContainment => b"BC05",
            Level::RockBottomDowntown => b"RB01",
            Level::RockBottomMuseum => b"RB02",
            Level::RockBottomTrench => b"RB03",
            Level::SandMountainHub => b"SM01",
            Level::SandMountainSlide1 => b"SM02",
            Level::SandMountainSlide2 => b"SM03",
            Level::SandMountainSlide3 => b"SM04",
            Level::KelpForest => b"KF01",
            Level::KelpSwamps => b"KF02",
            Level::KelpCaves => b"KF04",
            Level::KelpVines => b"KF05",
            Level::GraveyardLake => b"GY01",
            Level::GraveyardShipwreck => b"GY02",
            Level::GraveyardShip => b"GY03",
            Level::GraveyardBoss => b"GY04",
            Level::SpongebobsDream => b"DB01",
            Level::SandysDream => b"DB02",
            Level::SquidwardsDream => b"DB03",
            Level::KrabsDream => b"DB04",
            Level::PatricksDream => b"DB06",
            Level::ChumBucketLab => b"B302",
            Level::ChumBucketBrain => b"B303",
            Level::SpongeballArena => b"PG12",
        }
    }
}
