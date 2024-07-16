pub struct Button {
    label: String,
    color: String,
    size: String,
    variant: String,
    icon: Option<String>,
    disabled: bool,
    loading: bool,
    custom_style: Option<String>,
}

impl Button {
    pub fn new(label: &str, color: &str, size: &str, variant: &str, icon: Option<&str>, disabled: bool, loading: bool, custom_style: Option<&str>) -> Self {
        Self {
            label: label.to_string(),
            color: color.to_string(),
            size: size.to_string(),
            variant: variant.to_string(),
            icon: icon.map(|s| s.to_string()),
            disabled,
            loading,
            custom_style: custom_style.map(|s| s.to_string()),
        }
    }

    pub fn render(&self) -> String {
        let icon_html = if let Some(ref icon) = self.icon {
            format!("<i class='icon {}'></i> ", icon)
        } else {
            String::new()
        };

        let loading_html = if self.loading {
            "<span class='spinner'></span> ".to_string()
        } else {
            String::new()
        };

        let disabled_attr = if self.disabled { "disabled" } else { "" };

        let custom_style = if let Some(ref style) = self.custom_style {
            format!("style='{}'", style)
        } else {
            String::new()
        };

        format!(
            "<button class='btn btn-{} btn-{}' {} {}>{}{} {}</button>",
            self.variant, self.size, custom_style, disabled_attr, loading_html, icon_html, self.label
        )
    }
}
