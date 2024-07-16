pub mod event;
pub mod hooks;

pub fn to_css_class(name: &str) -> String {
    name.to_lowercase().replace(" ", "-")
}
