#[allow(dead_code)]
#[allow(unused_imports)]

mod net;
mod gamestate;
mod map;
mod client_gameloop;
mod player;
mod class;
mod render;
mod control;

fn main() {
    client_gameloop::gameloop();
}

/*
          ^(;,;)^
you have gazed upon main.rs
now you will surely perish.

*/
