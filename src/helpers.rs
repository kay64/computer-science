use rand::Rng;

pub fn crate_random_vector() -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..20)
        .map(|_| rng.gen_range(0..100))
        .collect()
}