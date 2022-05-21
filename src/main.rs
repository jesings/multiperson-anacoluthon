#[allow(dead_code)]
#[allow(unused_imports)]

mod net;
mod gamestate;
mod map;
mod client_gameloop;
mod client_netloop;
mod server_gameloop;
mod entity;
mod player;
mod class;
mod render;
mod control;

fn main() {
    let mut hasserver = true;
    let mut hasclient = true;
    for argument in std::env::args() {
        match argument.as_str() {
            "noserver" => {hasserver = false;}
            "noclient" => {hasclient = false;}
            _ => {}
        }
    }

    if hasclient {
        if hasserver {
            let server = std::thread::spawn(move || {
                server_gameloop::gameloop()
            });
            client_gameloop::gameloop().unwrap();
            server.join().unwrap();
        } else {
            client_gameloop::gameloop().unwrap();
        }
    } else if hasserver {
        server_gameloop::gameloop();
    } else {
        eprintln!("Wtfrick is the point of you running am2 anyway???");
    }
}

/*
          ^(;,;)^
you have gazed upon main.rs
now you will surely perish.

*/
