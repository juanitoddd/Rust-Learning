// mod closures;
// mod options;
// use crate::options::{drink, give_adult};

use learn::options::{drink, give_adult};

fn update(mut arr:[i32;3]){
    for i in 0..3 {
       arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
 }

 fn updateRef(arr:&mut [i32;3]){
    for i in 0..3 {
       arr[i] = 0;
    }
    println!("Inside update {:?}",arr);
 }
fn main() {
    // closures::run();
    println!("Hello, world!");

    let _content_string = String::from("TutorialsPoint");

    let water  = Some("water");
    let lemonade = Some("lemonade");
    let void  = None;

    give_adult(water);
    give_adult(lemonade);
    give_adult(void);

    let coffee = Some("coffee");
    // let nothing = None;

    drink(coffee);
    //drink(nothing);

    let a : i32  = rand::random();
    println!("{}", a);

    let mut z = String::new();
    z.push_str("hello");
    z.push_str(" ");
    z.push_str("Juan");
    println!("{}",z);

    let arr = [10,20,30];
    update(arr);
    print!("Inside main {:?}",arr);

    let mut arr = [10,20,30];
    updateRef(&mut arr);
    print!("Inside main {:?}",arr);
}
