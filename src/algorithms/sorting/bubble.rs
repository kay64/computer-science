pub fn sort<V>(vec: &mut Vec<V>)
    where V: Ord + Clone {
    let length = vec.len();

    if length < 2 {
        return;
    }

    let mut end = length - 1;

    while end > 0 {
        let mut i = 0;
        while i < end {
            if vec[i] > vec[i + 1] {
                vec.swap(i, i + 1);
            }

            i += 1;
        }
        end -= 1;
    }
}
