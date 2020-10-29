pub use crate::rsort_helpers::is_sorted_asc;
pub use rand::Rng;

// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
pub fn insertion_sort(v: &mut Vec<f64>, start: isize, end: isize) {

    if v.len() < 2 {
        return;
    }

    for i in start+1..end {
        let key = v[i as usize];
        let mut j = (i as i64) - 1;

        while j >= 0 && v[j as usize] > key {
            v[(j + 1) as usize] = v[j as usize];
            j -= 1;
        }

        v[(j + 1) as usize] = key;
    }
}

#[test]
fn test_insertion_sort() {
    let mut v = Vec::new();

    for _ in 0..10_000 {
        v.push(rand::thread_rng().gen_range(1.0, 65536.0));
    }

    let l = v.len();
    insertion_sort(&mut v, 0, l as isize);
    assert_eq!(is_sorted_asc(&mut v), true);
}
