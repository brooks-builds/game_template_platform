pub mod component;

use std::collections::HashMap;

use component::ComponentData;

pub struct World {
    component_data: HashMap<String, Vec<ComponentData>>,
    components_count: usize,
}

impl World {
    pub fn new() -> Self {
        Self {
            component_data: HashMap::new(),
            components_count: 0,
        }
    }

    pub fn register(&mut self, name: &str) {
        self.component_data.insert(name.to_owned(), vec![]);
    }

    pub fn create_component(&mut self) -> &mut Self {
        self.component_data
            .iter_mut()
            .for_each(|(name, data)| data.push(ComponentData::None));
        self.components_count += 1;
        self
    }

    pub fn with(&mut self, name: &str, data: ComponentData) -> &mut Self {
        if let Some(component) = self.component_data.get_mut(name) {
            component[self.components_count - 1] = data;
        }

        self
    }

    pub fn query(&self, name: &str) -> Option<&Vec<ComponentData>> {
        self.component_data.get(name)
    }

    pub fn length(&self) -> usize {
        self.components_count
    }
}
