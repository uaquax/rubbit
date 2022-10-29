use std::env;

pub mod init;
pub mod initserv;
pub mod add;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        if args.get(1).expect("No init found") == "init" {
            init::init();

            println!("Rubbit initialized");
        }

        if args.get(1).expect("No initserv found") == "initserv" && args.get(2) != None && args.get(2).unwrap().len() >= 5 {
            initserv::initserv(args.get(2).unwrap());

            println!("Rubbit server initialized");
        }
        
        else if args.get(1).expect("No add found") == "add" && args.get(2).expect("No url found").starts_with("http") {
            add::add(&args[2].to_string(), args.get(3).expect("No password found"));
            println!("{} successfully added to rubbit!", args[2]);
        }
    }

    else {
        panic!("Unknown command...");
    }
}