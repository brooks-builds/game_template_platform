use std::collections::HashMap;

use crate::component::{Component, ComponentDATA};

pub trait System {
    fn run(&mut self, data: &mut HashMap<String, Vec<Box<dyn ComponentDATA>>>);
}
