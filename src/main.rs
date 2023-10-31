mod app;

use fp_server::*;
use app::*;
use std::io::Read;

static mut APP: App = App::new();

fn main() {
    unsafe {APP.init()};

    const SERVER_ADDR: &str = "0.0.0.0:8080";

    let mut server = HTTPServer::new(SERVER_ADDR).unwrap();
    println!("Server started at {}", SERVER_ADDR);
    server.handle(RequestMethod::Get, Some(get));
    server.handle(RequestMethod::Post, Some(post));

    server.start()
}

//
// GET
//

fn get(request: Request) -> Response {
    match request.uri.as_str() {
        _ => {
            get_base(&request)
        }
    }
}

fn get_base(request: &Request) -> Response {
    let mut uri = request.uri.clone();

    if uri.ends_with("/") {
        uri += "index.html";
    }

    let file_result = std::fs::File::open(String::from("./server") + uri.as_str());

    let mut file = match file_result {
        Ok(file) => file,
        Err(_) => {
            return Response::create_404(&request.version, &[]);
        }
    };

    let mut buffer = vec![];
    if let Err(_) = file.read_to_end(&mut buffer) {
        return Response::create_404(&request.version, &[]);
    }

    Response::create_200(&request.version, &buffer)
}

//
// POST
//

fn post(request: Request) -> Response {
    match request.uri.as_str() {
        "/join" => {
            join(&request)
        },
        "/leave" => {
            leave(&request)
        },
        "/send" => {
            send(&request)
        },
        "/read" => {
            recieve(&request)
        }
        _ => {
            Response::create_404(&request.version, &[])
        }
    }
}

fn join(request: &Request) -> Response {
    let client = unsafe {APP.add_client()};

    if let Some(id) = client {
        let data = format!("{{\"id\": {}}}", id);

        let mut response = Response::create_200(&request.version, data.as_bytes());
        response.add_header("content-type: text/json");
        response
    }
    else {
        Response::create_404(&request.version, &[])
    }
}

fn leave(request: &Request) -> Response {
    let json = String::from_utf8_lossy(&request.body);

    let client: Client = serde_json::from_str(&json).unwrap();

    unsafe {APP.kill_client(client.id)};

    Response::create_200(&request.version, &[])
}

fn send(request: &Request) -> Response {
    let json = String::from_utf8_lossy(&request.body);
    let message: Message = serde_json::from_str(&json).unwrap();

    unsafe {APP.send_message(message)};

    Response::create_200(&request.version, &[])
}

fn recieve(request: &Request) -> Response {
    let json = String::from_utf8_lossy(&request.body);
    let client: Client = serde_json::from_str(&json).unwrap();

    let messages = unsafe {APP.recieve_messages(client.id)};

    if messages.is_empty() {
        Response::create_200(&request.version, b"{\"messages\": []}")
    }
    else {
        let mut json = String::from("{\"messages\": [");

        for message in messages {
            let msg_json = serde_json::to_string(&message).unwrap();
            json += msg_json.as_str();
            json += ", ";
        }

        json = json.trim_end_matches(", ").to_string();
        json += "]}";

        Response::create_200(&request.version, json.as_bytes())
    }
}