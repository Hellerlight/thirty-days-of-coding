use std::io;
use std::vec;
use std::io::{Read, stdin};

// Day one
// Table drawer
fn main() {
    println!("Hello, world!");
    let mut table_size_x:i32 = 15;
    let mut table_size_y:i32 = 5;
    drawtable(table_size_x, table_size_y)
}
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

