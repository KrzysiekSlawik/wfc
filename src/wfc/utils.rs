use rand::Rng;

pub fn get_random<T:Copy, I:IntoIterator<Item = T>>(from: I) -> T
{
    let members_vec = Vec::<T>::from_iter(from);
    let random = rand::thread_rng().gen_range(0..members_vec.len());
    return members_vec[random]
}