#[allow(dead_code)]
use crate::engine::Engine;

mod engine;
mod interface;

fn main() {
    let engine =  Engine::new();
    println!("Let's play Tetris");
}
