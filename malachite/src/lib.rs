pub mod components;
pub mod styles;
pub mod utils;


#[cfg(test)]
mod tests {
    use super::components::button::Button;
    use super::components::input::Input;
    use super::components::card::Card;
    use super::components::modal::Modal;
    use super::components::badge::Badge;
    use super::components::alert::Alert;
    use super::components::navbar::Navbar;
    use super::components::tooltip::Tooltip;
    use super::components::dropdown::Dropdown;
    use super::components::progress_bar::ProgressBar;
    use super::components::tabs::{Tabs, Tab};
    use super::components::accordion::{Accordion, AccordionItem};
    use super::components::carousel::{Carousel, CarouselItem};
    use super::components::popover::Popover;
    use super::components::table::Table;
    use super::components::pagination::Pagination;
    use super::styles::color::Color;
    use super::styles::border::Border;
    use super::styles::spacing::Spacing;
    use super::styles::theme::Theme;
    use super::styles::animation::Animation;
    use super::utils::event::Event;
    use super::utils::hooks::{use_state, State};

    #[test]
    fn test_button_render() {
        let button = Button::new("Click Me", Color::PRIMARY, "md", "primary", Some("icon-class"), false, false, Some("custom-style"));
        assert_eq!(button.render(), "<button class='btn btn-primary btn-md' style='custom-style' ><i class='icon icon-class'></i> Click Me</button>");
    }

    #[test]
    fn test_input_render() {
        let input = Input::new("Enter text", "Hello");
        assert_eq!(input.render(), "<input placeholder='Enter text' value='Hello' />");
    }

    #[test]
    fn test_card_render() {
        let card = Card::new("Card Title", "Card content");
        assert_eq!(card.render(), "<div class='card'><h2>Card Title</h2><p>Card content</p></div>");
    }

    #[test]
    fn test_modal_render() {
        let modal = Modal::new("Modal Title", "Modal content", true);
        assert_eq!(modal.render(), "<div class='modal'><h2>Modal Title</h2><p>Modal content</p></div>");
    }

    #[test]
    fn test_badge_render() {
        let badge = Badge::new("New", Color::SUCCESS);
        assert_eq!(badge.render(), "<span class='badge' style='background-color: #28a745'>New</span>");
    }

    #[test]
    fn test_alert_render() {
        let alert = Alert::new("This is an alert!", "danger", true, Some("icon-class"), true);
        assert_eq!(alert.render(), "<div class='alert alert-danger alert-dismissible alert-animated'><i class='icon icon-class'></i> This is an alert!<button type='button' class='close' data-dismiss='alert'>&times;</button></div>");
    }

    #[test]
    fn test_navbar_render() {
        let navbar = Navbar::new("MyBrand", vec!["Home", "About", "Contact"]);
        assert_eq!(
            navbar.render(),
            "<nav class='navbar'><span class='navbar-brand'>MyBrand</span><div class='navbar-links'><a class='nav-link' href='#'>Home</a><a class='nav-link' href='#'>About</a><a class='nav-link' href='#'>Contact</a></div></nav>"
        );
    }

    #[test]
    fn test_tooltip_render() {
        let tooltip = Tooltip::new("Hover over me", "Tooltip text");
        assert_eq!(
            tooltip.render(),
            "<span class='tooltip'>Hover over me<span class='tooltiptext'>Tooltip text</span></span>"
        );
    }

    #[test]
    fn test_dropdown_render() {
        let dropdown = Dropdown::new("Choose an option", vec!["Option 1", "Option 2", "Option 3"]);
        assert_eq!(
            dropdown.render(),
            "<label>Choose an option<select><option value='Option 1'>Option 1</option><option value='Option 2'>Option 2</option><option value='Option 3'>Option 3</option></select></label>"
        );
    }

    #[test]
    fn test_progress_bar_render() {
        let progress_bar = ProgressBar::new(50, 100);
        assert_eq!(
            progress_bar.render(),
            "<div class='progress'><div class='progress-bar' style='width: 50%'></div></div>"
        );
    }

    #[test]
    fn test_tabs_render() {
        let tabs = Tabs::new(
            vec![
                Tab::new("Tab 1", "Content 1"),
                Tab::new("Tab 2", "Content 2"),
                Tab::new("Tab 3", "Content 3"),
            ],
            1,
        );
        assert_eq!(
            tabs.render(),
            "<div class='tabs'><ul class='tab-list'><li class='tab '>Tab 1</li><li class='tab active'>Tab 2</li><li class='tab '>Tab 3</li></ul><div class='tab-content'>Content 2</div></div>"
        );
    }

