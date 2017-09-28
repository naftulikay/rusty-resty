use consul::Client;
use consul::RegisterService;

/// Register this node with Consul
pub fn register(host: String, port: u16) -> Result<(), String>  {
    let client = Client::new("http://127.0.0.1:8500");
    let service = RegisterService {
        ID: String::from("rusty"),
        Name: String::from("rusty"),
        Tags: vec!(String::from("rusty"), String::from("rust")),
        Port: port,
        Address: host
    };

    client.agent.register(service)
}
