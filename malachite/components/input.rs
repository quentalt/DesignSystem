pub struct Input {
    placeholder: String,
    value: String,
}

impl Input {
    pub fn new(placeholder: &str, value: &str) -> Self {
        Self {
            placeholder: placeholder.to_string(),
            value: value.to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!("<input placeholder='{}' value='{}' />", self.placeholder, self.value)
    }
}
