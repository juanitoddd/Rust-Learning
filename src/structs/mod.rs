pub struct Rectangle {
    pub width:u32, 
    pub height:u32
 }
 
 //logic to calculate area of a rectangle
 impl Rectangle {

    // Static Method, does not use &self as first parameter
    pub fn get_pi() -> i32 {
        2
    }

    pub fn area(&self)->u32 {
       //use the . operator to fetch the value of a field via the self keyword
       self.width * self.height
    }
 }