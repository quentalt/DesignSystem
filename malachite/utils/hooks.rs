pub struct State<T> {
    value: T,
}

impl<T> State<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}

pub fn use_state<T>(initial_value: T) -> State<T> {
    State::new(initial_value)
}
