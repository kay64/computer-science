pub fn sort<V>(vec: &mut Vec<V>)
    where V: Ord + Clone {
    let length = vec.len();

    if length < 2 {
        return;
    }

    let mut i = 1;

    while i < length {
        let value = vec[i].clone();
        let mut j = i;
        while j > 0 && value < vec[j - 1] {
            vec[j] = vec[j - 1].clone();
            j = j - 1;
        }

        vec[j] = value;
        i += 1;
    }
}
