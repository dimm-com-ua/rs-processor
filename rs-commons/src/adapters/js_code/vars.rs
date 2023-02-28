use crate::adapters::models::worker::task_variable::TaskVariable;
use rhai::Dynamic;
use std::sync::{Arc, Mutex};

pub struct Vars {
    vars: Arc<Mutex<Vec<TaskVariable>>>,
}

impl Vars {
    pub fn new() -> Self {
        Self {
            vars: Arc::new(Mutex::new(vec![])),
        }
    }

    pub fn fill(&mut self, vars: Vec<TaskVariable>) -> Result<(), String> {
        for x in vars {
            self.vars.lock().unwrap().push(x);
        }
        Ok(())
    }

    pub fn get(&self, name: String) -> Result<Dynamic, ()> {
        match self.vars.lock().unwrap().iter().find(|x| x.name == name) {
            None => Err(()),
            Some(var) => Ok(Dynamic::from(var.value.clone())),
        }
    }

    pub fn put(&self, name: String, value: Dynamic) -> Result<(), ()> {
        if let Some(found_idx) = self
            .vars
            .lock()
            .unwrap()
            .iter()
            .position(|x| x.name == name)
        {
            self.vars.lock().unwrap().remove(found_idx);
        }
        self.vars
            .lock()
            .unwrap()
            .push(TaskVariable::from_dynamic(name, value));
        Ok(())
    }
}
