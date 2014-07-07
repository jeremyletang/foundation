
extern crate foundation;

use foundation::{
    NSString,
    NSMutableArray,
    Array,
};

fn main() {
    let nsstr = NSString::from_str("hello world");
    println!("len: {}, content: {}", nsstr.len(), &nsstr);
    let mut nsa = NSMutableArray::<NSString>::new();
    nsa.add(nsstr);
    let f: NSString = nsa.first().unwrap();
    let f2: NSString = nsa.first().unwrap();
    println!("first: {}", f);
    println!("first2: {}", f2);
}