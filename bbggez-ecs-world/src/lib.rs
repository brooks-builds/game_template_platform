pub mod component;
pub mod system;

use std::collections::HashMap;

use component::{Component, ComponentDATA};

pub struct World<DATA: ComponentDATA> {
    component_data: HashMap<String, Vec<DATA>>,
}

impl<DATA: ComponentDATA> World<DATA> {
    pub fn new() -> Self {
        Self {
            component_data: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: &str) {
        let component = vec![];
        self.component_data.insert(name.to_owned(), component);
    }

    pub fn create_component(&mut self) -> &mut Self {
        self
    }

    pub fn with(&mut self, name: &str, data: DATA) -> &mut Self {
        if let Some(component) = self.component_data.get_mut(name) {
            component.push(data);
        }

        self
    }

    // pub fn draw<FUNCTION: FnOnce(&Vec<(f32, f32)>, &Vec<f32>)>(&self, closure: FUNCTION) {
    //     closure(&self.locations, &self.size);
    // }
}
