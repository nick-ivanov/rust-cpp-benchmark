use std::collections::HashMap;

pub fn hashmap_insert(keys: &Vec<u64>, values: &Vec<f64>, map: &mut HashMap<u64, f64>) {
    assert_eq!(keys.len(), values.len());

    for i in 0..keys.len() {
        map.insert(keys[i], values[i]);
    }
}

#[test]
fn test_hashmap_insert() {
    
}