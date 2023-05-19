use std::collections::HashSet;
pub fn set_me() {
    let mut set = HashSet::new();
    set.insert("a");
    set.insert("b");
    set.insert("c");
    set.insert("d");
    set.insert("a");

    println!("set: {:?}", set);
    if set.contains("a") {
        println!("set contains a");
    } else {
        println!("set does not contain a");
    }

    // iterate over set
    for item in &set {
        println!("item: {}", item);
    }

    let set1: HashSet<_> = [1, 2, 3, 4, 5].iter().collect();
    let set2: HashSet<_> = [4, 5, 6, 7, 8].iter().collect();

    // Will print [1, 2, 3]
    for item in set1.difference(&set2) {
        println!("{}", item);
    }

    // Will print [4, 5]
    for item in set1.intersection(&set2) {
        println!("{}", item);
    }

    // Will print [1, 2, 3, 6, 7, 8]
    for item in set1.symmetric_difference(&set2) {
        println!("{}", item);
    }

    // Will print [1, 2, 3, 4, 5, 6, 7, 8]
    for item in set1.union(&set2) {
        println!("{}", item);
    }
}
