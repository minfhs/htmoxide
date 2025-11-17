use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub completed: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub editing: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TodoList {
    pub todos: Vec<Todo>,
    #[serde(default)]
    pub next_id: usize,
}

impl TodoList {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: String) -> usize {
        let id = self.next_id;
        self.next_id += 1;
        self.todos.push(Todo {
            id,
            title,
            completed: false,
            editing: None,
        });
        id
    }

    pub fn toggle(&mut self, id: usize) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.completed = !todo.completed;
        }
    }

    pub fn toggle_all(&mut self, completed: bool) {
        for todo in &mut self.todos {
            todo.completed = completed;
        }
    }

    pub fn delete(&mut self, id: usize) {
        self.todos.retain(|t| t.id != id);
    }

    pub fn update_title(&mut self, id: usize, title: String) {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.title = title;
        }
    }

    pub fn clear_completed(&mut self) {
        self.todos.retain(|t| !t.completed);
    }

    pub fn active_count(&self) -> usize {
        self.todos.iter().filter(|t| !t.completed).count()
    }

    pub fn completed_count(&self) -> usize {
        self.todos.iter().filter(|t| t.completed).count()
    }

    pub fn filtered(&self, filter: &str) -> Vec<&Todo> {
        match filter {
            "active" => self.todos.iter().filter(|t| !t.completed).collect(),
            "completed" => self.todos.iter().filter(|t| t.completed).collect(),
            _ => self.todos.iter().collect(),
        }
    }
}
