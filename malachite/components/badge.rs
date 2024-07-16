pub struct Badge {
    label: String,
    color: String,
}

impl Badge {
    pub fn new(label: &str, color: &str) -> Self {
        Self {
            label: label.to_string(),
            color: color.to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!("<span class='badge' style='background-color: {}'>{}</span>", self.color, self.label)
    }
}
