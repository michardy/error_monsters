
use std::collections::HashMap;

struct BigType {
    characters:[[[u8; 11]; 11]; 5],
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
                ]
            ],
            cur_row: 0u8,
            code: err
        }
    }
    pub fn print_row(&mut self) {
        if self.cur_row < 11 {
            for i in 0..3 {
                //println!("{}", (self.code)[i])
                //self.characters[(self.code)[i] as usize];
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
    let hum_code = "404";
    let code = [4u8, 0u8, 4u8];

    let desc: HashMap<[u8; 3], String> = [([4u8, 0u8, 0u8], "Bad Request".to_string()),
	([4u8, 0u8, 1u8], "Unauthorized".to_string()),
        ([4u8, 0u8, 2u8], "Payment Required".to_string()),
        ([4u8, 0u8, 3u8], "Forbidden".to_string()),
        ([4u8, 0u8, 4u8], "Not Found".to_string()),
        ([4u8, 0u8, 5u8], "Method Not Allowed".to_string()),
        ([4u8, 0u8, 6u8], "Not Acceptable".to_string()),
        ([4u8, 0u8, 7u8], "Proxy Authentication Required".to_string()),
        ([4u8, 0u8, 8u8], "Request Timeout".to_string()),
        ([4u8, 0u8, 9u8], "Conflict".to_string()),//fighting siblings
        ([4u8, 1u8, 0u8], "Gone".to_string()),//"Move along! Move along!"
        ([4u8, 1u8, 1u8], "Length Required".to_string()),
        ([4u8, 1u8, 2u8], "Precondition Failed".to_string()),//Hat required sign & monster without hat
        ([4u8, 1u8, 3u8], "Payload Too Large".to_string()), //show overloaded truck
        ([4u8, 1u8, 4u8], "URI Too Long".to_string()), //long string or receipt & confused monster
        ([4u8, 1u8, 5u8], "Unsupported Media Type".to_string()), //confused monster in front of thing with lots of dohickies
        ([5u8, 0u8, 0u8], "Internal Server Error".to_string()),
        ([5u8, 0u8, 1u8], "Not Implimented".to_string()),
        ([5u8, 0u8, 2u8], "Bad Gateway".to_string()),
        ([5u8, 0u8, 3u8], "Service Unavailable".to_string())]
            .iter().cloned().collect();

    //start writing a page
    println!("HTTP/1.1 {} {}\r\n", hum_code, desc.get(&code).unwrap());
    println!("Content-Type:text/html\r\n");
    println!("<!DOCTYPE html><html><head><title>");
        if desc.contains_key(&code){
            println!("{}, {}", hum_code, desc.get(&code).unwrap());
        } else {
            println!("{}", hum_code);
        }
    println!("</title></head><body><pre>");
    let mut error = BigType::new(code);
    for i in 0..11 {
        error.print_row();
        print!("\r\n");
    }
    println!("</pre></body></html>")
}
