#[derive()]
pub struct RakunService {
    name: String,
    host: String,
    config: String,
    port: u16,
}

impl RakunService {
    pub fn new(name: String, host: String, config: String, port: u16) -> Self {
        RakunService {
            name,
            host,
            config,
            port,
        }
    }
    pub fn start(&self) {
        println!("Start Agent rakun://{}:{}/{}", self.host, self.port, self.name);
    }
    pub fn stop(&self) {
        println!("Stop Agent rakun://{}:{}/{}", self.host, self.port, self.name);
    }
}