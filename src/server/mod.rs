mod game;
use std::{
    thread,
    sync::{Arc, Mutex, Condvar},
    net::{TcpListener, TcpStream},
    io::{BufReader, prelude::*}
};

const PLAYER_ID_MUTEX : Mutex<usize> = Mutex::new(0);
static mut PLAYER_ID       : usize = 0;

fn id_manager() -> String {
    let binding = &PLAYER_ID_MUTEX;
    let _lock = binding.lock().unwrap();
    unsafe {
        let id = PLAYER_ID;
        PLAYER_ID += 1;

        format!("\r\n{}\r\n{}", id, BOARDS[id])
    }
}

// const ATTACK_LOCK : Mutex<usize> = Mutex::new(0);
// const UPDATE_LOCK : Mutex<usize> = Mutex::new(1);

static mut BOARDS: [game::Board; 2] = [
    game::Board::default(),
    game::Board::default(),
];

static mut LAST_ATTACK : (usize, usize) = (0,0); 

fn attack_manager(http_request: &Vec<String>, manager: Manager) -> String{
    let id = usize::from_str_radix(&http_request[1][4..], 10).unwrap();
    let enemy_id = (id + 1) % 2;
    let (x, y) = {
        let split : Vec<&str> = http_request[2][5..].split("x").into_iter().collect();
        (usize::from_str_radix(&split[0], 10).unwrap(),
         usize::from_str_radix(&split[1], 10).unwrap())
    };

    println!("attack request from: {id} ({x},{y})");
    let (lock, condvar) = &*manager;

    let turn_id   = id * 2;
    let mut thread_id = lock.lock().unwrap();
    while turn_id != *thread_id{
        println!("locking attack {id}");
        thread_id = condvar.wait(thread_id).unwrap();
    }
    println!("unlock from: {id}");

    let result = unsafe{
        LAST_ATTACK.0 = x;
        LAST_ATTACK.1 = y;
        use game::Cell::*;
        match &BOARDS[id].0[x][y] {
            Empty => {BOARDS[enemy_id].0[x][y] = Miss; "M"},
            Ship  => {BOARDS[enemy_id].0[x][y] = Hit ; "H"},
                _ => {"N"}
        }.to_string()
    };

    *thread_id = step_turn(*thread_id);
    condvar.notify_all();

    result
}

fn update_manager(http_request: &Vec<String>, manager: Manager) -> String{
    let id = usize::from_str_radix(&http_request[1][4..], 10).unwrap();

    println!("update request from: {id}");
    let (lock, condvar) = &*manager;

    let turn_id   = (id * 2) + 1;
    let mut thread_id = lock.lock().unwrap();
    while turn_id != *thread_id{
        println!("locking update: {id}");
        thread_id = condvar.wait(thread_id).unwrap();
    }

    println!("unlock update from: {id}");
    let result = unsafe{
        format!("{}\r\n{}", LAST_ATTACK.0, LAST_ATTACK.1)
    };
    *thread_id = step_turn(*thread_id);
    condvar.notify_all();

    result 
}

type Manager = Arc<(Mutex<usize>, Condvar)>;

fn connection_manager(mut stream: TcpStream, manager: Manager) {
    let buf_reader = BufReader::new(&mut stream);
    // let http_request = buf_reader.lines().next().unwrap().unwrap();

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();



    let (status_line, contents) = match http_request[0].as_str(){
        "GET /ID HTTP/1.1" =>
            ("HTTP/1.1 200 OK", id_manager()),
        "GET /HIT HTTP/1.1" =>
            ("HTTP/1.1 200 OK", attack_manager(&http_request, manager)),
        "GET /UPD HTTP/1.1" =>
            ("HTTP/1.1 200 OK", update_manager(&http_request, manager)),
        _ =>
            ("HTTP/1.1 404 OK", String::new()),
    };

    let lenght   = contents.len();
    let response = format!("{status_line}\r\nContent-lenght: {lenght}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
    stream.shutdown(std::net::Shutdown::Both).unwrap();
}

fn step_turn(step: usize) -> usize{
    let next = match step {
        0b00 => 0b11, 
        0b11 => 0b10,
        0b10 => 0b01,
        0b01 => 0b00,
           _ => 0b00,
    };
    println!("{step:02b} -> {next:02b}");
    next
}

pub fn main() {
    unsafe{
        BOARDS[0].init();
        BOARDS[1].init();
    }
    let manager = Arc::new((Mutex::new(0b00), Condvar::new()));
    let listener = TcpListener::bind("localhost:8080").unwrap();

    for stream in listener.incoming(){
        let clone = Arc::clone(&manager);
        thread::spawn(move || connection_manager(stream.unwrap(), clone));
    }
}
