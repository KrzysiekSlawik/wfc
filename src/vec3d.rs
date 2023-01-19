#[derive(Clone)]
pub struct Vec3D<T>{
    repr: Vec::<T>,
    pub x_size: usize,
    pub y_size: usize,
    pub z_size: usize,
}

impl<T:Clone> Vec3D<T>{
    pub fn new(x_size: usize, y_size: usize, z_size: usize, init: T) -> Vec3D<T>
    {
        Vec3D{
            repr: vec![init; x_size * y_size * z_size],
            x_size,
            y_size,
            z_size,
        }
    }
    pub fn with_borders(x_size: usize, y_size: usize, z_size: usize, interior: T, border: T) -> Vec3D<T>
    {
        let mut a = Vec3D::new(x_size, y_size, z_size, interior);
        for i in 0..x_size{
            for j in 0..y_size{
                a.set(i, j, 0, border.clone());
                a.set(i, j, z_size-1, border.clone());
                a.set(0, i, j, border.clone());
                a.set(x_size-1, i, j, border.clone());
                a.set(i, 0, j, border.clone());
                a.set(i, y_size-1, j, border.clone());
            }
        }
        a
    }
    pub fn get(&self, x: usize, y: usize, z: usize) -> T
    {
        self.repr[x + (y * self.x_size) + (z * self.x_size * self.y_size)].clone()
    }
    pub fn set(&mut self, x: usize, y: usize, z: usize, val: T)
    {
        self.repr[x + (y * self.x_size) + (z * self.x_size * self.y_size)] = val
    }
}

pub struct PosIter3D
{
    size: (usize, usize, usize),
    i: usize,
    no_border: bool
}

impl PosIter3D
{
    pub fn new<T>(from: &Vec3D<T>) -> PosIter3D
    {
        PosIter3D {size: (from.x_size, from.y_size, from.z_size), i: 0, no_border: false}
    }

    pub fn new_no_border<T>(from: &Vec3D<T>) -> PosIter3D
    {
        PosIter3D {size: (from.x_size-2, from.y_size-2, from.z_size-2), i: 0, no_border: true}
    }
}
impl Iterator for PosIter3D
{
    type Item = (usize, usize, usize);
    fn next(&mut self) -> Option<Self::Item> {
        let (x, y, z) = self.size;
        if self.i < x * y * z{
            self.i += 1;
            let pos = get_pos_from_index(self.size, self.i-1);
            if self.no_border{
                Some((pos.0+1, pos.1+1, pos.2+1))
            }
            else {
                Some(pos)
            }
        }
        else {
            None
        }
    }
}
fn get_pos_from_index((x_size, y_size, z_size): (usize, usize, usize), i: usize) -> (usize, usize, usize)
    {
        let x = i % x_size;
        let y = (i/x_size) % y_size;
        let z = (i/(x_size*y_size)) % z_size;
        (x, y, z)
    }