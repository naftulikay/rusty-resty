use consul::Client;
use consul::RegisterService;
use hyper;
use hyper::header::{Headers, ContentType};

fn get_service_id(port: u16) -> String {
    String::from(format!("rusty-{}", port))
}

/// Register this node with Consul
pub fn register(host: String, port: u16) -> Result<(), String>  {
    let client = Client::new("http://127.0.0.1:8500");
    let service = RegisterService {
        ID: get_service_id(port),
        Name: String::from("rusty"),
        Tags: vec!(String::from("rusty"), String::from("rust")),
        Port: port,
        Address: host
    };

    client.agent.register(service)
}

/// Deregister this node with Consul
pub fn deregister(port: u16) {

    let client = hyper::Client::new();
    let mut headers = Headers::new();
    headers.set(ContentType::json());

    let host = "localhost";
    let url = format!("http://localhost:8500/v1/agent/service/deregister/{}",
        get_service_id(port));

    client.put(url.as_str()).headers(headers).send();
}
