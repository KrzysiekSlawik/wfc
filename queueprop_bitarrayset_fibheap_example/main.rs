use wfc::wfc::{vec3d::{Vec3D, PosIter3D}, rules, queueprop_bitarrayset_fibheap, traits::WFC};

fn main()
{
    let size:usize = 35;
    let map = Vec3D::with_borders(size, size, size, rules::EMPTY, rules::BORDER);
    let rules = rules::get_pipes_rules();
    let solution = queueprop_bitarrayset_fibheap::QueuePropBitArraySetFibHeap::solve(&map, &rules);
    //do something with solution so it is not optimised out
    for (x, y, z) in PosIter3D::new(&map)
    {
        println!("DUMMY anty optimise! {}", solution.get(x, y, z));
    }
}