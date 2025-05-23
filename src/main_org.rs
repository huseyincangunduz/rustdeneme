#[path = "pk/participant.rs"]
pub mod participant;

use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

use std::env;
use std::error::Error;

use std::f32::consts::E;
use std::io::{self, Read, Write};
use std::sync::{Arc, Mutex, mpsc};

use participant::*;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {

    let addr = "127.0.0.1:13131";

    let listener = TcpListener::bind(&addr).await?;
    println!("Listening on: {}", addr);

    loop {
        // Asynchronously wait for an inbound socket.
        let (tx, rx) = mpsc::channel::<participant::Participant>();
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            // let tx = tx.clone();

            let mut buf = vec![0; 1024];
            tx.send(participant::Participant {
                client_id: Uuid::new_v4(),
                instance_name: String::from("31 çek"),
                subject_name: String::from("Subject name"),
                subject_type: SubjectType::EVENT,
            })
            .unwrap();
            loop {
                let n = socket
                    .read(&mut buf)
                    .await
                    .expect("failed to read data from socket");

                if n == 0 {
                    return;
                }

                socket
                    .write_all(&buf[0..n])
                    .await
                    .expect("failed to write data to socket");
            }
        });
        // let a = midone.clone();
        // print!("{}", a.subject_name);
        let a = rx.recv().unwrap().subject_name.to_string();
        print!("{}",a);
    }
}
// async fn main() {

//     TESTO: TCP Ping Pong
//     let listener = TcpListener::bind("127.0.0.1:13131").unwrap();
//     for stream_wrap in listener.incoming() {
//         let mut stream = stream_wrap.unwrap();

//         let mut client_read = String::new();
//         while !client_read.as_str().ends_with("tmm") {
//             stream.read_to_string(&mut client_read).expect("Hata");
//             let mut outStr = ["Asıl sen", &mut client_read].join(" ");//String::new("Asıl sen" + client_read);

//             stream.write(outStr.as_bytes()).expect("Hata");
//             let msg2 = String::from("Müge anlının amk");
//             stream.write(msg2.as_bytes()).expect("Hata");
//             client_read = String::new();
//         }

//     }

//     TESTO: How to create a list
//     let mut p_list: Vec<participant::Participant> = Vec::new();
//     p_list.push(participant::Participant {
//         client_id: Uuid::new_v4(),
//         instance_name: String::from("Tetakent UBS Users"),
//         subject_name: String::from("User Created"),
//         subject_type: SubjectType::EVENT,
//     });
//     p_list.push(participant::Participant {
//         client_id: Uuid::new_v4(),
//         instance_name: String::from("Tetakent UBS Users"),
//         subject_name: String::from("GetUser"),
//         subject_type: SubjectType::REQUEST,
//     });
//     for ele in p_list {
//         println!("ENGINE5: \"{}\" {} from \"{}\" has been registered, {}", ele.subject_name, ele.subject_type , ele.instance_name, ele.client_id)
//     }

//     TESTO: How to construct a struct instance?

//     let mut p1:participant::Participant = participant::Participant {
//         instance_name: String::from("UBS Users Tetakent"),
//         subject_name: String::from("User Created"),
//         subject_type: SubjectType::EVENT
//     };
//     println!("ENGINE5: {} from {} has been registered", p1.subject_name, p1.instance_name)

//     TESTO: TCP Server + Response with Kyle Broflovski Gif and JSON

//     let listener = TcpListener::bind("127.0.0.1:13131").unwrap();
//     let mut i = 0;
//     for stream in listener.incoming() {
//         i += 1;
//         let mut stream_unwrap = stream.unwrap();
//         stream_unwrap
//             // .write("HTTP/1.1 200 OK\r\n\r\n<html><body><img src='https://media4.giphy.com/media/HdUw2UlH1BbMUfGiVH/giphy.gif?cid=6c09b952wr14w7n89llwxmpufm9lg9jr67z8dzhq3ezjna4r&ep=v1_internal_gif_by_id&rid=giphy.gif&ct=s'></img></body></html>".as_bytes())
//             // .write("HTTP/1.1 200 OK\r\n\r\n{\"e5\": {\"status\": \"HEALTY\"}}".as_bytes())
//             .write(i.to_string().as_bytes())
//             .expect("31");
//         let clientRead = &mut String::new();
//         // let mut buf =
//         stream_unwrap.read_to_string(clientRead).expect("31");
//         println!("Bağlanan kişi şunu girdi: {}", clientRead);
//         // String::from_utf8(clientRead.);

//         println!("Connection established!");
//     }

//     TESTO: 1

//     let mut x = "sdsd";
//     println!("The value of x is: {x}");
//     x = "sdfsdf";
//     println!("The value of x is: {x}");
//     println!("Sayıyı gir!");
//     let mut guess = String::new();
//     io::stdin().read_line(&mut guess).expect("Kör oldum amk okuyamıyorum");
//     println!("Seçtiğin sayı: {}", guess);
// }
