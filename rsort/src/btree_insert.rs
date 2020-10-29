use std::collections::BTreeMap;

pub fn btree_insert(keys: &Vec<u64>, values: &Vec<f64>, map: &mut BTreeMap<u64, f64>) {
    assert_eq!(keys.len(), values.len());

    for i in 0..keys.len() {
        map.insert(keys[i], values[i]);
    }
}

#[test]
fn test_btree_insert() {
    
}