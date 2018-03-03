#[derive(Debug)]
struct Actors;

#[derive(Debug)]
struct Skill;

#[derive(Debug)]
struct Item;

#[derive(Debug)]
struct Enemy;

#[derive(Debug)]
struct Troop;

#[derive(Debug)]
struct Terrain;

#[derive(Debug)]
struct Attribute;

#[derive(Debug)]
struct State;

#[derive(Debug)]
struct Animation;

#[derive(Debug)]
struct Chipset;

#[derive(Debug)]
struct Terms;

#[derive(Debug)]
struct System;

#[derive(Debug)]
struct Switch;

#[derive(Debug)]
struct Variable;

#[derive(Debug)]
struct CommonEvent;

#[derive(Debug)]
struct BattleCommands;

#[derive(Debug)]
struct Class;

#[derive(Debug)]
struct BattleAnimation;

use std::io::{Read, Write};
use failure::Error;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use traits::{Deserialize, ReadPascalString};


#[derive(Debug)]
pub struct Database {
    actors: Vec<Actors>,
    skills: Vec<Skill>,
    items: Vec<Item>,
    enemies: Vec<Enemy>,
    troops: Vec<Troop>,
    terrains: Vec<Terrain>,
    attributes: Vec<Attribute>,
    states: Vec<State>,
    animations: Vec<Animation>,
    chipsets: Vec<Chipset>,
    terms: Terms,
    system: System,
    switches: Vec<Switch>,
    variables: Vec<Variable>,
    commonevents: Vec<CommonEvent>,
    version: i32,
    battlecommands: BattleCommands,
    classes: Vec<Class>,
    battleranimations: Vec<BattleAnimation>,
}


impl Database {
    pub fn new_from_existing_database(reader: &mut (impl ReadBytesExt + ReadPascalString)) -> Result<Database, Error> {
        ensure!(reader.read_string()? == "LcfDataBase", "Was wrong");

        let id = reader.read_i32::<LittleEndian>()?;
        let length = reader.read_i32::<LittleEndian>()?;
        //TODO early out when id == 0?!

        if id == 0 || length == 0 {
            bail!("Aehm!"); //TODO: aehm.. wat
        }



        unimplemented!()
    }
}

//Database,skills,f,Array<Skill>,0x0C,,RPG::Skill
//Database,items,f,Array<Item>,0x0D,,RPG::Item
//Database,enemies,f,Array<Enemy>,0x0E,,RPG::Enemy
//Database,troops,f,Array<Troop>,0x0F,,RPG::Troop
//Database,terrains,f,Array<Terrain>,0x10,,RPG::Terrain
//Database,attributes,f,Array<Attribute>,0x11,,RPG::Attribute
//Database,states,f,Array<State>,0x12,,RPG::State
//Database,animations,f,Array<Animation>,0x13,,RPG::Animation
//Database,chipsets,f,Array<Chipset>,0x14,,RPG::Chipset
//Database,terms,f,Terms,0x15,,RPG::Terms
//Database,system,f,System,0x16,,RPG::System
//Database,switches,f,Array<Switch>,0x17,,RPG::Switchs
//Database,variables,f,Array<Variable>,0x18,,RPG::Variables
//Database,commonevents,f,Array<CommonEvent>,0x19,,RPG::CommonEvent
//Database,version,f,Int32,0x1A,0,Indicates version of database. When 1 the database was converted to RPG Maker 2000 v1.61
//Database,commoneventD2,f,,0x1B,,Duplicated? - Not used - RPG2003
//Database,commoneventD3,f,,0x1C,,Duplicated? - Not used - RPG2003
//Database,battlecommands,f,BattleCommands,0x1D,,RPG::BattleCommand - RPG2003
//Database,classes,f,Array<Class>,0x1E,,RPG::Class - RPG2003
//Database,classD1,f,,0x1F,,Duplicated? - Not used - RPG2003
//Database,battleranimations,f,Array<BattlerAnimation>,0x20,,RPG::BattlerAnimation - RPG2003

trait Field {
    type Struct;
    const INDEX: u8;
    const NAME: &'static str;

    fn read(&self, obj: &Self::Struct) -> Result<Self, Error> where Self: Sized;
}

struct Put;

impl Field for Put {
    type Struct = Database;
    const INDEX: u8 = 0x0B;
    const NAME: &'static str = "actors";

    fn read(&self, obj: &Self::Struct) -> Result<Self, Error> where Self: Sized {
        unimplemented!()
    }
}

const FIELDS: &[Put] = &[Put];

impl Deserialize for Database {
    type Out = Self;

    fn deserialize(reader: &mut impl ReadPascalString) -> Result<Self::Out, Error> {
        unimplemented!()
    }
}