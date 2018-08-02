extern crate tokio;

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

mod engine;

fn main() {
    let mut command = String::new();

    let addr = "127.0.0.1:17666".parse().unwrap();
    let listener = TcpListener::bind(&addr).unwrap();

    println!("write \"-create_channel\" for creating channel or \"-exit\"\n");

    io::stdin().read_to_string(&mut command);

    // нужно где-то вызвать метод client_handler
    // Socket->async_connect(Endpoint, std::bind( client_handler, Socket, std::placeholders::_1) );

    loop {
        match command.as_ref() {
            "-exit" => break,
            "-create_channel" => {
                let server = listener
                    .incoming()
                    .for_each(|socket| {
                        println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

                        let connection = io::write_all(socket, "hello world\n").then(|res| {
                            println!("wrote message; success={:?}", res.is_ok());
                            Ok(())
                        });

                        tokio::spawn(connection);

                        Ok(())
                    }).map_err(|err| {
                        println!("accept error = {:?}", err);
                    });

                println!("server running on localhost:6142");

                tokio::run(server);
            },
            _ => println!("Пожалуйста, введите команду");
        }
    }
}
