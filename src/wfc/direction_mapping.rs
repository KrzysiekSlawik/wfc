
pub struct DirectionMapping<T>{
    repr: Vec::<T>
}

impl <T> DirectionMapping<T>{
    pub fn new(up: T, down: T, right: T, left: T, front: T, back: T) -> DirectionMapping<T>{
        DirectionMapping{
            repr: vec![up, down, right, left, front, back]
        }
    }
    pub fn up(&self) -> &T{
        &self.repr[0]
    }
    pub fn down(&self) -> &T{
        &self.repr[1]
    }
    pub fn right(&self) -> &T{
        &self.repr[2]
    }
    pub fn left(&self) -> &T{
        &self.repr[3]
    }
    pub fn front(&self) -> &T{
        &self.repr[4]
    }
    pub fn back(&self) -> &T{
        &self.repr[5]
    }
}