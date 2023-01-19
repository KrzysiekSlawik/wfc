use std::{collections::HashSet, hash::Hash, clone};

use super::direction_mapping::DirectionMapping;

const UP: u8    = 0b1000_0000;
const DOWN: u8  = 0b0100_0000;
const LEFT: u8  = 0b0010_0000;
const RIGHT: u8 = 0b0001_0000;
const FRONT: u8 = 0b0000_1000;
const BACK: u8  = 0b0000_0100;
const RED: u8   = 0b0000_0000;
const GREEN: u8 = 0b0000_0001;
const BLUE: u8  = 0b0000_0010;
const SPECIAL: u8 = 0b00000011;
pub const EMPTY: u8  = 0b00000011;
pub const BORDER: u8 = 0b00000111;
const TYPE_MASK: u8 = 0b0000_0011;

fn is_special_type(a: u8) -> bool
{
    a == EMPTY || a == BORDER
}

fn is_pipe(a: u8) -> bool
{
    (a & TYPE_MASK) ^ TYPE_MASK != 0
}

fn is_pipe_with_direction(a: u8, dir: u8) -> bool
{
    is_pipe(a) && a & dir != 0
}

fn is_pipe_without_direction(a: u8, dir: u8) -> bool
{
    is_pipe(a) && a & dir == 0
}

fn get_oposite_dir(dir: u8) -> u8
{
    match dir{
        UP => DOWN,
        DOWN => UP,
        LEFT => RIGHT,
        RIGHT => LEFT,
        FRONT => BACK,
        BACK => FRONT,
        _ => 0
    }
}

fn get_pipe_dir_rules(i: u8, dir: u8, pipe_type: u8) -> HashSet<u8>
{
    if i & dir != 0
    {
        HashSet::from_iter(
            (0..u8::MAX).filter(|&a| is_pipe_with_direction(a, get_oposite_dir(dir)) && a & TYPE_MASK == pipe_type || a == BORDER)
        )
    }
    else
    {
        HashSet::from_iter(
            (0..u8::MAX).filter(|&a| is_pipe_without_direction(a, get_oposite_dir(dir)) || a == BORDER)
        )
    }
}

pub fn get_any_tile() -> HashSet<u8>
{
    (0..u8::MAX).filter(|&a| is_pipe(a) || a == BORDER).collect()
}

fn get_pipe_rules(i: u8) -> DirectionMapping<HashSet<u8>>
{
    let pipe_type = i & TYPE_MASK;
    if !is_pipe(i) && !is_special_type(i)
    {
        return DirectionMapping::new(
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new(),
            HashSet::new()
        );
    }
    if pipe_type == SPECIAL{
        return DirectionMapping::new(
            get_any_tile(),
            get_any_tile(),
            get_any_tile(),
            get_any_tile(),
            get_any_tile(),
            get_any_tile()
        );
    }
    DirectionMapping::new(
        get_pipe_dir_rules(i, UP, pipe_type),
        get_pipe_dir_rules(i, DOWN, pipe_type),
        get_pipe_dir_rules(i, RIGHT, pipe_type),
        get_pipe_dir_rules(i, LEFT, pipe_type),
        get_pipe_dir_rules(i, FRONT, pipe_type),
        get_pipe_dir_rules(i, BACK, pipe_type)
    )
}

pub fn get_pipes_rules() -> Vec<DirectionMapping<HashSet<u8>>>
{
    (0..u8::MAX).map(|a| get_pipe_rules(a)).collect()
}
