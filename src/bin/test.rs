
#![feature(phase)]

extern crate foundation;
#[phase(plugin)]
extern crate objcruntime;
extern crate objcruntime;

use foundation::{
    NSString,
    NSMutableArray,
    Array,
};

fn main() {
    // let pool = m![m![cls!(NSAutoreleasePool) alloc] init];
    let nsstr = NSString::from_str("hello world");
    println!("len: {}, content: {}", nsstr.len(), &nsstr);
    let mut nsa = NSMutableArray::<NSString>::new();
    nsa.add(nsstr);
    let f: NSString = nsa.first().unwrap();
    let f2: NSString = nsa.first().unwrap();
    println!("first: {}", f);
    println!("first2: {}", f2);
    // m![pool drain];
}