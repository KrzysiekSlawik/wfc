use std::{collections::{HashSet, HashMap, VecDeque}, hash::Hash, env::current_exe};

use crate::{wfc::traits::WFC, vec3d::{Vec3D, PosIter3D}, wfc::utils::get_random};

use super::{direction_mapping::DirectionMapping, rules::{EMPTY, BORDER, self}, utils, bits256_set::Bits256Set};

pub struct QueuePropBitArraySet{}

impl QueuePropBitArraySet
{
    fn prepare_rules(rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec<DirectionMapping<Bits256Set>>
    {
        rules.iter().map(|dir_rules| DirectionMapping::new(
            Bits256Set::new_from_hash_set(dir_rules.up()),
            Bits256Set::new_from_hash_set(dir_rules.down()),
            Bits256Set::new_from_hash_set(dir_rules.right()),
            Bits256Set::new_from_hash_set(dir_rules.left()),
            Bits256Set::new_from_hash_set(dir_rules.front()),
            Bits256Set::new_from_hash_set(dir_rules.back())
        )).collect()
    }
    fn prepare_map(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<Bits256Set>>) -> Vec3D<Bits256Set>
    {
        let mut to_propagate = VecDeque::<(usize, usize, usize)>::new();
        let mut solution = Vec3D::<Bits256Set>::new(map.x_size, map.y_size, map.z_size, Bits256Set::new_from_hash_set(&rules::get_any_tile()));
        for (x, y, z) in PosIter3D::new(&solution){
            let val = map.get(x, y, z);
            if val != EMPTY{
                solution.set(x, y, z, Bits256Set::new_from_vec(Vec::from([val])));
                to_propagate.push_back((x, y, z));
            }
        }
        QueuePropBitArraySet::propagate(&mut solution, rules, & mut to_propagate);
        solution
    }

    fn propagate(solution: & mut Vec3D<Bits256Set>, rules: &Vec<DirectionMapping<Bits256Set>>, to_propagate: & mut VecDeque<(usize,usize,usize)>) -> ()
    {
        while !to_propagate.is_empty(){
            let (x, y, z) = to_propagate.pop_front().unwrap();
            if solution.get(x, y, z).len() > 1
            {
                let updated = QueuePropBitArraySet::legal_tiles(x, y, z, &solution, rules);
                if updated != solution.get(x, y, z) {
                    solution.set(x, y, z, updated);
                    //propagation changed state, need to continue propagation
                    to_propagate.push_back((x+1,y,z));
                    to_propagate.push_back((x-1,y,z));
                    to_propagate.push_back((x,y+1,z));
                    to_propagate.push_back((x,y-1,z));
                    to_propagate.push_back((x,y,z+1));
                    to_propagate.push_back((x,y,z-1));
                }
            }
        }
    }

    fn find_minimal(solution: & mut Vec3D<Bits256Set>) -> Option<(usize,usize,usize, Bits256Set)>
    {
        let mut not_collapsed = PosIter3D::new(&solution)
                .map(|(x,y,z)| (x,y,z,solution.get(x, y, z)))
                .filter(|(_, _, _, v)| v.len() > 1)
                .collect::<Vec<(usize,usize,usize, Bits256Set)>>();
        not_collapsed.sort_unstable_by(|(_,_,_,a),(_,_,_,b)| a.len().partial_cmp(&b.len()).unwrap());
        return not_collapsed.pop()
    }

    fn legal_tiles(x: usize, y: usize, z: usize, map: &Vec3D<Bits256Set>, rules: &Vec<DirectionMapping<Bits256Set>>) -> Bits256Set
    {
        let k = Bits256Set::new_sum(map.get(x,y+1, z).items().iter().map(|&s| *rules[s as usize].down()).collect::<Vec<Bits256Set>>());
        let dirs = vec![
            Bits256Set::new_sum(map.get(x,y+1, z).items().iter().map(|&s| *rules[s as usize].down()).collect::<Vec<Bits256Set>>()),
            Bits256Set::new_sum(map.get(x,y-1, z).items().iter().map(|&s| *rules[s as usize].up()).collect::<Vec<Bits256Set>>()),
            Bits256Set::new_sum(map.get(x-1,y, z).items().iter().map(|&s| *rules[s as usize].left()).collect::<Vec<Bits256Set>>()),
            Bits256Set::new_sum(map.get(x+1,y, z).items().iter().map(|&s| *rules[s as usize].right()).collect::<Vec<Bits256Set>>()),
            Bits256Set::new_sum(map.get(x,y, z-1).items().iter().map(|&s| *rules[s as usize].front()).collect::<Vec<Bits256Set>>()),
            Bits256Set::new_sum(map.get(x,y, z+1).items().iter().map(|&s| *rules[s as usize].back()).collect::<Vec<Bits256Set>>())
        ];
        Bits256Set::new_intersection(dirs)
    }

    fn format_solution(solution: &Vec3D<Bits256Set>) -> Vec3D<u8>
    {
        let mut ret = Vec3D::new(solution.x_size, solution.y_size, solution.z_size, EMPTY);
        for (x, y, z) in PosIter3D::new(&solution)
        {
            ret.set(x, y, z, solution.get(x, y, z).items().into_iter().next().unwrap_or(EMPTY));
        }
        ret
    }


}

impl WFC for QueuePropBitArraySet{
    fn solve(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec3D<u8>
    {

        let rules_internal = QueuePropBitArraySet::prepare_rules(rules);
        let mut solution = QueuePropBitArraySet::prepare_map(map, &rules_internal);
        loop {
            //find minimal non zero entropy
            let minimal = QueuePropBitArraySet::find_minimal(&mut solution);
            match minimal {
                Some((x,y, z, current)) => {
                    //minimal found setting it randomly
                    solution.set(x, y, z, Bits256Set::new_from_vec(vec![utils::get_random(current.items())]));
                    let mut to_propagate = VecDeque::<(usize,usize,usize)>::new();
                    to_propagate.push_back((x+1,y,z));
                    to_propagate.push_back((x-1,y,z));
                    to_propagate.push_back((x,y+1,z));
                    to_propagate.push_back((x,y-1,z));
                    to_propagate.push_back((x,y,z+1));
                    to_propagate.push_back((x,y,z-1));
                    QueuePropBitArraySet::propagate(&mut solution, &rules_internal, &mut to_propagate);
                },
                None => {
                    //nothing left to be collapsed, returning solution
                    return QueuePropBitArraySet::format_solution(&solution);
                }
            }
        }
    }
}