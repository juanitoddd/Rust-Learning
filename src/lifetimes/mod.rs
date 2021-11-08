fn f(s: &str) -> &str {
    s
}

// When a function recieves multiple references
// each reference is given its own lifetime 

// fn f2(s: &str, t: &str) -> &str {
//     if s.len() > 5 { s } else { t }
// }

// The way to achieve this is to give both input parameters the same lifetime annotation.
// It’s how we tell the compiler that as long as both of these input parameters are valid, 
// so is the returned value.

fn f3<'a>(s: &'a str, t: &'a str) -> &'a str {
    if s.len() > 5 { s } else { t }
}

pub fn run() {
    let x  =f3("hello", "world");
    println!("The value of 'x' is {}.", x);
}