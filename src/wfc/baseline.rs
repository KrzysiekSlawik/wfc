use std::{collections::{HashSet, HashMap}};

use super::{direction_mapping::DirectionMapping, rules::{EMPTY, self}, utils, vec3d::{Vec3D, PosIter3D}, traits::WFC};

pub struct BaseLine{}

impl BaseLine
{
    fn prepare(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec3D<HashSet<u8>>
    {
        let mut solution = Vec3D::<HashSet<u8>>::new(map.x_size, map.y_size, map.z_size, rules::get_any_tile());
        for (x, y, z) in PosIter3D::new(&solution){
            let val = map.get(x, y, z);
            if val != EMPTY{
                solution.set(x, y, z, HashSet::from([val]));
            }
        }
        BaseLine::propagate(&mut solution, rules);
        solution
    }

    fn propagate(solution: & mut Vec3D<HashSet<u8>>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> ()
    {
        let mut changed = true;
        while changed{
            changed = false;
            for (x, y, z) in PosIter3D::new_no_border(&solution) {
                if solution.get(x, y, z).len() > 1
                {
                    let updated = BaseLine::legal_tiles(x, y, z, &solution, rules);
                    if updated != solution.get(x, y, z) {
                        solution.set(x, y, z, updated);
                        changed = true;
                        //propagation changed state, need to continue propagation
                    }
                }
            }
        }
    }

    fn find_minimal(solution: & mut Vec3D<HashSet<u8>>) -> Option<(usize,usize,usize, HashSet<u8>)>
    {
        let mut not_collapsed = PosIter3D::new(&solution)
                .map(|(x,y,z)| (x,y,z,solution.get(x, y, z)))
                .filter(|(_, _, _, v)| v.len() > 1)
                .collect::<Vec<(usize,usize,usize, HashSet<u8>)>>();
        not_collapsed.sort_unstable_by(|(_,_,_,a),(_,_,_,b)| a.len().partial_cmp(&b.len()).unwrap());
        return not_collapsed.pop()
    }

    fn legal_tiles(x: usize, y: usize, z: usize, map: &Vec3D<HashSet<u8>>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> HashSet<u8>
    {
        let w: Vec<Vec<u8>> = vec![
            map.get(x, y + 1, z).iter()
                .map(|&s| rules[s as usize].down())
                .flatten()
                .map(|&a| a)
                .collect(),
            map.get(x, y - 1, z).iter()
                .map(|&s| rules[s as usize].up())
                .flatten()
                .map(|&a| a)
                .collect(),
            map.get(x - 1, y, z).iter()
                .map(|&s| rules[s as usize].left())
                .flatten()
                .map(|&a| a)
                .collect(),
            map.get(x + 1, y, z).iter()
                .map(|&s| rules[s as usize].right())
                .flatten()
                .map(|&a| a)
                .collect(),
            map.get(x, y, z - 1).iter()
                .map(|&s| rules[s as usize].front())
                .flatten()
                .map(|&a| a)
                .collect(),
            map.get(x, y, z + 1).iter()
                .map(|&s| rules[s as usize].back())
                .flatten()
                .map(|&a| a)
                .collect()];

        let mut deduped = Vec::<Vec<u8>>::new();
        for mut c in w{
            c.sort_unstable();
            c.dedup();
            deduped.push(c);
        }
        let flat = deduped.iter().flatten();
        let mut map: HashMap<u8, usize> = HashMap::new();
        for item in flat
        {
            map.entry(*item).and_modify(|i| *i += 1).or_insert(1);
        }
        let filtered = map.iter().filter_map(|(key, value)| {
            if *value == 6{
                Some(key)
            }
            else
            {
                None
            }
        });

        let r = HashSet::<u8>::from_iter(filtered.map(|&a| a));
        r
    }

    fn format_solution(solution: &Vec3D<HashSet<u8>>) -> Vec3D<u8>
    {
        let mut ret = Vec3D::new(solution.x_size, solution.y_size, solution.z_size, EMPTY);
        for (x, y, z) in PosIter3D::new(&solution)
        {
            ret.set(x, y, z, solution.get(x, y, z).into_iter().next().unwrap_or(EMPTY));
        }
        ret
    }


}

impl WFC for BaseLine{
    fn solve(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec3D<u8>
    {
        //prepare format
        let mut solution = BaseLine::prepare(map, rules);
        loop {
            //propagation
            BaseLine::propagate(&mut solution, rules);
            //find minimal non zero entropy
            let minimal = BaseLine::find_minimal(&mut solution);
            match minimal {
                Some((x,y, z, current)) => {
                    //minimal found setting it randomly
                    solution.set(x, y, z, HashSet::from([utils::get_random(current.clone())]));
                },
                None => {
                    //nothing left to be collapsed, returning solution
                    return BaseLine::format_solution(&solution);
                }
            }
        }
    }
}