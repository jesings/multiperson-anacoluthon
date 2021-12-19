#[allow(dead_code)]
#[allow(unused_imports)]

mod net;
mod gamestate;
mod map;
mod client_gameloop;

fn main() {
    client_gameloop::gameloop();
}
