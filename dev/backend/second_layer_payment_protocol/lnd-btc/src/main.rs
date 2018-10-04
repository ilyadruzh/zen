extern crate bitcoin;
extern crate bitcoin_bech32;
extern crate bitcoin_rpc_json;
extern crate serde;
extern crate tokio;

use serde::ser::{Serialize, Serializer};
use std::env;
use std::io::prelude::*;
use std::io::{self, ErrorKind, Read, Write};
use std::net::{AddrParseError, IpAddr, Ipv4Addr, SocketAddr, TcpStream};
use tokio::net::TcpListener;
use tokio::prelude::*;

mod lib;

// программа запускает демона, который ждёт команду, которая что-то сделает
// нужно указывать сеть: testnet или mainnet
// нужно указать собственный IP:PORT
// нужно получить из БД адреса хабов и своих партнеров
fn main() {
    env::set_var("RUST_LOG", "info");

    /// Типы команд
    /// - `-start` - запуск демона с предыдущими настройками + настройки
    /// - `-create_channel` - создать канал + параметры
    /// - `-close_channel` - закрыть канал + параметры
    /// - `-make_node` - стать узлом сети + параметры
    /// - `-exit` - выйти из программы

    // сообщаем пользователю типы команд
    println!(
        "Write command:: 
        \"-start\" --- for creating channel;
        \"-create_channel\" --- for creating channel;
        \"-close_channel\" --- for creating channel;
        \"-make_node\" --- for creating channel;
        \"-exit\" --- for exit\n"
    );

    // записываем команду
    let mut command = String::new();

    match io::stdin().read_line(&mut command) {
        Ok(n) => {
            match command.as_ref() {
                // запустить
                "-start\n" => {
                    println!("Ligthning Network started with params: {}", "{ some params }");
                    lib::engine::client::client_handler();
                },
                "-create_channel\n" => {
                    println!("Channel with {} created", "{ partners name }");

                    // запуск client_handler

                    // let server = listener
                    //     .incoming()
                    //     .for_each(|socket| {
                    //         println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

                    //         let connection = io::write_all(socket, "hello world\n").then(|res| {
                    //             println!("wrote message; success={:?}", res.is_ok());
                    //             Ok(())
                    //         });

                    //         tokio::spawn(connection);

                    //         Ok(())
                    //     })
                    //     .map_err(|err| {
                    //         println!("accept error = {:?}", err);
                    //     });

                    // println!("server running on localhost:6142");

                    // tokio::run(server);
                }
                "-close_channel\n" => {
                    println!("Channel {} closed", "{ partner name }");
                }
                "-make_node\n" => {
                    println!("Lightning node created!");
                }
                "-exit\n" => {
                    println!("Exit from SLPP");
                }
                _ => { println!("Неверная команда"); }
            }
        }

        Err(error) => println!("error: {}", error),
    }

    // слушаем свой адрес по порту 17666
    // let addr = "127.0.0.1:17666".parse().unwrap();
    // let listener = TcpListener::bind(&addr).unwrap();

    // нужно где-то вызвать метод client_handler
    // Socket->async_connect(Endpoint, std::bind( client_handler, Socket, std::placeholders::_1) );
}
