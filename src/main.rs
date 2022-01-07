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
    let mut server = true;
    for argument in std::env::args() {
        if argument.as_str() == "noserver" {
            server = false;
        }
    }
    let client = std::thread::spawn(move || {
        client_gameloop::gameloop()
    });
    if server {
        server_gameloop::gameloop();
    }
    client.join().unwrap().unwrap();
}

/*
          ^(;,;)^
you have gazed upon main.rs
now you will surely perish.

*/
