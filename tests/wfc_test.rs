use std::collections::HashSet;

use wfc::wfc::{vec3d::Vec3D, direction_mapping::DirectionMapping, baseline, traits::WFC, rules, queueprop, queueprop_bitarrayset, queueprop_bitarrayset_fibheap, stackprop};

fn validate(map: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> bool
{
    for x in 1..map.x_size-1{
        for y in 1..map.y_size-1{
            for z in 1..map.z_size-1{
                let possible_tiles_dirs = vec![
                    rules[map.get(x, y + 1, z) as usize].down(),
                    rules[map.get(x, y - 1, z) as usize].up(),
                    rules[map.get(x - 1, y, z) as usize].left(),
                    rules[map.get(x + 1, y, z) as usize].right(),
                    rules[map.get(x, y, z - 1) as usize].front(),
                    rules[map.get(x, y, z + 1) as usize].back()];
                let possible_tiles = possible_tiles_dirs
                    .iter()
                    .skip(1)
                    .fold(possible_tiles_dirs[0].clone(), |acc, val| {
                        acc.intersection(val).cloned().collect()
                    });
                if !possible_tiles.contains(&map.get(x, y, z)) || (possible_tiles.is_empty() && map.get(x, y, z) == rules::BORDER)
                {
                    println!("invalid value: {:#010b}", map.get(x, y, z));
                    println!("up:          : {:#010b}", map.get(x, y+1, z));
                    println!("down         : {:#010b}", map.get(x, y-1, z));
                    println!("left         : {:#010b}", map.get(x+1, y, z));
                    println!("right        : {:#010b}", map.get(x-1, y, z));
                    println!("front        : {:#010b}", map.get(x, y, z+1));
                    println!("back         : {:#010b}", map.get(x, y, z-1));
                    return false;
                }
            }
        }
    }
    true
}

#[test]
fn test_baseline(){
    let size:usize = 8;
    let map = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
    let rules = rules::get_pipes_rules();
    let solution = baseline::BaseLine::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}

#[test]
fn test_queueprop(){
    let size:usize = 20;
    let map = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
    let rules = rules::get_pipes_rules();
    let solution = queueprop::QueueProp::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}

#[test]
fn test_stackprop(){
    let size:usize = 20;
    let map = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
    let rules = rules::get_pipes_rules();
    let solution = stackprop::StackProp::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}

#[test]
fn test_queueprop_bitarrayset(){
    let size:usize = 8;
    let map = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
    let rules = rules::get_pipes_rules();
    let solution = queueprop_bitarrayset::QueuePropBitArraySet::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}

#[test]
fn test_queueprop_bitarrayset_fibheap(){
    let size:usize = 8;
    let map = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
    let rules = rules::get_pipes_rules();
    let solution = queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}