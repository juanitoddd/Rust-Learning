// use crate::structs::Rectangle;
// mod structs;
// use structs::Rectangle;

use learn::structs::Rectangle;

fn add_one_reference(e: &mut i32) {
   *e += 1;
}

fn add_one_normal(mut a: i32) -> i32 {
   a += 1;
   a
}

fn modify_my_vector(mut w: Vec<i32>) {
   w.push(5);
   println!("modify_my_vector {:?}", w);
}

fn modify_my_reference(w: &mut Vec<i32>) {
   w.push(5);   
}

fn main() {

   // Primitive values (Stored on stack)
   let j = 1;
   let k = add_one_normal(j);
   println!("j: {}", j);
   println!("k: {}", k);

   let mut i = 3;
   add_one_reference(&mut i);
   println!("i: {}", i);

   // Heap-allocated values

   let v = vec![1,2,3,4];
   modify_my_vector(v);
   // v is out of scope from here on


   let mut b = vec![1,2,3,4];
   modify_my_reference(&mut b);
   // b is borrowed to the function
   println!("b {:?}", b);

   let m = Rectangle {
      height: 20,
      width:10
   };

   // println!("Area of rectangle {:?}", m.area());
   println!("Area of rectangle {}", m.area());

   println!("Pi {}", Rectangle::get_pi());
}
