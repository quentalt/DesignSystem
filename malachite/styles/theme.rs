pub struct Theme {
    primary_color: String,
    secondary_color: String,
    success_color: String,
    danger_color: String,
    warning_color: String,
    info_color: String,
    light_color: String,
    dark_color: String,
}

impl Theme {
    pub fn new(
        primary_color: &str,
        secondary_color: &str,
        success_color: &str,
        danger_color: &str,
        warning_color: &str,
        info_color: &str,
        light_color: &str,
        dark_color: &str,
    ) -> Self {
        Self {
            primary_color: primary_color.to_string(),
            secondary_color: secondary_color.to_string(),
            success_color: success_color.to_string(),
            danger_color: danger_color.to_string(),
            warning_color: warning_color.to_string(),
            info_color: info_color.to_string(),
            light_color: light_color.to_string(),
            dark_color: dark_color.to_string(),
        }
    }

    pub fn apply(&self) -> String {
        format!(
            ":root {{
                --primary-color: {};
                --secondary-color: {};
                --success-color: {};
                --danger-color: {};
                --warning-color: {};
                --info-color: {};
                --light-color: {};
                --dark-color: {};
            }}",
            self.primary_color,
            self.secondary_color,
            self.success_color,
            self.danger_color,
            self.warning_color,
            self.info_color,
            self.light_color,
            self.dark_color
        )
    }

    pub fn update(&mut self, primary_color: Option<&str>, secondary_color: Option<&str>, success_color: Option<&str>, danger_color: Option<&str>, warning_color: Option<&str>, info_color: Option<&str>, light_color: Option<&str>, dark_color: Option<&str>) {
        if let Some(color) = primary_color {
            self.primary_color = color.to_string();
        }
        if let Some(color) = secondary_color {
            self.secondary_color = color.to_string();
        }
        if let Some(color) = success_color {
            self.success_color = color.to_string();
        }
        if let Some(color) = danger_color {
            self.danger_color = color.to_string();
        }
        if let Some(color) = warning_color {
            self.warning_color = color.to_string();
        }
        if let Some(color) = info_color {
            self.info_color = color.to_string();
        }
        if let Some(color) = light_color {
            self.light_color = color.to_string();
        }
        if let Some(color) = dark_color {
            self.dark_color = color.to_string();
        }
    }
}
