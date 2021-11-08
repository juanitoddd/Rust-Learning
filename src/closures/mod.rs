use crate::closures::entity::Entity;

mod entity;

pub fn  run() {
    let _my_entity = Entity {
        id: 1
    };
    println!("Hello Rust");
}