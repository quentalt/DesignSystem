pub struct Pagination {
    total_pages: u32,
    current_page: u32,
}

impl Pagination {
    pub fn new(total_pages: u32, current_page: u32) -> Self {
        Self { total_pages, current_page }
    }

    pub fn render(&self) -> String {
        let mut pages_html = String::new();
        for page in 1..=self.total_pages {
            let active_class = if page == self.current_page { "active" } else { "" };
            pages_html.push_str(&format!("<li class='page-item {}'><a class='page-link' href='#'>{}</a></li>", active_class, page));
        }

        format!("<ul class='pagination'>{}</ul>", pages_html)
    }
}
