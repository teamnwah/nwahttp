use crate::plugin::*;
use crate::server_info::counters::*;
use crate::server_info::events::PlayerPosition;
use crate::server_info::Specialization::{Combat, Magic, Stealth};
use serde::Serialize;
use std::collections::{HashMap, HashSet};
use std::os::raw::{c_double, c_int, c_ushort};

#[derive(Serialize, Debug, Copy, Clone, Default, PartialOrd, PartialEq)]
pub struct Vec3 {
    pub x: c_double,
    pub y: c_double,
    pub z: c_double,
}

impl Vec3 {
    fn distance(&self, rhs: Vec3) -> f64 {
        let x = self.x - rhs.x;
        let y = self.y - rhs.y;
        let z = self.z - rhs.z;

        ((x * x) + (y * y) + (z * z)).sqrt()
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
#[allow(dead_code)]
pub enum Attribute {
    Strength = 0,
    Intelligence = 1,
    Willpower = 2,
    Agility = 3,
    Speed = 4,
    Endurance = 5,
    Personality = 6,
    Luck = 7,
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct AttributeValue {
    pub id: c_ushort,
    pub name: String,
    pub damage: c_double,
    pub modifier: c_int,
    pub base: c_int,
}

impl AttributeValue {
    fn get(player_id: c_ushort, attribute_id: c_ushort) -> AttributeValue {
        AttributeValue {
            id: attribute_id,
            name: get_attribute_name(attribute_id),
            damage: get_attribute_damage(player_id, attribute_id),
            modifier: get_attribute_modifier(player_id, attribute_id),
            base: get_attribute_base(player_id, attribute_id),
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u16)]
#[allow(dead_code)]
pub enum Skill {
    Block = 0,
    Armorer = 1,
    MediumArmor = 2,
    HeavyArmor = 3,
    Blunt = 4,
    Longblade = 5,
    Axe = 6,
    Spear = 7,
    Athletics = 8,
    Enchant = 9,
    Destruction = 10,
    Alteration = 11,
    Illusion = 12,
    Conjuration = 13,
    Mysticism = 14,
    Restoration = 15,
    Alchemy = 16,
    Unarmored = 17,
    Security = 18,
    Sneak = 19,
    Acrobatics = 20,
    LightArmor = 21,
    Shortblade = 22,
    Marksman = 23,
    Mercantile = 24,
    Speechcraft = 25,
    HandToHand = 26,
}

#[derive(Serialize, Debug, Clone, Copy, Ord, PartialOrd, Eq, PartialEq)]
pub enum SkillType {
    Major,
    Minor,
    Misc,
}

impl Default for SkillType {
    fn default() -> Self {
        SkillType::Misc
    }
}

#[derive(Serialize, Clone, Copy, Debug, Ord, PartialOrd, Eq, PartialEq)]
#[repr(i32)]
pub enum Specialization {
    Combat = 0,
    Magic = 1,
    Stealth = 2,
    None = 3,
}

impl Specialization {
    fn get_for_skill(skill_id: u16) -> Specialization {
        if skill_id < 9 {
            Combat
        } else if skill_id < 18 {
            Magic
        } else if skill_id < 27 {
            Stealth
        } else {
            Specialization::None
        }
    }

    fn get(id: c_int) -> Specialization {
        match id {
            0 => Specialization::Combat,
            1 => Specialization::Magic,
            2 => Specialization::Stealth,
            _ => Specialization::None,
        }
    }
}

impl Default for Specialization {
    fn default() -> Self {
        Specialization::None
    }
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct SkillValue {
    pub id: c_ushort,
    pub name: String,
    pub progress: c_double,
    pub base: c_int,
    pub increase: c_int,
    pub modifier: c_int,
    pub damage: c_double,
    pub progress_requirement: c_double,
    pub progress_percent: c_double,
    pub skill_type: SkillType,
}

impl SkillValue {
    fn get(player_id: c_ushort, skill_id: c_ushort) -> Self {
        SkillValue {
            id: skill_id,
            name: get_skill_name(skill_id),
            progress: get_skill_progress(player_id, skill_id),
            base: get_skill_base(player_id, skill_id),
            increase: get_skill_increase(player_id, skill_id.into()),
            modifier: get_skill_modifier(player_id, skill_id),
            damage: get_skill_damage(player_id, skill_id),
            progress_requirement: 0f64,
            progress_percent: 0f64,
            skill_type: SkillType::Minor,
        }
    }

    fn calculate_progress(&mut self, is_specialization: bool, skill_type: SkillType) {
        let mut requirement = (1 + self.base) as f64;
        self.skill_type = skill_type;

        requirement *= match skill_type {
            SkillType::Major => 0.75,
            SkillType::Minor => 1.0,
            SkillType::Misc => 1.25,
        };

        if is_specialization {
            requirement *= 0.8;
        }

        self.progress_requirement = requirement;
        self.progress_percent = self.progress / requirement;
    }
}

impl Into<(c_double, c_double, c_double)> for Vec3 {
    fn into(self) -> (c_double, c_double, c_double) {
        (self.x, self.y, self.z)
    }
}

impl From<(c_double, c_double, c_double)> for Vec3 {
    fn from(x: (c_double, c_double, c_double)) -> Self {
        Vec3::new(x.0, x.1, x.2)
    }
}

impl Vec3 {
    pub fn new(x: c_double, y: c_double, z: c_double) -> Self {
        Self { x, y, z }
    }

    pub fn get_position(player_id: c_ushort) -> Self {
        Self::new(
            get_pos_x(player_id),
            get_pos_y(player_id),
            get_pos_z(player_id),
        )
    }

    pub fn get_rotation(player_id: c_ushort) -> Self {
        Self::new(get_rot_x(player_id), 0.into(), get_rot_z(player_id))
    }
}

#[derive(Serialize, Clone, Debug, Default)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub id: c_ushort,
    pub name: String,
    pub head: String,
    pub hair: String,
    pub logged_in: bool,
    pub distance_travelled: f64,
    pub race: String,
    pub class: PlayerClass,
    pub cell: String,
    pub is_outside: bool,
    pub position: Vec3,
    pub rotation: Vec3,
    pub health: c_double,
    pub health_base: c_double,
    pub fatigue: c_double,
    pub fatigue_base: c_double,
    pub magicka: c_double,
    pub magicka_base: c_double,
    pub level: c_int,
    pub level_progress: c_int,
    pub attributes: Vec<AttributeValue>,
    pub skills: Vec<SkillValue>,
    pub major_skills: HashSet<c_ushort>,
    pub minor_skills: HashSet<c_ushort>,
    pub specialisation: Specialization,
}

#[derive(Serialize, Clone, Debug)]
#[serde(tag = "type")]
#[serde(rename_all = "camelCase")]
pub enum PlayerClass {
    Custom { name: String, description: String },
    Default { name: String },
    None,
}

impl Default for PlayerClass {
    fn default() -> Self {
        PlayerClass::None
    }
}

impl Player {
    pub fn new(id: c_ushort) -> Self {
        let mut player = Player::default();
        player.id = id;
        player.update();
        player.low_frequency_update();

        player
    }

    pub fn get_player_position(&self) -> PlayerPosition {
        PlayerPosition {
            name: self.name.clone(),
            position: (self.position.x, self.position.y),
            rotation: self.rotation.z,
            cell: self.cell.clone(),
            is_outside: self.is_outside,
        }
    }

    pub fn get_skill_type(&self, skill_id: c_ushort) -> SkillType {
        if self.major_skills.contains(&skill_id) {
            SkillType::Major
        } else if self.minor_skills.contains(&skill_id) {
            SkillType::Minor
        } else {
            SkillType::Misc
        }
    }

    pub fn update(&mut self) {
        self.rotation = Vec3::get_rotation(self.id);
        self.is_outside = is_in_exterior(self.id);

        let cell = get_cell(self.id);
        let position = Vec3::get_position(self.id);

        if cell == self.cell {
            self.distance_travelled += self.position.distance(position)
        }

        self.position = position;

        self.cell = cell;
        self.health_base = get_health_base(self.id);
        self.health = get_health_current(self.id);
        self.fatigue_base = get_fatigue_base(self.id);
        self.fatigue = get_fatigue_current(self.id);
        self.magicka_base = get_magicka_base(self.id);
        self.magicka = get_magicka_current(self.id);
        self.level = get_level(self.id);
        self.level_progress = get_level_progress(self.id)
    }

    pub fn update_once(&mut self) {
        self.name = get_name(self.id);
        self.race = get_race(self.id);
        self.head = get_head(self.id);
        self.hair = get_hair(self.id);

        self.major_skills = HashSet::new();
        self.major_skills
            .insert(get_class_major_attribute(self.id, 0) as c_ushort);
        self.major_skills
            .insert(get_class_major_attribute(self.id, 1) as c_ushort);

        self.minor_skills = HashSet::new();
        self.minor_skills
            .insert(get_class_minor_skill(self.id, 0) as c_ushort);
        self.minor_skills
            .insert(get_class_minor_skill(self.id, 1) as c_ushort);
        self.minor_skills
            .insert(get_class_minor_skill(self.id, 2) as c_ushort);
        self.minor_skills
            .insert(get_class_minor_skill(self.id, 3) as c_ushort);
        self.minor_skills
            .insert(get_class_minor_skill(self.id, 4) as c_ushort);

        self.specialisation = Specialization::get(get_class_specialization(self.id));

        let default_class = get_default_class(self.id);

        if default_class.len() == 0 {
            self.class = PlayerClass::Custom {
                name: get_class_name(self.id),
                description: get_class_desc(self.id),
            }
        } else {
            self.class = PlayerClass::Default {
                name: default_class,
            }
        }
    }

    pub fn low_frequency_update(&mut self) {
        self.attributes = (0..get_attribute_count() as c_ushort)
            .map(|id| AttributeValue::get(self.id, id))
            .collect();
        self.skills = (0..get_skill_count() as c_ushort)
            .map(|id| {
                let mut skill = SkillValue::get(self.id, id);
                skill.calculate_progress(
                    self.specialisation == Specialization::get_for_skill(id),
                    self.get_skill_type(id),
                );

                skill
            })
            .collect();

        if self.logged_in {
            self.report_stats();
        }
    }

    pub fn report_stats(&mut self) {
        for skill in &self.skills {
            let mut map = HashMap::new();
            map.insert("player", self.name.as_str());
            map.insert("skill", &skill.name.as_str());
            let id = skill.id.to_string();
            map.insert("skill_id", &id);
            SKILL_LEVEL.with(&map).set(skill.base as i64);
            SKILL_PROGRESS.with(&map).set(skill.progress_percent);
        }

        for attribute in &self.attributes {
            let mut map = HashMap::new();
            map.insert("player", self.name.as_str());
            map.insert("attribute", &attribute.name.as_str());
            let id = attribute.id.to_string();
            map.insert("attribute_id", &id);
            ATTRIBUTE_LEVEL.with(&map).set(attribute.base as i64);
        }
        let mut map = HashMap::new();
        map.insert("player", self.name.as_str());

        LEVEL.with(&map).set(self.level as i64);
        LEVEL_PROGRESS.with(&map).set(self.level_progress as i64);

        MAGICKA_BASE.with(&map).set(self.magicka_base);
        MAGICKA.with(&map).set(self.magicka);

        HEALTH_BASE.with(&map).set(self.health_base);
        HEALTH.with(&map).set(self.health);

        FATIGUE_BASE.with(&map).set(self.fatigue_base);
        FATIGUE.with(&map).set(self.fatigue);

        DISTANCE_TRAVELED
            .with(&map)
            .observe(self.distance_travelled);
        self.distance_travelled = 0.0;
    }

    pub fn on_login(&mut self) {
        send_message(self.id, "#ff0000This server runs #0000ffnwahttp#ff0000 and this is it's obnoxious login message for #00ff00you#ff0000!!\n", false, false);
        self.update_once();
        self.low_frequency_update();
    }
}
