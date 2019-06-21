use std::env;
use node_create;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        println!("Not enough arguments");
        return;
    }

    let src = args[1].clone();
    let dest = args[2].clone();
    let fname = src + &String::from("_") + &dest;

    if let Ok(_) = node_create::editor(&fname, &String::from("noexist")) {
        let res = node_create::file_to_dict(&fname).expect("parsing json filed. Try again");
        println!("{:?}", res);
    } else {
        println!("Editor does not exist");
    }

}

