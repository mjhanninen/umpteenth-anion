// Copyright (C) 2016 Matti Hänninen
//
// This file is part of Umpteenth Anion.
//
// Umpteenth Anion is free software: you can redistribute it and/or modify it
// under the terms of the GNU General Public License as published by the Free
// Software Foundation, either version 3 of the License, or (at your option)
// any later version.
//
// Umpteenth Anion is distributed in the hope that it will be useful, but
// WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY
// or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU General Public License
// for more details.
//
// You should have received a copy of the GNU General Public License along
// with Umpteenth Anion.  If not, see <http://www.gnu.org/licenses/>.

use space::Space;

pub type Tag = u8;

pub type Production = i16;

pub type Strength = i16;

#[derive(Debug)]
pub struct Environment
{
    pub my_tag: Tag,
    pub total_turns: u32,
    pub space: Space,
    pub production_map: Vec<Production>,
}

fn total_turns(width: i16, height: i16) -> u32
{
    ((width * height) as f32).sqrt().floor() as u32
}

impl Environment
{
    pub fn create(my_tag: Tag, width: i16, height: i16) -> Self
    {
        let space = Space::with_dims(width, height);
        let production_map = Vec::with_capacity(space.len());
        Environment {
            my_tag: my_tag,
            total_turns: total_turns(width, height),
            space: space,
            production_map: production_map,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Occupation
{
    pub tag: Tag,
    pub strength: Strength,
}

#[derive(Debug)]
pub struct State
{
    pub turn: u32,
    pub occupation_map: Vec<Occupation>,
}

impl State
{
    pub fn for_environment(environment: &Environment) -> Self
    {
        State {
            turn: 0,
            occupation_map:
                vec![Occupation {tag: 0, strength: 0}; environment.space.len()],
        }
    }
}
