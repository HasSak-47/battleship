mod game;
use std::{
    thread,
    sync::{Arc, Mutex},
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*}
};

use rand::random;
#[allow(non_upper_case_globals)]
static rand : fn() -> usize = random::<usize>;

static mut PLAYER_ID_MUTEX : Mutex<usize> = Mutex::new(0);
static mut PLAYER_ID       : usize = 0;

const TURN_MUTEX : Mutex<usize> = Mutex::new(1);
static mut  TURN : usize = 0b00;

fn id_manager() -> String {
    unsafe{
        let binding = &PLAYER_ID_MUTEX;
        let _lock = binding.lock().unwrap();
        let id = PLAYER_ID;
        PLAYER_ID += 1;

        id.to_string()
    }
}

const ATTACK_LOCK : Mutex<usize> = Mutex::new(0);
const UPDATE_LOCK : Mutex<usize> = Mutex::new(1);

const BOARD_LOCK  : Mutex<usize> = Mutex::new(2);
static mut BOARDS: [game::Board; 2] = [
    game::Board::default(),
    game::Board::default(),
];

fn attack_manager(http_request: &Vec<String>) -> String{
    let lock_bind = ATTACK_LOCK;
    let res = lock_bind.lock();
    let id = usize::from_str_radix(&http_request[1][4..], 10).unwrap();
    let (x, y) = {
        let split : Vec<&str> = http_request[2][5..].split("x").into_iter().collect();
        (usize::from_str_radix(&split[0], 10).unwrap(),
         usize::from_str_radix(&split[1], 10).unwrap())
    };

    println!("{id} ({x},{y})");
    unsafe{
        use game::Cell::*;
        match &BOARDS[id].0[x][y] {
            Empty => {"M"},
            Ship  => {"H"},
                _ => {"N"}
        }.to_string()
    }

}

async fn wait_until_turn(turn : usize){
}

fn update_manager(http_request: &Vec<String>) -> String{
    todo!();
}

fn step() {
    let bind = TURN_MUTEX;
    unsafe{
        TURN = (TURN + 1) % 4;
    }
}

fn connection_manager(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request = buf_reader.lines().next().unwrap().unwrap();

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();


    println!("requests: {:#?}", http_request);

    let (status_line, contents) = match http_request[0].as_str(){
        "GET /ID HTTP/1.1" =>
            ("HTTP/1.1 200 OK", id_manager()),
        "GET /HIT HTTP/1.1" =>
            ("HTTP/1.1 200 OK", attack_manager(&http_request)),
        "GET /UPD HTTP/1.1" =>
            ("HTTP/1.1 200 OK", update_manager(&http_request)),
        _ =>
            ("HTTP/1.1 404 OK", String::new()),
    };

    let lenght   = contents.len();
    let response = format!("{status_line}\r\nContent-lenght: {lenght}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();

    println!("connection dropped");
}

fn init_board(board: &mut game::Board){
    let ships = vec![
        (rand() % 6, rand() % 6, rand() % 1),
        (rand() % 6, rand() % 6, rand() % 1),
        (rand() % 6, rand() % 6, rand() % 1),
    ];

    for j in 0..3{
        let mut x  = ships[j].0;
        let mut y  = ships[j].1;
        for _ in 0..(j + 2){
            board.0[x][y] = game::Cell::Ship;
            if ships[0].2 == 1{
                x += 1;
            }
            else{
                y += 1;
            }
        }
    }
}

pub fn main() {
    unsafe{
        init_board(&mut BOARDS[0]);
        init_board(&mut BOARDS[1]);
    }
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming(){
        thread::spawn(|| connection_manager(stream.unwrap()));
    }
}
