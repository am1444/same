use std::fs;
use std::env;
use std::process::exit;
use std::path::Path;

fn check_exists(path:String) -> Vec<u8> {
    let path_o = Path::new(&path);
    match fs::exists(path.clone()) {
        Ok(is_viable) => {
            if is_viable && path_o.metadata().unwrap().is_file(){
                return fs::read(path.clone()).expect("Error loading file.");
            } else if is_viable {
                eprintln!("{} is a directory.",path);
                exit(3);
            } else {
                eprintln!("File {} does not exist.",path);
                exit(3);
            }
        },

        Err(_) => {
            eprintln!("File {}'s existance can be neither confirmed nor denied.",path);
            exit(3);
        }
    }
}

fn main() {
    let clargs:Vec<String> = env::args().collect();

    if clargs.len() < 3 {
        if clargs[1]=="-v".to_string() {
            println!("same 1.1");
            exit(3);
        }
        eprintln!("Not enough arguments specified.");
        exit(4);
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
