use std::collections::HashMap;

pub fn hashmap_delete(keys: &Vec<u64>, map: &mut HashMap<u64, f64>) {
    for i in 0..keys.len() {
        map.remove(&keys[i]);
    }
}

#[test]
fn test_hashmap_delete() {
    
}