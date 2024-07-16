pub struct Tooltip {
    text: String,
    tooltip_text: String,
}

impl Tooltip {
    pub fn new(text: &str, tooltip_text: &str) -> Self {
        Self {
            text: text.to_string(),
            tooltip_text: tooltip_text.to_string(),
        }
    }

    pub fn render(&self) -> String {
        format!(
            "<span class='tooltip'>{text}<span class='tooltiptext'>{tooltip_text}</span></span>",
            text = self.text,
            tooltip_text = self.tooltip_text
        )
    }
}
