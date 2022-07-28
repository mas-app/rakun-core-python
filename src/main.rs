mod chat;

fn main() {
    let agent_service = chat::chat::AgentService::new();
    agent_service.start();
    agent_service.chat("Hello, world!");
    agent_service.stop();
}