use std::{collections::HashSet};

use crate::{wfc::traits::WFC, vec3d::Vec3D, wfc::utils::get_random};

use super::{direction_mapping::DirectionMapping, rules::{EMPTY, BORDER}};

pub struct Simple{}
impl WFC for Simple{
    fn solve(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec3D<u8>
    {
        let mut solution = map.clone();
        loop {
            let mut collapsed = false;
            for x in 1..map.x_size-1{
                for y in 1..map.y_size-1{
                    for z in 1.. map.z_size-1{
                        if solution.get(x, y, z) == EMPTY
                        {
                            let legal = Simple::legal_tiles(&solution, x, y, z, rules);
                            if Simple::is_collapsible(&legal){
                                solution.set(x, y, z, *legal.iter().next().unwrap());
                                collapsed = true;
                            }
                        }

                    }
                }
            }
            if !collapsed{
                let mut min = usize::MAX;
                let mut min_position = (0usize,0usize,0usize);//get min position left to fill
                for x in 1..map.x_size-1{
                    for y in 1..map.y_size-1{
                        for z in 1.. map.z_size-1{
                            let number_of_legal_tiles = Simple::legal_tiles(&solution, x, y, z, rules).len();
                            if solution.get(x, y, z) == EMPTY && number_of_legal_tiles > 0 && number_of_legal_tiles < min{
                                min = number_of_legal_tiles;
                                min_position = (x,y,z);
                            }
                        }
                    }
                }
                if min != usize::MAX{ //if found min position left to fill
                    solution.set(
                        min_position.0, min_position.1, min_position.2,
                        get_random(Simple::legal_tiles(&solution, min_position.0, min_position.1, min_position.2, rules)));
                }
                else{
                    return solution;
                }
            }
        }
    }
}

impl Simple
{
    fn legal_tiles(map: &Vec3D<u8>, x: usize, y: usize, z: usize, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> HashSet<u8>
    {
        let possible_tiles = vec![
            rules[map.get(x, y + 1, z) as usize].down(),
            rules[map.get(x, y - 1, z) as usize].up(),
            rules[map.get(x - 1, y, z) as usize].left(),
            rules[map.get(x + 1, y, z) as usize].right(),
            rules[map.get(x, y, z - 1) as usize].front(),
            rules[map.get(x, y, z + 1) as usize].back()];
        possible_tiles
            .iter()
            .skip(1)
            .fold(possible_tiles[0].clone(), |acc, val| {
                acc.intersection(val).cloned().collect()
            })
    }
    fn is_collapsible(legal_tiles: &HashSet<u8>) -> bool{
        legal_tiles.len() == 1
    }
}