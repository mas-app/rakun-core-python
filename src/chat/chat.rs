use std::cell::Cell;

pub enum AgentStatus {
    CREATED,
    INITIALIZED,
    STOPPED,
}

pub struct AgentService {
    status: Cell<AgentStatus>,
}

impl AgentService {
    fn init(&self) {
        self.status.set(AgentStatus::INITIALIZED);
    }
}

impl AgentService {
    pub fn new() -> AgentService {
        let mut agent_service = AgentService {
            status: Cell::new(AgentStatus::CREATED),
        };
        agent_service.init();
        agent_service
    }

    pub fn start(&self) {
        println!("AgentService started");
    }

    pub fn stop(&self) {
        self.status.set(AgentStatus::STOPPED);
    }

    pub fn chat(&self, message: &str) {
        println!("AgentService received message: {}", message);
    }
}