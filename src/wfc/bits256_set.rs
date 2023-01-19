use std::collections::HashSet;

#[derive(Copy, Clone, PartialEq)]
pub struct Bits256Set{
    first: u128,
    second: u128
}

impl Bits256Set{
    pub fn new(first: u128, second: u128) -> Bits256Set
    {
        Bits256Set{first, second}
    }

    pub fn new_from_vec(items: Vec<u8>) -> Bits256Set
    {
        let mut bits = Bits256Set::new_empty();
        for item in items{
            bits.insert(item);
        }
        bits
    }

    pub fn new_any() -> Bits256Set
    {
        Bits256Set::new(u128::MAX, u128::MAX)
    }

    pub fn new_empty() -> Bits256Set
    {
        Bits256Set::new(0, 0)
    }

    pub fn new_from_hash_set(set: &HashSet<u8>) -> Bits256Set
    {
        let mut bits = Bits256Set::new_empty();
        for item in set
        {
            bits.insert(*item);
        }
        bits
    }

    pub fn new_intersection(sets: Vec<Bits256Set>) -> Bits256Set
    {
        let (first, second) = sets.iter().fold((u128::MAX, u128::MAX),|(first, second), x| (first & x.first, second & x.second));
        Bits256Set::new(first, second)
    }

    pub fn new_sum(sets: Vec<Bits256Set>) -> Bits256Set
    {
        let (first, second) = sets.iter().fold((0, 0),|(first, second), x| (first | x.first, second | x.second));
        Bits256Set::new(first, second)
    }

    pub fn contains(&self, x: u8) -> bool
    {
        if x / 128 == 0
        {
            2u128.pow(x as u32) & self.first != 0
        }
        else {
            2u128.pow(x as u32 % 128) & self.second != 0
        }
    }

    pub fn insert(& mut self, x: u8)
    {
        if x / 128 == 0
        {
            self.first = 2u128.pow(x as u32) | self.first;
        }
        else {
            self.second = 2u128.pow(x as u32 % 128) | self.second;
        }
    }

    pub fn remove(& mut self, x: u8)
    {
        if x / 128 == 0
        {
            self.first = 2u128.pow(x as u32) & !self.first;
        }
        else {
            self.second = 2u128.pow(x as u32 % 128) & !self.second;
        }
    }

    pub fn len(&self) -> usize
    {
        (self.first.count_ones() + self.second.count_ones()) as usize
    }

    pub fn items(&self) -> Vec<u8>
    {
        (0..u8::MAX).filter(|&item| self.contains(item)).collect()
    }

}