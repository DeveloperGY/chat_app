use std::collections::VecDeque;

pub struct IdGen {
    current_id: usize,
    available_ids: VecDeque<usize>
}

impl IdGen {
    pub const fn new() -> Self {
        Self {
            current_id: 0,
            available_ids: VecDeque::new()
        }
    }

    pub fn get_id(&mut self) -> Option<usize> {
        if !self.available_ids.is_empty() {
            let id = self.available_ids.pop_back().unwrap();
            Some(id)
        }
        else {
            let id = self.current_id;
            self.current_id = match self.current_id.checked_add(1) {
                Some(id) => id,
                None => return None
            };

            Some(id)
        }
    }

    pub fn return_id(&mut self, id: usize) {
        if self.current_id > id && !self.available_ids.contains(&id) {
            self.available_ids.push_back(id);
        }
    }
}