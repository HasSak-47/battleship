mod game;
use std::{
    net::TcpStream,
    io::prelude::*,
};
use rand::random as __random;

#[allow(non_upper_case_globals)]
static random : fn() -> usize = __random::<usize>;

fn request<S>(request: S, fields: Option<Vec<S>>) -> Vec<String>
where
    S: AsRef<str>,
{
    let fields = if let Some(fields) = fields{
        let mut processed = String::new();
        for field in fields{
            processed.push_str(format!("\r\n{}", field.as_ref()).as_str());
        }
        processed
    }
    else{
        String::new()
    };

    let request = format!(
        "GET /{} HTTP/1.1{}",
        request.as_ref(),
        fields,
    );

    match TcpStream::connect("localhost:8080"){
        Ok(k) => manage_request(k, request),
        Err(_) => panic!("panic")
    }
}

fn manage_request(mut stream: TcpStream, request: String) -> Vec<String>{
    stream.write(request.as_bytes()).unwrap();
    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let mut buffer = Vec::<u8>::new();
    stream.read_to_end(&mut buffer).unwrap();
    let response = std::str::from_utf8(&buffer).unwrap();
    let lines : Vec<String> = response
        .lines()
        .into_iter()
        .fold(Vec::new(),
            |mut v, a| {
                v.push(a.to_string());
                v
            });

    lines
}

use game::{EnemyBoard as EBoard, PersonalBoard as PBoard};


fn hit(id: usize, e: &mut EBoard, _: &mut PBoard) -> String{
    // spagqefh
    let (x, y) = e.decide_attack();
    let p = request("HIT", Some(vec![
        &format!("Id: {id}"),
        &format!("HIT: {x}x{y}", ),
    ]));

    String::new()
}


fn upd(id: usize, _: &mut EBoard, p: &mut PBoard) -> String{
    let response = request("UPD", Some(vec![
        &format!("Id: {id}"),
    ]));

    let x = usize::from_str_radix(&response[3], 10).unwrap();
    let y = usize::from_str_radix(&response[4], 10).unwrap();

    p.0[y][x] = match p.0[y][x]{
        game::PersonalCell::Miss => game::PersonalCell::Miss,
        game::PersonalCell::Empty => game::PersonalCell::Miss,
        _ => game::PersonalCell::Hit,
    };

    String::new()
}

fn id() -> (usize, game::PersonalBoard){
    let response = request("ID", None);
    let id_str = &response[4];
    let id = usize::from_str_radix(id_str.as_str(), 10).unwrap();
    let pboard = game::PersonalBoard::from_string(&response[5]);

    (id, pboard)
}

fn update(response: String) -> bool {
    println!("{response}");
    response.as_str() != "ended"
}

pub fn main(){
    let (id, mut pboard) = id(); 
    let mut eboard = game::EnemyBoard::default();

    let funcs = [hit, upd];
    let mut index = id;
    println!("id : {id}");
    while update(funcs[index % 2](id, &mut eboard, &mut pboard)) {
        println!("-----------------------------");
        println!("{eboard}");
        println!("-----------------------------");
        println!("{pboard}");
        println!("-----------------------------");
        index += 1;
    }
}
