use vdf::Reader;
use vdf::Value;
use std::fs;
use std::io::stdin;
use std::str::FromStr;
fn main() {
    println!("Hello, world!");
    let a = fs::read_to_string(r"D:\rust\vdf\target\debug\test.txt").unwrap();
    let mut b = Reader::new(a);
    let mut c = b.read_kv();
    /*
    let d = &c["items_game"]["game_info"]["first_valid_class"];
    match d {
        vdf::Value::Simple(y) => {
            println!("first_valid_class:{}",y);
        },
        _ =>{
            panic!("none")
        },
    }
    //let e = &d["game_info"];
    //println!("{}",c["items_game"]"game_info"["first_valid_class"]);
    */
    print!("{}",c);
    
}
