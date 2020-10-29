pub fn is_sorted_asc(v: &mut Vec<f64>) -> bool {

    let mut prev = v[0];
    for i in 1..v.len() {
        if v[i] < prev {
            return false;
        }
        prev = v[i]
    }

    true
}

pub fn print_usage_and_quit(ret: i32) {
    println!("USAGE:");
    println!("rsort merge");
    println!("rsort insert");
    println!("rsort hybrid");
    println!("rsort hashmap");
    println!("rsort bintree");
    std::process::exit(ret);
}