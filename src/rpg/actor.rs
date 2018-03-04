
use rpg::*;

#[derive(Debug)]
pub struct Actor {
    id: i32,
    name: String,
    title: String,
    character_name: String,
    character_index: i32,
    transparent: bool,
    initial_level: u8,
    final_level: Option<u8>,
    critial_hit: bool,
    critical_hit_chance: i32,
    face_name: String,
    face_index: i32,
    two_weapons: bool,
    lock_equipment: bool,
    auto_battle: bool,
    super_guard: bool,
    parameters: Parameters,
    exp_base: Option<u32>,
    exp_inflation: Option<u32>,
    exp_correction: u32,
    initial_equipment: Equipment,
    unarmed_animation: u32,
    class_id: u32,
    battle_x: u32,
    battle_y: u32,
    battler_animation: u32,
    skills: Vec<Learning>,
    rename_skill: bool,
    skill_name: String,
    state_ranks: Vec<u8>,
    attribute_ranks: Vec<u8>,
    battle_commands: i32,
}

impl Lcf for Actor {
    type Output = Actor;

    fn read_lcf(reader: &mut impl ReadPascalString) -> Result<Self::Output, Error> {
        unimplemented!()
    }
}