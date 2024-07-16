pub struct Alert {
    message: String,
    alert_type: String,
    dismissible: bool,
    icon: Option<String>,
    animated: bool,
}

impl Alert {
    pub fn new(message: &str, alert_type: &str, dismissible: bool, icon: Option<&str>, animated: bool) -> Self {
        Self {
            message: message.to_string(),
            alert_type: alert_type.to_string(),
            dismissible,
            icon: icon.map(|s| s.to_string()),
            animated,
        }
    }

    pub fn render(&self) -> String {
        let icon_html = if let Some(ref icon) = self.icon {
            format!("<i class='icon {}'></i> ", icon)
        } else {
            String::new()
        };

        let animation_class = if self.animated { "alert-animated" } else { "" };

        if self.dismissible {
            format!(
                "<div class='alert alert-{} alert-dismissible {}'>{}{}<button type='button' class='close' data-dismiss='alert'>&times;</button></div>",
                self.alert_type, animation_class, icon_html, self.message
            )
        } else {
            format!("<div class='alert alert-{} {}'>{} {}</div>", self.alert_type, animation_class, icon_html, self.message)
        }
    }
}
