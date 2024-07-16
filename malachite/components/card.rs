pub struct Card {
    title: String,
    content: String,
}

impl Card {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!(
            "<div class='card'><h2>{}</h2><p>{}</p></div>",
            self.title, self.content
        )
    }
}
