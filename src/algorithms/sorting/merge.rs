fn merge<V>(left: Vec<V>, right: Vec<V>) -> Vec<V>
    where V: Ord + Clone {
    let mut result = Vec::<V>::with_capacity(left.len() + right.len());

    let mut i = 0;
    let mut j = 0;
    while i < left.len() && j < right.len() {
        result.insert(
            i + j,
            if left[i] < right[j] {
                i += 1;
                left[i - 1].clone()
            } else {
                j += 1;
                right[j - 1].clone()
            },
        );
    }

    while i < left.len() {
        result.insert(i + j, left[i].clone());
        i += 1;
    }

    while j < right.len() {
        result.insert(i + j, right[j].clone());
        j += 1;
    }

    result
}

fn split_and_merge<V>(vec: &[V]) -> Vec<V>
    where V: Ord + Clone {
    let length = vec.len();
    if length < 2 {
        return Vec::from(vec);
    }

    let middle = length / 2;

    let left = split_and_merge(&vec[0..middle]);
    let right = split_and_merge(&vec[middle..length]);

    return merge(left, right);
}

pub fn sort<V>(vec: &mut Vec<V>)
    where V: Ord + Clone {
    let length = vec.len();

    if length < 2 {
        return;
    }

    *vec = split_and_merge(vec.as_slice());
}
