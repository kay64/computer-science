pub fn sort<V>(vec: &mut Vec<V>)
    where V: Ord + Clone {
    let length = vec.len();

    if length < 2 {
        return;
    }

    let mut i = 1;
    while i < length {
        let mut j = i;
        while j > 0 && vec[j - 1] > vec[j] {
            vec.swap(j - 1, j);
            j -= 1;
        }
        i += 1;
    }
}
