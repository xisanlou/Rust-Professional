use std::collections::HashSet;

pub fn new_count_distinct(input_str: &str) -> usize {
    let v: Vec<&str> = input_str.split(',').collect();

    let mut set = HashSet::new();
    for s in v.iter() {
        set.insert(s);
    }

    set.len()
}
