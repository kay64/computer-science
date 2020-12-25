pub fn sort<V>(vec: &mut Vec<V>)
    where V: Ord + Clone {
    let length = vec.len();

    if length < 2 {
        return;
    }

    let mut start = 0;
    let mut end = length - 1;
    while start < end {
        let mut i = start;
        while i < end {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
            }
            i += 1;
        }

        i -= 1;
        end -= 1;

        while i > start {
            if vec[i] < vec[i - 1] {
                vec.swap(i, i - 1);
            }
            i -= 1;
        }

        start += 1;
    }
}
