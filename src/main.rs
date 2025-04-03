use std::io::{self, Write};
use std::net::TcpListener;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:13131").unwrap();

    for stream in listener.incoming() {
        let mut stream_unwrap = stream.unwrap();
        stream_unwrap
            .write("HTTP/1.1 200 OK\r\n\r\n<html><body><img src='https://media4.giphy.com/media/HdUw2UlH1BbMUfGiVH/giphy.gif?cid=6c09b952wr14w7n89llwxmpufm9lg9jr67z8dzhq3ezjna4r&ep=v1_internal_gif_by_id&rid=giphy.gif&ct=s'></img></body></html>".as_bytes())
            .expect("31");
        println!("Connection established!");
    }
    // let mut x = "sdsd";
    // println!("The value of x is: {x}");
    // x = "sdfsdf";
    // println!("The value of x is: {x}");
    // println!("Sayıyı gir!");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Kör oldum amk okuyamıyorum");
    // println!("Seçtiğin sayı: {}", guess);
}
