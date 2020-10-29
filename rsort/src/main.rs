mod rsort_helpers;
mod merge_sort;
mod insertion_sort;
mod hybrid_sort;
mod hashmap_insert;
mod hashmap_delete;
mod btree_insert;
mod btree_delete;

pub use rsort_helpers::is_sorted_asc;
pub use merge_sort::merge_sort;
pub use merge_sort::merge;
pub use insertion_sort::insertion_sort;
pub use hybrid_sort::hybrid_sort;
pub use rsort_helpers::print_usage_and_quit;
pub use hashmap_insert::hashmap_insert;
pub use hashmap_delete::hashmap_delete;
pub use btree_insert::btree_insert;
pub use btree_delete::btree_delete;

//use std::str::FromStr;
use std::env;
use rand::Rng;
use std::time::{SystemTime, UNIX_EPOCH};
use std::iter::Iterator;
use std::collections::BTreeMap;
use std::collections::HashMap;
use std::io::Write;


fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.len() < 1 {
        print_usage_and_quit(1);
        std::process::exit(1);
    }

    let command = args[0].trim().to_string();

    match command.as_str() {
        "merge" => {
            let number_of_runs = 10_000;
            for vector_size in (25..=1_000).step_by(25) {
                let mut total_time: f64 = 0.0;
                for _ in 0..number_of_runs {
                    let mut v = Vec::new();

                    for _ in 0..vector_size {
                        v.push(rand::thread_rng().gen_range(1.0, 65536.0));
                    }

                    let l = v.len();
                
                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();            
                
                    merge_sort(&mut v, 0, l as isize);
                                
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();
                
                    let elapsed = ((end_time - start_time) as f64) / 1_000_000.0;
                    total_time += elapsed;
                }

                print!("{}, ", (total_time / (number_of_runs as f64)));
                std::io::stdout().flush().unwrap();
            }
        },

        "insert" => {
            let number_of_runs = 10_000;
            for vector_size in (25..=1_000).step_by(25) {
                let mut total_time: f64 = 0.0;
                for _ in 0..number_of_runs {
                    let mut v = Vec::new();

                    for _ in 0..vector_size {
                        v.push(rand::thread_rng().gen_range(1.0, 65536.0));
                    }
                
                    let l = v.len();
                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();            
                            
                    
                    insertion_sort(&mut v, 0, l as isize);
                    
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();
                
                    let elapsed = ((end_time - start_time) as f64) / 1_000_000.0;
                    total_time += elapsed;
                }

                print!("{}, ", (total_time / (number_of_runs as f64)));
                std::io::stdout().flush().unwrap();
            }
        },

        "hybrid" => {
            let hybrid_threshold = 512;
            let number_of_runs = 10_000;
        
            for vector_size in (250..=10000).step_by(250) {
                let mut total_time: f64 = 0.0;
                for _ in 0..number_of_runs {
                    let mut v = Vec::new();

                    for _ in 0..vector_size {
                        v.push(rand::thread_rng().gen_range(1.0, 65536.0));
                    }

                    let l = v.len();
                
                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();            
                
                    hybrid_sort(&mut v, 0, l as isize, hybrid_threshold as usize);
                                
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();
                
                    let elapsed = ((end_time - start_time) as f64) / 1_000_000.0;
                    total_time += elapsed;
                }

                print!("{}, ", (total_time / (number_of_runs as f64)));
                std::io::stdout().flush().unwrap();
            }
        },

        "hashmap" => {
            let mut res1: Vec<f64> = Vec::new();
            let mut res2: Vec<f64> = Vec::new();

            let number_of_runs = 1_0000;
            for vector_size in (100..=10000).step_by(100) {
                println!("running vector size {}", vector_size);
                let mut total_time: f64 = 0.0;
                let mut total_time1: f64 = 0.0;
                for _ in 0..number_of_runs {
                    let mut keys = Vec::new();
                    let mut values = Vec::new();
                    let mut map : HashMap<u64,f64> = HashMap::new();

                    for _ in 0..vector_size {
                        keys.push(rand::thread_rng().gen_range(1, 18446744073709551615));
                        values.push(rand::thread_rng().gen_range(1.0, 65536.0));
                    }
                
                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();            
                            
                    hashmap_insert(&keys, &values, &mut map);
                    
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();
                
                    let elapsed = ((end_time - start_time) as f64) / 1_000_000.0;
                    total_time += elapsed;

                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();            
                            
                    hashmap_delete(&keys, &mut map);
                    
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();
                
                    let elapsed = ((end_time - start_time) as f64) / 1_000_000.0;
                    total_time1 += elapsed;
                }

                res1.push(total_time);
                res2.push(total_time1);
            }

            for i in 0..res1.len() {
                print!("{}, ", res1[i] / (number_of_runs as f64));
                std::io::stdout().flush().unwrap();
            }

            println!("\n---");

            for i in 0..res2.len() {
                print!("{}, ", res2[i] / (number_of_runs as f64));
                std::io::stdout().flush().unwrap();
            }
        },

        "bintree" => {
            let mut res1: Vec<f64> = Vec::new();
            let mut res2: Vec<f64> = Vec::new();

            let number_of_runs = 10_000;
            for vector_size in (100..=10000).step_by(100) {
                println!("running vector size {}", vector_size);
                let mut total_time: f64 = 0.0;
                let mut total_time1: f64 = 0.0;
                for _ in 0..number_of_runs {
                    let mut keys = Vec::new();
                    let mut values = Vec::new();
                    let mut map : BTreeMap<u64,f64> = BTreeMap::new();

                    for _ in 0..vector_size {
                        keys.push(rand::thread_rng().gen_range(1, 18446744073709551615));
                        values.push(rand::thread_rng().gen_range(1.0, 65536.0));
                    }
                
                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();            
                            
                    btree_insert(&keys, &values, &mut map);
                    
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();
                
                    let elapsed = ((end_time - start_time) as f64) / 1_000_000.0;
                    total_time += elapsed;

                    let start_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();            
                            
                    btree_delete(&keys, &mut map);
                    
                    let end_time = SystemTime::now().duration_since(UNIX_EPOCH)
                        .expect("Time error").as_micros();
                
                    let elapsed = ((end_time - start_time) as f64) / 1_000_000.0;
                    total_time1 += elapsed;
                }

                res1.push(total_time);
                res2.push(total_time1);
            }

            for i in 0..res1.len() {
                print!("{}, ", res1[i] / (number_of_runs as f64));
                std::io::stdout().flush().unwrap();
            }

            println!("\n---");

            for i in 0..res2.len() {
                print!("{}, ", res2[i] / (number_of_runs as f64));
                std::io::stdout().flush().unwrap();
            }
        },

        _ => {
            print_usage_and_quit(1);
        }
    };
}
