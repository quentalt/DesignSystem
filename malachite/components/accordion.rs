pub struct AccordionItem {
    title: String,
    content: String,
}

impl AccordionItem {
    pub fn new(title: &str, content: &str) -> Self {
        Self {
            title: title.to_string(),
            content: content.to_string(),
        }
    }
}

pub struct Accordion {
    items: Vec<AccordionItem>,
}

impl Accordion {
    pub fn new(items: Vec<AccordionItem>) -> Self {
        Self { items }
    }

    pub fn render(&self) -> String {
        let items_html: String = self.items.iter()
            .map(|item| {
                format!(
                    "<div class='accordion-item'><div class='accordion-title'>{}</div><div class='accordion-content'>{}</div></div>",
                    item.title, item.content
                )
            })
            .collect();

        format!("<div class='accordion'>{}</div>", items_html)
    }
}
