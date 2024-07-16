pub struct Navbar {
    brand: String,
    links: Vec<String>,
}

impl Navbar {
    pub fn new(brand: &str, links: Vec<&str>) -> Self {
        Self {
            brand: brand.to_string(),
            links: links.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    pub fn render(&self) -> String {
        let links_html: String = self.links.iter()
            .map(|link| format!("<a class='nav-link' href='#'>{}</a>", link))
            .collect();
        format!(
            "<nav class='navbar'><span class='navbar-brand'>{}</span><div class='navbar-links'>{}</div></nav>",
            self.brand, links_html
        )
    }
}
