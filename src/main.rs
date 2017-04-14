extern crate error_monsters_404;

use std::env;
use std::collections::HashMap;
//use error_monsters_404;

struct BigType {
    characters:[[[u8; 11]; 11]; 6],
    cur_row: u8,
    code: [u8; 3],
}

impl BigType {
    pub fn new(err:[u8; 3]) -> BigType {
        BigType {
            characters:[
	        [
                    [32, 32, 32, 35, 35, 35, 35, 35, 32, 32, 32],
                    [32, 32, 35, 35, 32, 32, 32, 35, 35, 32, 32],
                    [32, 35, 35, 32, 32, 32, 32, 32, 35, 35, 32],
                    [35, 35, 32, 32, 32, 32, 32, 32, 32, 35, 35],
                    [35, 32, 32, 32, 32, 35, 32, 32, 32, 32, 35],
                    [35, 32, 32, 32, 35, 35, 35, 32, 32, 32, 35],
                    [35, 32, 32, 32, 32, 35, 32, 32, 32, 32, 35],
                    [35, 35, 32, 32, 32, 32, 32, 32, 32, 35, 35],
                    [32, 35, 35, 32, 32, 32, 32, 32, 35, 35, 32],
                    [32, 32, 35, 35, 32, 32, 32, 35, 35, 32, 32],
                    [32, 32, 32, 35, 35, 35, 35, 35, 32, 32, 32]
                ],
                [
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32]
                ],
                [
                    [32, 32, 32, 32, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 35, 35, 35, 35, 35, 35, 35, 32, 32],
                    [32, 35, 35, 35, 32, 32, 32, 35, 35, 35, 32],
                    [35, 35, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [35, 32, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 32, 35, 35, 35, 32],
                    [32, 32, 32, 32, 32, 35, 35, 35, 32, 32, 32],
                    [32, 32, 32, 35, 35, 35, 32, 32, 32, 32, 32],
                    [32, 35, 35, 35, 32, 32, 32, 32, 32, 32, 32],
                    [35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35],
                    [35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35]
                ],
                [
                    [32, 32, 35, 35, 35, 35, 35, 35, 35, 32, 32],
                    [32, 35, 35, 35, 32, 32, 32, 35, 35, 35, 32],
                    [35, 35, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 32, 35, 35, 35, 32],
                    [32, 32, 32, 32, 32, 32, 35, 35, 35, 32, 32],
                    [32, 32, 32, 35, 35, 35, 32, 32, 32, 32, 32],
                    [32, 32, 32, 32, 32, 32, 35, 35, 35, 32, 32],
                    [32, 32, 32, 32, 32, 32, 32, 35, 35, 35, 32],
                    [35, 35, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [32, 35, 35, 35, 32, 32, 32, 35, 35, 35, 32],
                    [32, 32, 35, 35, 35, 35, 35, 35, 35, 32, 32]
                ],
                [
                    [32, 32, 32, 32, 32, 32, 32, 35, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 35, 35, 35, 35, 35],
                    [32, 32, 32, 32, 32, 35, 35, 35, 35, 35, 35],
                    [32, 32, 32, 32, 35, 35, 35, 32, 35, 35, 35],
                    [32, 32, 32, 35, 35, 35, 32, 32, 35, 35, 35],
                    [32, 32, 35, 35, 35, 32, 32, 32, 35, 35, 35],
                    [32, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35],
                    [35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 32, 32, 35, 35, 35]
                ],
                [
                    [35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35],
                    [35, 35, 35, 35, 35, 35, 35, 35, 35, 35, 35],
                    [35, 35, 35, 32, 32, 32, 32, 32, 32, 32, 32],
                    [35, 35, 35, 35, 35, 35, 35, 32, 32, 32, 32],
                    [32, 32, 32, 32, 32, 35, 35, 35, 35, 32, 32],
                    [32, 32, 32, 32, 32, 32, 32, 35, 35, 35, 32],
                    [32, 32, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 32, 32, 35, 35, 35],
                    [32, 32, 32, 32, 32, 32, 32, 35, 35, 35, 32],
                    [32, 32, 32, 32, 32, 35, 35, 35, 35, 32, 32],
                    [35, 35, 35, 35, 35, 35, 32, 32, 32, 32, 32],
                ]
            ],
            cur_row: 0u8,
            code: err
        }
    }
    pub fn print_row(&mut self) {
        if self.cur_row < 11 {
            for i in 0..3 {
                print!("{}", String::from_utf8_lossy(
                    &((self.characters[(self.code)[i] as usize])[self.cur_row as usize])
                ));
                print!(" ");
            }
        }
        self.cur_row += 1;
    }
}

fn main() {
    let key = "QUERY_STRING";
    let hum_code = match env::var(key) {
        Ok(val) => val,
        Err(e) => "500".to_string(),
    };
    let ecbs = hum_code.as_bytes();
    let mut code = [0u8, 0u8, 0u8];
    for i in 0..3 {
        code[i] = ecbs[i] - 48;
    }

    let desc: HashMap<[u8; 3], String> = [([4u8, 0u8, 0u8], "Bad Request".to_string()),
	([4u8, 0u8, 1u8], "Unauthorized".to_string()),
        ([4u8, 0u8, 2u8], "Payment Required".to_string()), //little monster turned away from amusement park ride or monster with bribes jar
        ([4u8, 0u8, 3u8], "Forbidden".to_string()), //Do not enter sign
        ([4u8, 0u8, 4u8], "Not Found".to_string()), //monster, map, & desert
        ([4u8, 0u8, 5u8], "Method Not Allowed".to_string()),
        ([4u8, 0u8, 6u8], "Not Acceptable".to_string()),
        ([4u8, 0u8, 7u8], "Proxy Authentication Required".to_string()),
        ([4u8, 0u8, 8u8], "Request Timeout".to_string()), //monster & stopwatch
        ([4u8, 0u8, 9u8], "Conflict".to_string()), //fighting siblings
        ([4u8, 1u8, 0u8], "Gone".to_string()), //"Move along! Move along!"
        ([4u8, 1u8, 1u8], "Length Required".to_string()),
        ([4u8, 1u8, 2u8], "Precondition Failed".to_string()), //Hat required sign & monster without hat
        ([4u8, 1u8, 3u8], "Payload Too Large".to_string()), //show overloaded truck
        ([4u8, 1u8, 4u8], "URI Too Long".to_string()), //long string or receipt & confused monster
        ([4u8, 1u8, 5u8], "Unsupported Media Type".to_string()), //confused monster in front of thing with lots of dohickies
        ([5u8, 0u8, 0u8], "Internal Server Error".to_string()), //monster, wrench, & leaking pipes
        ([5u8, 0u8, 1u8], "Not Implimented".to_string()), //incomplete bridge or walkway
        ([5u8, 0u8, 2u8], "Bad Gateway".to_string()), //failing arch, monster, & tools.  "we are probably down for maintanence(R)"
        ([5u8, 0u8, 3u8], "Service Unavailable".to_string())] //closed sign
            .iter().cloned().collect();

    //start writing a page
    println!("Content-Type:text/html;charset=UTF-8");
    println!("Status: {} {}\n\n", hum_code, desc.get(&code).unwrap());
    println!("<!DOCTYPE html><html><head><title>");
    println!("{}, {}", hum_code, desc.get(&code).unwrap());
    println!("</title></head><body><pre>");
    let mut error = BigType::new(code);
    let mut map: error_monsters_404::Map = match hum_code.as_ref() {
	"404" => error_monsters_404::Map::new(),
        _ => error_monsters_404::Map::new()
    };
    for i in 0..11 {
        map.print_lside();
        error.print_row();
        map.print_rside();
        print!("\r\n");
    }
    map.print_end();
    println!("</pre></body></html>")
}
