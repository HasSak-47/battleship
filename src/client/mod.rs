use std::{
    net::TcpStream,
    io::prelude::*,
};

use rand::random as __random;

#[allow(non_upper_case_globals)]
static random : fn() -> usize = __random::<usize>;

fn request<S>(request: S, fields: Option<Vec<S>>) -> String
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
        Err(_) => String::new(),
    }
}

fn manage_request(mut stream: TcpStream, request: String) -> String{
    stream.write(request.as_bytes()).unwrap();
    stream.shutdown(std::net::Shutdown::Write).unwrap();

    let mut buffer = Vec::<u8>::new();
    stream.read_to_end(&mut buffer).unwrap();
    let response = std::str::from_utf8(&buffer).unwrap();
    let lines : Vec<&str> = response.lines().into_iter().collect();

    //bullshit name
    let mut rrr = String::new();
    for i in 2..lines.len(){
        rrr.push_str(lines[i])

    }

    rrr
}

fn hit(id: usize) -> String{
    // spagqefh
    request("HIT", Some(vec![
        &format!("Id: {id}"),
        &format!("HIT: {}x{}", random() % 10, random() % 10),
    ]))
}
fn upd(id: usize) -> String{"".to_string()}

fn update(response: String) -> bool {
    println!("{response}");
    response.as_str() != "H"
}

pub fn main(){
    let id = usize::from_str_radix(request("ID", None).as_str(), 10).unwrap();
    let funcs = [hit, upd];
    let mut index = id;

    while update(funcs[index % 2](id)) {
        index += 1;
    }
}
