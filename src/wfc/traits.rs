use std::collections::HashSet;
use crate::vec3d::Vec3D;

use super::direction_mapping::DirectionMapping;

pub trait WFC{
    fn solve(problem: &Vec3D<u8>, rules: &Vec<DirectionMapping<HashSet<u8>>>) -> Vec3D<u8>;
}