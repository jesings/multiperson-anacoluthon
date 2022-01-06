#[allow(dead_code)]
#[allow(unused_imports)]

mod net;
mod gamestate;
mod map;
mod client_gameloop;
mod client_netloop;
mod server_gameloop;
mod player;
mod class;
mod render;

fn main() {
    std::thread::spawn(move || {
        client_gameloop::gameloop();
    });
    server_gameloop::gameloop();
}

/*
          ^(;,;)^
you have gazed upon main.rs
now you will surely perish.

*/
