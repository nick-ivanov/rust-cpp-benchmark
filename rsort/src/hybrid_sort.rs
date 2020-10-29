pub use crate::rsort_helpers::is_sorted_asc;
pub use crate::merge_sort::merge;
pub use crate::insertion_sort::insertion_sort;
pub use rand::Rng;

// This function is inspired by the code originally written 
// by Dr. Charles Ofria and published here:
// https://raw.githubusercontent.com/mercere99/CSE-830/master/InClass/Week2/TestSorts.cc
pub fn hybrid_sort(v: &mut Vec<f64>, start: isize, mut end: isize, k: usize)
{
    if end == -1 {
        end = v.len() as isize;
    }

    if end - start < k as isize {
        insertion_sort(v, start, end);
        return;
    }

    let midpoint: isize = (start + end) / 2;
    hybrid_sort(v, start, midpoint, k);
    hybrid_sort(v, midpoint, end, k);
    merge(v, start as usize, midpoint as usize, end as usize);
}

// #[test]
// fn test_hybrid_sort() {
//     let mut v = Vec::new();

//     for _ in 0..10_000 {
//         v.push(rand::thread_rng().gen_range(1.0, 65536.0));
//     }

//     let l = v.len();       

//     hybrid_sort(&mut v, 0, l as isize, 20);
//     assert_eq!(is_sorted_asc(&mut v), true);
// }