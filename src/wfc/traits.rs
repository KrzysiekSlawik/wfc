use std::collections::HashSet;

use super::{direction_mapping::DirectionMapping, vec3d::Vec3D};

pub trait WFC{
    fn solve(problem: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec3D<u8>;
}