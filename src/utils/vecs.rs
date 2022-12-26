use std::ops::AddAssign;

pub fn add_vecs<const N: usize, T>(fst: &[T; N], snd: &[T; N]) -> [T; N]
where
    T: Copy + AddAssign<T>,
{
    let mut res = *fst;
    for i in 0..N {
        res[i] += snd[i];
    }
    res
}
