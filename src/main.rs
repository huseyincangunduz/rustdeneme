use std::f32::consts::E;
use std::io::{self, Read, Write};
use std::net::TcpListener;
#[path = "pk/participant.rs"]
pub mod participant;

use participant::*;
use uuid::Uuid;

fn main() {
    // TESTO: How to create a list
    let mut p_list: Vec<participant::Participant> = Vec::new();
    p_list.push(participant::Participant {
        client_id: Uuid::new_v4(),
        instance_name: String::from("Tetakent UBS Users"),
        subject_name: String::from("User Created"),
        subject_type: SubjectType::EVENT,
    });
    p_list.push(participant::Participant {
        client_id: Uuid::new_v4(),
        instance_name: String::from("Tetakent UBS Users"),
        subject_name: String::from("GetUser"),
        subject_type: SubjectType::REQUEST,
    });
    for ele in p_list {
        println!("ENGINE5: \"{}\" {} from \"{}\" has been registered, {}", ele.subject_name, ele.subject_type , ele.instance_name, ele.client_id)
    }

    // TESTO: How to construct a struct instance?
    
    // let mut p1:participant::Participant = participant::Participant {
    //     instance_name: String::from("UBS Users Tetakent"),
    //     subject_name: String::from("User Created"),
    //     subject_type: SubjectType::EVENT
    // };
    // println!("ENGINE5: {} from {} has been registered", p1.subject_name, p1.instance_name)

    // TESTO: TCP Server + Response with Kyle Broflovski Gif and JSON
    
    // let listener = TcpListener::bind("127.0.0.1:13131").unwrap();
    // let mut i = 0;
    // for stream in listener.incoming() {
    //     i += 1;
    //     let mut stream_unwrap = stream.unwrap();
    //     stream_unwrap
    //         // .write("HTTP/1.1 200 OK\r\n\r\n<html><body><img src='https://media4.giphy.com/media/HdUw2UlH1BbMUfGiVH/giphy.gif?cid=6c09b952wr14w7n89llwxmpufm9lg9jr67z8dzhq3ezjna4r&ep=v1_internal_gif_by_id&rid=giphy.gif&ct=s'></img></body></html>".as_bytes())
    //         // .write("HTTP/1.1 200 OK\r\n\r\n{\"e5\": {\"status\": \"HEALTY\"}}".as_bytes())
    //         .write(i.to_string().as_bytes())
    //         .expect("31");
    //     let clientRead = &mut String::new();
    //     // let mut buf =
    //     stream_unwrap.read_to_string(clientRead).expect("31");
    //     println!("Bağlanan kişi şunu girdi: {}", clientRead);
    //     // String::from_utf8(clientRead.);

    //     println!("Connection established!");
    // }

    // TESTO: 1
    
    // let mut x = "sdsd";
    // println!("The value of x is: {x}");
    // x = "sdfsdf";
    // println!("The value of x is: {x}");
    // println!("Sayıyı gir!");
    // let mut guess = String::new();
    // io::stdin().read_line(&mut guess).expect("Kör oldum amk okuyamıyorum");
    // println!("Seçtiğin sayı: {}", guess);
}
