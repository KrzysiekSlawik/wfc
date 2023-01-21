use wfc::wfc::{vec3d::{Vec3D, PosIter3D}, rules, baseline, traits::WFC};

fn main()
{
    let size:usize = 8;
    let map = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
    let rules = rules::get_pipes_rules();
    let solution = baseline::BaseLine::solve(&map, &rules);
    //do something with solution so it is not optimised out
    for (x, y, z) in PosIter3D::new(&map)
    {
        println!("DUMMY anty optimise! {}", solution.get(x, y, z));
    }
}