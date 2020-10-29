pub use crate::rsort_helpers::is_sorted_asc;
pub use rand::Rng;

// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
pub fn merge(v: &mut Vec<f64>, start: usize, midpoint: usize, end: usize) {
    let mut merged = Vec::new(); // Q: How do we know the type? A: Rust's smart type inferece!

    let mut p1 = start;
    let mut p2 = midpoint;

    while p1 < midpoint && p2 < end {
        if v[p1] < v[p2] {
            merged.push(v[p1]);
            p1 += 1;
        }
        else {
            merged.push(v[p2]);
            p2 += 1;
        }
    }

    while p1 < midpoint {
        merged.push(v[p1]);
        p1 += 1;
    }
    while p2 < end {
        merged.push(v[p2]);
        p2 += 1;
    }

    let mut i = 0;
    for a in &merged {
        v[start + i] = *a;
        i += 1;
    }
}


// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
pub fn merge_sort(v: &mut Vec<f64>, start: isize, mut end: isize)
{
    if end == -1 {
        end = v.len() as isize;
    }

    if end - start < 2 {
        return;
    }

    let midpoint: isize = (start + end) / 2;
    merge_sort(v, start, midpoint);
    merge_sort(v, midpoint, end);
    merge(v, start as usize, midpoint as usize, end as usize);
}

#[test]
fn test_merge_sort() {
    let mut v = Vec::new();

    for _ in 0..10_000 {
        v.push(rand::thread_rng().gen_range(1.0, 65536.0));
    }

    let l = v.len();       

    merge_sort(&mut v, 0, l as isize);
    assert_eq!(is_sorted_asc(&mut v), true);
}