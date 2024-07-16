pub struct Modal {
    title: String,
    content: String,
    is_visible: bool,
}

impl Modal {
    pub fn new(title: &str, content: &str, is_visible: bool) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
            is_visible,
        }
    }

    pub fn render(&self) -> String {
        if self.is_visible {
            format!(
                "<div class='modal'><h2>{}</h2><p>{}</p></div>",
                self.title, self.content
            )
        } else {
            String::new()
        }
    }
}
