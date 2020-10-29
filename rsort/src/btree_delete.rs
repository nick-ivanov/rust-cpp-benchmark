use std::collections::BTreeMap;


pub fn btree_delete(keys: &Vec<u64>, map: &mut BTreeMap<u64, f64>) {
    for i in 0..keys.len() {
        map.remove(&keys[i]);
    }
}

#[test]
fn test_btree_delete() {
    
}