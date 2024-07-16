pub struct Popover {
    trigger: String,
    content: String,
    position: String,
}

impl Popover {
    pub fn new(trigger: &str, content: &str, position: &str) -> Self {
        Self {
            trigger: trigger.to_string(),
            content: content.to_string(),
            position: position.to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!(
            "<div class='popover' data-trigger='{}' data-content='{}' data-position='{}'>{}</div>",
            self.trigger, self.content, self.position, self.trigger
        )
    }
}
