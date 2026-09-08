use super::message::Message;

#[derive(Debug, Default)]
pub struct AgentState {
    pub messages: Vec<Message>,
    pub iteration: usize,
    pub tool_calls: usize,
}

impl AgentState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn next_iteration(&mut self) {
        self.iteration += 1;
    }

    pub fn record_tool_call(&mut self) {
        self.tool_calls += 1;
    }
}
