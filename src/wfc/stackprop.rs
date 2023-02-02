use std::collections::HashSet;

use super::{direction_mapping::DirectionMapping, rules::{EMPTY, self}, utils, vec3d::{Vec3D, PosIter3D}, traits::WFC};

pub struct StackProp{}

impl StackProp
{
    fn vec_bool_set_from_set(set: &HashSet<u8>) -> Vec<bool>
    {
        (0..u8::MAX).into_iter().map(|x| set.contains(&x)).collect()
    }
    fn vec_bool_set_from(collection: &[u8]) -> Vec<bool>
    {
        (0..u8::MAX).into_iter().map(|x| collection.contains(&x)).collect()
    }
    fn prepare_rules(rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec<DirectionMapping<Vec<bool>>>
    {
        rules.iter().map(|dir_rules| DirectionMapping::new(
            Self::vec_bool_set_from_set(&dir_rules.up()),
            Self::vec_bool_set_from_set(&dir_rules.down()),
            Self::vec_bool_set_from_set(&dir_rules.right()),
            Self::vec_bool_set_from_set(&dir_rules.left()),
            Self::vec_bool_set_from_set(&dir_rules.front()),
            Self::vec_bool_set_from_set(&dir_rules.back()),
        )).collect()
    }
    fn prepare_map(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<Vec<bool>>>) -> Vec3D<Vec<bool>>
    {
        let mut to_propagate = Vec::<(usize, usize, usize)>::new();
        let mut solution = Vec3D::<Vec<bool>>::new(map.x_size, map.y_size, map.z_size, Self::vec_bool_set_from_set(&rules::get_any_tile()));
        for (x, y, z) in PosIter3D::new(&solution){
            let val = map.get(x, y, z);
            if val != EMPTY{
                solution.set(x, y, z, Self::vec_bool_set_from(&[val]));
                to_propagate.push((x, y, z));
            }
        }
        Self::propagate(&mut solution, rules, & mut to_propagate);
        solution
    }

    fn propagate(solution: & mut Vec3D<Vec<bool>>, rules: &Vec<DirectionMapping<Vec<bool>>>, to_propagate: & mut Vec<(usize,usize,usize)>) -> ()
    {
        while !to_propagate.is_empty(){
            let (x, y, z) = to_propagate.pop().unwrap();
            if solution.get(x, y, z).iter().filter(|&&x| x).count() > 1
            {
                let updated = Self::legal_tiles(x, y, z, &solution, rules);
                if updated != solution.get(x, y, z) {
                    solution.set(x, y, z, updated);
                    //propagation changed state, need to continue propagation
                    to_propagate.push((x+1,y,z));
                    to_propagate.push((x-1,y,z));
                    to_propagate.push((x,y+1,z));
                    to_propagate.push((x,y-1,z));
                    to_propagate.push((x,y,z+1));
                    to_propagate.push((x,y,z-1));
                }
            }
        }
    }

    fn find_minimal(solution: &Vec3D<Vec<bool>>) -> Option<(usize,usize,usize, Vec<bool>)>
    {
        PosIter3D::new(solution)
        .map(|(x,y,z)| Some((x, y, z, solution.get(x, y, z))))
        .fold(None, |acc, x| {
            match acc
            {
                Some(a) => {
                    let x = x.unwrap();
                    let next_len = x.3.iter().filter(|&&x|x).count();
                    let this_len = a.3.iter().filter(|&&x|x).count();
                    if  next_len > 1 && next_len < this_len
                    {
                        Some(x)
                    }
                    else
                    {
                        Some(a)
                    }
                },
                None =>
                {
                    let x = x.unwrap();
                    let next_len = x.3.iter().filter(|&&x|x).count();
                    if next_len > 1
                    {
                        Some(x)
                    }
                    else
                    {
                        None
                    }
                }
            }
        })
    }

    fn legal_tiles(x: usize, y: usize, z: usize, solution: &Vec3D<Vec<bool>>, rules: &Vec<DirectionMapping<Vec<bool>>>) -> Vec<bool>
    {
        let w: Vec<Vec<bool>> = vec![
            solution.get(x, y + 1, z)
               .iter()
               .zip(0..u8::MAX)
               .filter_map(|(b, idx)| match b{true => Some(idx), false => None})
               .map(|idx| rules[idx as usize].down())
               .fold(vec![false;u8::MAX as usize],|acc:Vec<bool>, b| acc.iter()
                                                                        .zip(b)
                                                                        .map(|(&a,&b)| a||b)
                                                                        .collect()),
            solution.get(x, y - 1, z)
               .iter()
               .zip(0..u8::MAX)
               .filter_map(|(b, idx)| match b{true => Some(idx), false => None})
               .map(|idx| rules[idx as usize].up())
               .fold(vec![false;u8::MAX as usize],|acc:Vec<bool>, b| acc.iter()
                                                                        .zip(b)
                                                                        .map(|(&a,&b)| a||b)
                                                                        .collect()),
            solution.get(x - 1, y, z)
               .iter()
               .zip(0..u8::MAX)
               .filter_map(|(b, idx)| match b{true => Some(idx), false => None})
               .map(|idx| rules[idx as usize].left())
               .fold(vec![false;u8::MAX as usize],|acc:Vec<bool>, b| acc.iter()
                                                                        .zip(b)
                                                                        .map(|(&a,&b)| a||b)
                                                                        .collect()),
            solution.get(x + 1, y, z)
               .iter()
               .zip(0..u8::MAX)
               .filter_map(|(b, idx)| match b{true => Some(idx), false => None})
               .map(|idx| rules[idx as usize].right())
               .fold(vec![false;u8::MAX as usize],|acc:Vec<bool>, b| acc.iter()
                                                                        .zip(b)
                                                                        .map(|(&a,&b)| a||b)
                                                                        .collect()),
            solution.get(x, y, z - 1)
               .iter()
               .zip(0..u8::MAX)
               .filter_map(|(b, idx)| match b{true => Some(idx), false => None})
               .map(|idx| rules[idx as usize].front())
               .fold(vec![false;u8::MAX as usize],|acc:Vec<bool>, b| acc.iter()
                                                                        .zip(b)
                                                                        .map(|(&a,&b)| a||b)
                                                                        .collect()),
            solution.get(x, y, z + 1)
               .iter()
               .zip(0..u8::MAX)
               .filter_map(|(b, idx)| match b{true => Some(idx), false => None})
               .map(|idx| rules[idx as usize].back())
               .fold(vec![false;u8::MAX as usize],|acc:Vec<bool>, b| acc.iter()
                                                                        .zip(b)
                                                                        .map(|(&a,&b)| a||b)
                                                                        .collect())];

        w.iter().fold(vec![true; u8::MAX as usize], |acc, x| acc.iter()
                                                                                               .zip(x)
                                                                                               .map(|(&a,&b)| a && b)
                                                                                               .collect())
    }

    fn format_solution(solution: &Vec3D<Vec<bool>>) -> Vec3D<u8>
    {
        let mut ret = Vec3D::new(solution.x_size, solution.y_size, solution.z_size, EMPTY);
        for (x, y, z) in PosIter3D::new(&solution)
        {
            ret.set(x, y, z, solution.get(x, y, z).iter().zip(0..u8::MAX).filter_map(|(&egz, x)|
                {
                    if egz
                    {
                        Some(x)
                    }
                    else {
                        None
                    }
                }).next().unwrap_or(EMPTY));
        }
        ret
    }
}

impl WFC for StackProp{
    fn solve(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec3D<u8>
    {
        let rules = &Self::prepare_rules(rules);
        //prepare format
        let mut solution = Self::prepare_map(map, rules);
        loop {
            //find minimal non zero entropy
            let minimal = StackProp::find_minimal(&mut solution);
            match minimal {
                Some((x,y, z, current)) => {
                    //minimal found setting it randomly
                    solution.set(x, y, z, Self::vec_bool_set_from(&[utils::get_random(current.iter()
                                                                                                                  .zip(0..u8::MAX)
                                                                                                                  .filter_map(|(&egz,x)|{
                                                                                                                    if egz{
                                                                                                                        Some(x)
                                                                                                                    }
                                                                                                                    else {
                                                                                                                        None
                                                                                                                    }
                                                                                                                  }))]));
                    let mut to_propagate = Vec::<(usize,usize,usize)>::new();
                    to_propagate.push((x+1,y,z));
                    to_propagate.push((x-1,y,z));
                    to_propagate.push((x,y+1,z));
                    to_propagate.push((x,y-1,z));
                    to_propagate.push((x,y,z+1));
                    to_propagate.push((x,y,z-1));
                    StackProp::propagate(&mut solution, rules, &mut to_propagate);
                },
                None => {
                    //nothing left to be collapsed, returning solution
                    return Self::format_solution(&solution);
                }
            }
        }
    }
}