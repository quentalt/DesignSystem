pub struct Event;

impl Event {
    pub fn on_click(callback: fn()) {
        // Simuler un événement de clic
        callback();
    }

    pub fn on_change(callback: fn(String), value: String) {
        // Simuler un événement de changement
        callback(value);
    }
}
