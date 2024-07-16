pub struct Tab {
    label: String,
    content: String,
}

impl Tab {
    pub fn new(label: &str, content: &str) -> Self {
        Self {
            label: label.to_string(),
            content: content.to_string(),
        }
    }
}

pub struct Tabs {
    tabs: Vec<Tab>,
    active_index: usize,
}

impl Tabs {
    pub fn new(tabs: Vec<Tab>, active_index: usize) -> Self {
        Self { tabs, active_index }
    }

    pub fn render(&self) -> String {
        let tabs_html: String = self.tabs.iter().enumerate()
            .map(|(index, tab)| {
                let active_class = if index == self.active_index { "active" } else { "" };
                format!("<li class='tab {}'>{}</li>", active_class, tab.label)
            })
            .collect();

        let content_html = &self.tabs[self.active_index].content;

        format!(
            "<div class='tabs'><ul class='tab-list'>{}</ul><div class='tab-content'>{}</div></div>",
            tabs_html, content_html
        )
    }
}
