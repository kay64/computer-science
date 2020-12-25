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

#[test]
fn bubble() {
    invoke("Bubble", super::bubble::sort)
}

#[test]
fn gnome() {
    invoke("Gnome", super::gnome::sort)
}

#[test]
fn shaker() {
    invoke("Shaker", super::shaker::sort)
}

#[test]
fn merge() {
    invoke("Merge", super::merge::sort)
}
