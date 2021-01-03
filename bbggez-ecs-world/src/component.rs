pub trait ComponentDATA {}

pub struct Component {
    data: Box<dyn ComponentDATA>,
}

impl Component {
    pub fn new(data: Box<dyn ComponentDATA>) -> Self {
        Self { data }
    }
}

impl ComponentDATA for Component {}
