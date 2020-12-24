fn invoke<F>(name: &str, sort: F) where F: Fn(&mut Vec<i32>) {
    let mut vec = crate::helpers::crate_random_vector();

    println!("\nSorting : {}", name);
    println!("Initial : {:?}", &vec);
    sort(&mut vec);
    println!("Sorted  : {:?}", &vec);
}

#[test]
fn insertion() {
    invoke("Insertion", super::insertion::sort)
}

#[test]
fn selection() {
    invoke("Selection", super::selection::sort)
}
