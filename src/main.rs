use std::io;
use std::vec;
use std::io::{Read, stdin};

// Day one
// Table drawer
const initial_table_size_x: i32 = 15;
const initial_table_size_y: i32 = 5;
fn drawtable_bot_top(table_x:i32) {
    print!(" ");
    for i in (0..table_x) {
        print!("-");
    }
}
fn drawtable(table_x: i32, table_y:i32) {
    println!("Tablesize x:{}, y:{}", table_x, table_y);
    drawtable_bot_top(table_x);
    print!("\n");
    for i in (0..table_y) {
        print!("|");
        for i in (0..table_x) {
            print!(" ");
        }
        print!("|");
        print!("\n");
    }
    drawtable_bot_top(table_x);
}
// Day Two
fn addbar(name:&str, value:i32) -> String {
    let mut resultstring:String = String::new();
    resultstring.push_str(name);
    for i in (0..value) {
        resultstring.push_str("|");
    }
    return resultstring;
}

fn draw_bars(bars:Vec<String>) {
    for i in bars.iter() { println!("{}", i); }
}

fn main() {
    println!("Hello, world!");

    let mut table_databars:Vec<String> = Vec::new();
    table_databars.push(addbar("ptat", 30));
    table_databars.push(addbar("Bob", 15));
    table_databars.push(addbar("Timothy", 9));
    // drawtable(initial_table_size_x, initial_table_size_y);
    draw_bars(table_databars);

}
