use byteorder::ReadBytesExt;
use traits::ReadDelphi;
use failure::Error;

mod database;
mod actor;



pub use self::actor::Actor;

#[derive(Debug)]
pub struct Skill;

#[derive(Debug)]
pub struct Item;

#[derive(Debug)]
pub struct Enemy;

#[derive(Debug)]
pub struct Troop;

#[derive(Debug)]
pub struct Terrain;

#[derive(Debug)]
pub struct Attribute;

#[derive(Debug)]
pub struct State;

#[derive(Debug)]
pub struct Animation;

#[derive(Debug)]
pub struct Chipset;

#[derive(Debug)]
pub struct Terms;

#[derive(Debug)]
pub struct System;

#[derive(Debug)]
pub struct Switch;

#[derive(Debug)]
pub struct Variable;

#[derive(Debug)]
pub struct CommonEvent;

#[derive(Debug)]
pub struct BattleCommands;

#[derive(Debug)]
pub struct Class;

#[derive(Debug)]
pub struct BattleAnimation;

#[derive(Debug)]
pub struct Parameters;

#[derive(Debug)]
pub struct Equipment;

#[derive(Debug)]
pub struct Learning;


trait Lcf {
    type Output;

    fn read_lcf(reader: &mut impl ReadDelphi) -> Result<Self::Output, Error>;
}

impl<T> Lcf for Vec<T> where T: Lcf {
    type Output = Vec<T::Output>;

    fn read_lcf(reader: &mut impl ReadDelphi) -> Result<Self::Output, Error> {
        let size = reader.read_u8()?;
        let mut res = Vec::with_capacity(size as usize);

        for i in 0..size {
            res.push(T::read_lcf(reader)?);
        }

        Ok(res)
    }
}


