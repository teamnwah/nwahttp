use lazy_static::lazy_static;
use prometheus::*;

lazy_static! {
    pub static ref SKILL_LEVEL: IntGaugeVec = register_int_gauge_vec!(
        "openmw_player_skill_level",
        "The skill levels of players",
        &["player", "skill", "skill_id"]
    )
    .unwrap();
    pub static ref SKILL_PROGRESS: GaugeVec = register_gauge_vec!(
        "openmw_player_skill_progress",
        "The skill progress of players",
        &["player", "skill", "skill_id"]
    )
    .unwrap();
    pub static ref ATTRIBUTE_LEVEL: IntGaugeVec = register_int_gauge_vec!(
        "openmw_player_attr_level",
        "The attribute levels of players",
        &["player", "attribute", "attribute_id"]
    )
    .unwrap();
    pub static ref LEVEL: IntGaugeVec =
        register_int_gauge_vec!("openmw_player_level", "The level of players", &["player"])
            .unwrap();
    pub static ref LEVEL_PROGRESS: IntGaugeVec = register_int_gauge_vec!(
        "openmw_player_level_progress",
        "The level progress of players",
        &["player"]
    )
    .unwrap();
    pub static ref MAGICKA_BASE: GaugeVec = register_gauge_vec!(
        "openmw_player_magicka_base",
        "The base magicka of players",
        &["player"]
    )
    .unwrap();
    pub static ref MAGICKA: GaugeVec = register_gauge_vec!(
        "openmw_player_magicka",
        "The current magicka of players",
        &["player"]
    )
    .unwrap();
    pub static ref HEALTH_BASE: GaugeVec = register_gauge_vec!(
        "openmw_player_health_base",
        "The base health of players",
        &["player"]
    )
    .unwrap();
    pub static ref HEALTH: GaugeVec = register_gauge_vec!(
        "openmw_player_health",
        "The current health of players",
        &["player"]
    )
    .unwrap();
    pub static ref FATIGUE_BASE: GaugeVec = register_gauge_vec!(
        "openmw_player_fatigue_base",
        "The base fatigue of players",
        &["player"]
    )
    .unwrap();
    pub static ref FATIGUE: GaugeVec = register_gauge_vec!(
        "openmw_player_fatigue",
        "The current fatigue of players",
        &["player"]
    )
    .unwrap();
    pub static ref DISTANCE_TRAVELED: HistogramVec = register_histogram_vec!(
        "openmw_player_distance_traveled",
        "The amount of distance a player has travelled",
        &["player"]
    )
    .unwrap();
}
