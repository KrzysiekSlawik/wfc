use std::collections::HashSet;

use vec3d::Vec3D;
use wfc::{direction_mapping::DirectionMapping};

use crate::wfc::traits::WFC;

mod vec3d;
mod wfc;

fn main() {
    let size = L;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    wfc::queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
}

fn validate(map: &Vec3D<u8>, rules: &[DirectionMapping<HashSet<u8>>]) -> bool
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
                if !possible_tiles.contains(&map.get(x, y, z)) || (possible_tiles.is_empty() && map.get(x, y, z) == wfc::rules::BORDER)
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
const MULTIPIER: usize = 5;
const XS: usize = 4 * MULTIPIER;
const S: usize = 6 * MULTIPIER;
const M: usize = 7 * MULTIPIER;
const L: usize = 8 * MULTIPIER;
const XL: usize = 10 * MULTIPIER;

#[test]
fn baseline_xs(){
    let size:usize = XS;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::baseline::BaseLine::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn baseline_s(){
    let size = S;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::baseline::BaseLine::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn baseline_m(){
    let size = M;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::baseline::BaseLine::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn baseline_l(){
    let size = L;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::baseline::BaseLine::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn baseline_xl(){
    let size = XL;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::baseline::BaseLine::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}

#[test]
fn queueprop_xs(){
    let size:usize = XS;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop::QueueProp::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_s(){
    let size = S;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop::QueueProp::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_m(){
    let size = M;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop::QueueProp::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_l(){
    let size = L;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop::QueueProp::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_xl(){
    let size = XL;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop::QueueProp::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}

#[test]
fn queueprop_bitarrayset_xs(){
    let size:usize = XS;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset::QueuePropBitArraySet::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_s(){
    let size = S;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset::QueuePropBitArraySet::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_m(){
    let size = M;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset::QueuePropBitArraySet::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_l(){
    let size = L;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset::QueuePropBitArraySet::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_xl(){
    let size = XL;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset::QueuePropBitArraySet::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}

#[test]
fn queueprop_bitarrayset_fibheap_xs(){
    let size:usize = XS;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_fibheap_s(){
    let size = S;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_fibheap_m(){
    let size = M;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_fibheap_l(){
    let size = L;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}
#[test]
fn queueprop_bitarrayset_fibheap_xl(){
    let size = XL;
    let map = Vec3D::with_borders(size, size, size, wfc::rules::EMPTY, wfc::rules::BORDER);
    let rules = wfc::rules::get_pipes_rules();
    let solution = wfc::queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
    assert!(validate(&solution, &rules));
}