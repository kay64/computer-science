pub fn sort<V>(vec: &mut Vec<V>)
    where V: Ord + Clone {
    let length = vec.len();

    if length < 2 {
        return;
    }

    let mut i = 0;

    while i < length - 1 {
        let mut j = i + 1;
        let mut k = i;
        while j < length {
            if vec[j] < vec[i] {
                k = j;
            }
            j += 1;
        }

        if k != i {
            vec.swap(i, k);
        }

        i += 1;
    }
}
