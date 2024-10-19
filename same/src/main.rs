use std::fs;
use std::env;
use std::process::exit;

fn check_exists(path:String) -> Vec<u8> {
    match fs::exists(path.clone()) {
        Ok(is_viable) => {
            if is_viable {
                return fs::read(path.clone()).expect("Error loading file.");
            } else {
                eprintln!("File {} does not exist.",path);
                exit(0);
            }
        },

        Err(_) => {
            eprintln!("File {}'s existance can be neither confirmed nor denied.",path);
            exit(0);
        }
    }
}

fn main() {
    let clargs:Vec<String> = env::args().collect();

    if clargs.len() < 3 {
        eprintln!("Not enough arguments specified.");
        exit(0);
    }

    let data_a = check_exists(clargs[1].clone());
    let data_b = check_exists(clargs[2].clone());

    if data_a == data_b {
        println!("equal");
        exit(1);
    } else {
        println!("not equal");
        exit(0);
    }
}
