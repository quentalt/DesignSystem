pub struct Dropdown {
    label: String,
    options: Vec<String>,
}

impl Dropdown {
    pub fn new(label: &str, options: Vec<&str>) -> Self {
        Self {
            label: label.to_string(),
            options: options.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn render(&self) -> String {
        let options_html: String = self.options.iter()
            .map(|option| format!("<option value='{}'>{}</option>", option, option))
            .collect();
        format!(
            "<label>{label}<select>{options}</select></label>",
            label = self.label,
            options = options_html
        )
    }
}
