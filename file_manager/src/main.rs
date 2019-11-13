fn main() {
    read_instruction();
}

fn read_instruction() {
    println!("Enter instruction (Create, Read, Delete)");
    let mut s = get_input();
    parse_instruction(s.trim().to_string());
}

fn parse_instruction(instr: String) {
    match instr.as_ref() {
        "create" => {
            println!("Enter path:");
            create(get_input().trim().to_string());
        },
        "read" => {
            println!("Enter path:");
            read(get_input().trim().to_string());
        },
        "delete" => {
            println!("Enter path:");
            delete(get_input().trim().to_string());
        },
        _ => (),
    }
    read_instruction();
}

fn create(s: String) {
    let res = std::fs::File::create(std::path::Path::new(&s));
    if res.is_err() {
        println!("Err - {}", res.unwrap_err());
    } else {
        let file = res.unwrap();
        println!("Ok");
    }
}

fn delete(s: String) {
    let path = std::path::Path::new(&s);
    let res = std::fs::File::open(path);
    if res.is_err() {
        println!("Err - {}", res.unwrap_err());
    } else {
        let file = res.unwrap();
        std::fs::remove_file(path);
        println!("Ok");
    }
}

fn read(s: String) {
    let path = std::path::Path::new(&s);
    let mut res = std::fs::File::open(path);
    if res.is_err() {
        println!("Err - {}", res.unwrap_err());
    } else {
        let file = res.unwrap();
        let res = std::fs::read_to_string(path);
        if res.is_err() {
            println!("Err - {}", res.unwrap_err());
        } else {
            println!("----------------------");
            println!("{}", res.unwrap());
            println!("----------------------");
        }
    }
}

fn get_input() -> String {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).expect("Error");
    s
}