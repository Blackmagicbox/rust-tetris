#[allow(dead_code)]
use crate::engine::Engine;
use crate::interface::Interface;

mod engine;
mod interface;

fn main() {
    let engine =  Engine::new();
    Interface::run(engine);
    println!("Let's play Tetris");
}
