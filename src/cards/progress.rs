use svg::{node::element::Rectangle, Document};

pub fn create_progress_node(
    x: usize,
    y: usize,
    width: u16,
    color: &str,
    progress: u8,
    background_color: &str,
) -> Document {
    let progress = progress.clamp(2, 100);

    let background = Rectangle::new()
        .set("x", 0)
        .set("y", 0)
        .set("rx", 5)
        .set("width", width)
        .set("height", 8)
        .set("fill", background_color);
    let foreground = Rectangle::new()
        .set("height", 8)
        .set("fill", color)
        .set("rx", 5)
        .set("ry", 5)
        .set("x", 0)
        .set("y", 0)
        .set("data-testid", "lang-progress")
        .set("width", progress);
    Document::new()
        .set("width", width)
        .set("x", x)
        .set("y", y)
        .add(background)
        .add(foreground)
}
