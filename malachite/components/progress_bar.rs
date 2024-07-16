pub struct ProgressBar {
    value: u32,
    max: u32,
}

impl ProgressBar {
    pub fn new(value: u32, max: u32) -> Self {
        Self { value, max }
    }

    pub fn render(&self) -> String {
        let percentage = (self.value as f64 / self.max as f64) * 100.0;
        format!(
            "<div class='progress'><div class='progress-bar' style='width: {}%'></div></div>",
            percentage
        )
    }
}