    #[test]
    fn test_accordion_render() {
        let accordion = Accordion::new(vec![
            AccordionItem::new("Item 1", "Content 1"),
            AccordionItem::new("Item 2", "Content 2"),
            AccordionItem::new("Item 3", "Content 3"),
        ]);
        assert_eq!(
            accordion.render(),
            "<div class='accordion'><div class='accordion-item'><div class='accordion-title'>Item 1</div><div class='accordion-content'>Content 1</div></div><div class='accordion-item'><div class='accordion-title'>Item 2</div><div class='accordion-content'>Content 2</div></div><div class='accordion-item'><div class='accordion-title'>Item 3</div><div class='accordion-content'>Content 3</div></div></div>"
        );
    }

    #[test]
    fn test_carousel_render() {
        let carousel = Carousel::new(
            vec![
                CarouselItem::new("image1.jpg", "Caption 1"),
                CarouselItem::new("image2.jpg", "Caption 2"),
                CarouselItem::new("image3.jpg", "Caption 3"),
            ],
            0,
        );
        assert_eq!(
            carousel.render(),
            "<div class='carousel'><div class='carousel-item active'><img src='image1.jpg' alt='Caption 1'><div class='carousel-caption'>Caption 1</div></div><div class='carousel-item '><img src='image2.jpg' alt='Caption 2'><div class='carousel-caption'>Caption 2</div></div><div class='carousel-item '><img src='image3.jpg' alt='Caption 3'><div class='carousel-caption'>Caption 3</div></div></div>"
        );
    }

    #[test]
    fn test_popover_render() {
        let popover = Popover::new("Click me", "Popover content", "top");
        assert_eq!(
            popover.render(),
            "<div class='popover' data-trigger='Click me' data-content='Popover content' data-position='top'>Click me</div>"
        );
    }

    #[test]
    fn test_table_render() {
        let table = Table::new(
            vec!["Header 1", "Header 2", "Header 3"],
            vec![
                vec!["Row 1 Col 1", "Row 1 Col 2", "Row 1 Col 3"],
                vec!["Row 2 Col 1", "Row 2 Col 2", "Row 2 Col 3"],
            ],
        );
        assert_eq!(
            table.render(),
            "<table class='table'><thead><tr><th>Header 1</th><th>Header 2</th><th>Header 3</th></tr></thead><tbody><tr><td>Row 1 Col 1</td><td>Row 1 Col 2</td><td>Row 1 Col 3</td></tr><tr><td>Row 2 Col 1</td><td>Row 2 Col 2</td><td>Row 2 Col 3</td></tr></tbody></table>"
        );
    }

    #[test]
    fn test_pagination_render() {
        let pagination = Pagination::new(5, 3);
        assert_eq!(
            pagination.render(),
            "<ul class='pagination'><li class='page-item '><a class='page-link' href='#'>1</a></li><li class='page-item '><a class='page-link' href='#'>2</a></li><li class='page-item active'><a class='page-link' href='#'>3</a></li><li class='page-item '><a class='page-link' href='#'>4</a></li><li class='page-item '><a class='page-link' href='#'>5</a></li></ul>"
        );
    }

    #[test]
    fn test_border_styles() {
        assert_eq!(Border::RADIUS_SMALL, "4px");
        assert_eq!(Border::RADIUS_MEDIUM, "8px");
        assert_eq!(Border::RADIUS_LARGE, "16px");
    }

    #[test]
    fn test_spacing_styles() {
        assert_eq!(Spacing::SMALL, "8px");
        assert_eq!(Spacing::MEDIUM, "16px");
        assert_eq!(Spacing::LARGE, "32px");
    }

    #[test]
    fn test_event_on_click() {
        let mut clicked = false;
        Event::on_click(|| {
            clicked = true;
        });
        assert!(clicked);
    }

    #[test]
    fn test_event_on_change() {
        let mut value = String::new();
        Event::on_change(|new_value| {
            value = new_value;
        });

        Event::trigger_change("New Value".to_string());
        assert_eq!(value, "New Value");
    }

    #[test]
    fn test_use_state() {
        let state = use_state("Initial Value".to_string());
        assert_eq!(state.get(), "Initial Value");

        state.set("New Value".to_string());
        assert_eq!(state.get(), "New Value");
    }

    #[test]
    fn test_theme_styles() {
        assert_eq!(Theme::PRIMARY, "#007bff");
        assert_eq!(Theme::SECONDARY, "#6c757d");
        assert_eq!(Theme::SUCCESS, "#28a745");
        assert_eq!(Theme::DANGER, "#dc3545");
        assert_eq!(Theme::WARNING, "#ffc107");
        assert_eq!(Theme::INFO, "#17a2b8");
        assert_eq!(Theme::LIGHT, "#f8f9fa");
        assert_eq!(Theme::DARK, "#343a40");
    }

    #[test]
    fn test_animation_styles() {
        assert_eq!(Animation::FADE, "fade");
        assert_eq!(Animation::SLIDE, "slide");
        assert_eq!(Animation::ZOOM, "zoom");
        assert_eq!(Animation::BOUNCE, "bounce");
    }

    


