mod client;
mod id_gen;
mod message;

pub use client::Client;
pub use id_gen::IdGen;
pub use message::Message;
use std::sync::Mutex;
use std::collections::HashMap;
use std::collections::HashSet;

pub struct App {
    client_ids: Mutex<IdGen>,
    active_ids: Mutex<Option<HashSet<usize>>>,
    message_queues: Mutex<Option<HashMap<usize, Vec<Message>>>>
}

impl App {
    pub const fn new() -> Self {
        Self {
            client_ids: Mutex::new(IdGen::new()),
            active_ids: Mutex::new(None),
            message_queues: Mutex::new(None)
        }
    }

    pub fn init(&mut self) {
        *self.message_queues.lock().unwrap() = Some(HashMap::new());
        *self.active_ids.lock().unwrap() = Some(HashSet::new());
    }

    pub fn add_client(&mut self) -> Option<usize> {
        let id = self.client_ids.lock().unwrap().get_id();

        if let Some(id) = id {
            self.active_ids.lock().unwrap().as_mut().unwrap().insert(id);
            self.message_queues.lock().unwrap().as_mut().unwrap().insert(id, vec![]);
            
            Some(id)
        }
        else {
            None
        }
    }

    pub fn kill_client(&mut self, client_id: usize) {
        self.active_ids.lock().unwrap().as_mut().unwrap().remove(&client_id);
        self.message_queues.lock().unwrap().as_mut().unwrap().remove(&client_id);
        self.client_ids.lock().unwrap().return_id(client_id);
    }

    pub fn send_message(&mut self, message: Message) {
        for id in self.active_ids.lock().unwrap().as_ref().unwrap() {
            self.message_queues.lock().unwrap().as_mut().unwrap().get_mut(&id).unwrap().push(message.clone());
        }
    }

    pub fn recieve_messages(&mut self, client_id: usize) -> Vec<Message> {
        let mut messages = vec![];

        let mut queues = self.message_queues.lock().unwrap();
        let mut message_queue = queues.as_mut().unwrap().get_mut(&client_id);

        if let None = message_queue {
            return vec![];
        }

        for message in message_queue.as_deref_mut().unwrap() {
            messages.push(message.clone());
        }

        message_queue.unwrap().clear();

        messages
    }
}