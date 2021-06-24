use im_rc::Vector;
use reia::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct Todo {
    pub name: String,
    pub completed: bool,
}

pub fn todos_to_string(todos: &Vector<Todo>) -> String {
    serde_json::to_string(todos).unwrap()
}
pub fn todos_from_str(s: &str) -> Vector<Todo> {
    serde_json::from_str(s).unwrap()
}

#[derive(Clone, PartialEq)]
pub struct TodosReducer(pub Updater<Vector<Todo>>);

pub enum TodosReduction {
    Add(String),
    Remove(usize),
    Toggle(usize),
    SetName(usize, String),
    ToggleAll(bool),
    ClearCompleted(),
}

impl TodosReducer {
    pub fn reduce(&self, reduction: TodosReduction) {
        self.0.update_with(|mut v| {
            match reduction {
                TodosReduction::Add(name) => {
                    v.push_back(Todo {
                        name,
                        completed: false,
                    });
                }
                TodosReduction::Remove(idx) => {
                    v.remove(idx);
                }
                TodosReduction::Toggle(idx) => {
                    let current = v.get_mut(idx).unwrap();
                    current.completed = !current.completed;
                }
                TodosReduction::SetName(idx, name) => {
                    let current = v.get_mut(idx).unwrap();
                    current.name = name;
                }
                TodosReduction::ToggleAll(to) => {
                    for td in v.iter_mut() {
                        td.completed = to;
                    }
                }
                TodosReduction::ClearCompleted() => v.retain(|td| !td.completed),
            }
            v
        })
    }
}
