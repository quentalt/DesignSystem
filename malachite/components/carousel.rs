pub struct CarouselItem {
    image_url: String,
    caption: String,
}

impl CarouselItem {
    pub fn new(image_url: &str, caption: &str) -> Self {
        Self {
            image_url: image_url.to_string(),
            caption: caption.to_string(),
        }
    }
}

pub struct Carousel {
    items: Vec<CarouselItem>,
    active_index: usize,
}

impl Carousel {
    pub fn new(items: Vec<CarouselItem>, active_index: usize) -> Self {
        Self { items, active_index }
    }

    pub fn render(&self) -> String {
        let items_html: String = self.items.iter().enumerate()
            .map(|(index, item)| {
                let active_class = if index == self.active_index { "active" } else { "" };
                format!(
                    "<div class='carousel-item {}'><img src='{}' alt='{}'><div class='carousel-caption'>{}</div></div>",
                    active_class, item.image_url, item.caption, item.caption
                )
            })
            .collect();

        format!("<div class='carousel'>{}</div>", items_html)
    }
}
