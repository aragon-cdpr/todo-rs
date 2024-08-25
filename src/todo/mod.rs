use std::fmt::{Debug, Result};
use ratatui::text::Text;
    
pub struct Todo {
    message: String,
    done: bool
}

impl Todo {
    pub fn get_message(&self) -> &str {
        &self.message
    }
    pub fn is_finished(&self) -> bool {
        self.done
    }
    pub fn toggle(&mut self) -> &mut Self {
        self.done = !self.done;
        self
    }
    pub fn new(message: String, done: bool) -> Todo {
        Todo {
            message,
            done   
        }
    }
}

impl Debug for Todo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result {
        f.debug_struct("Todo")
            .field("message", &self.message)
            .field("done", &self.done)
            .finish()
    }
}

impl From<&Todo> for Text<'static> {
    fn from(todo: &Todo) -> Text<'static> {
        Text::from(
            format!("- [{}] {}", 
                if todo.is_finished() { "x" } else { " " }, 
                todo.get_message()
            )
        )
    }
}

impl Clone for Todo {
    fn clone(&self) -> Self {
        Todo {
            message: self.message.clone(),
            done: self.done
        }
    }
}
