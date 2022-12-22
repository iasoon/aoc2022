pub fn add_vecs<const N: usize>(fst: &[isize; N], snd: &[isize; N]) -> [isize; N] {
    let mut res = *fst;
    for i in 0..N {
        res[i] += snd[i];
    }
    res
}
