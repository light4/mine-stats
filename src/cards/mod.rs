use svg::{
    node::{
        self,
        element::{Description, Group, Rectangle, Style, Title},
    },
    Document, Node,
};
use tracing::info;

mod stats;
mod top_langs;

pub use stats::form_stats_card;

#[derive(Debug, Clone, Default)]
pub struct Color {}

#[derive(Debug, Clone, Default)]
pub struct Card {
    width: u16,
    height: u16,
    border_radius: f32,
    colors: Color,
    title: String,
}

#[derive(Clone)]
#[must_use]
pub struct CardBuilder {
    inner: Card,
}

impl Default for CardBuilder {
    fn default() -> Self {
        Self {
            inner: Card {
                width: 100,
                height: 100,
                border_radius: 4.5,
                ..Default::default()
            },
        }
    }
}

impl CardBuilder {
    #[inline]
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.inner.title = title.into();
        self
    }

    #[inline]
    pub fn build(self) -> Card {
        self.inner
    }
}

impl Card {
    pub fn render<T>(&self, body: T) -> Document
    where
        T: Node,
    {
        let title = Title::new().add(node::Text::new(&self.title));
        let desc = Description::new().add(node::Text::new("description"));
        let style = Style::new(
            r#"
          .header {
            font: 600 18px 'Segoe UI', Ubuntu, Sans-Serif;
            fill: ${this.colors.titleColor};
            animation: fadeInAnimation 0.8s ease-in-out forwards;
          }
          @supports(-moz-appearance: auto) {
            /* Selector detects Firefox */
            .header { font-size: 15.5px; }
          }
          ${this.css}
          ${process.env.NODE_ENV === "test" ? "" : getAnimations()}
          ${
            this.animations === false
              ? `* { animation-duration: 0s !important; animation-delay: 0s !important; }`
              : ""
          }
        "#,
        );
        let rect = Rectangle::new()
            .set("x", 0.5)
            .set("y", 0.5)
            .set("rx", self.border_radius)
            .set("height", "99%")
            .set("stroke", "red")
            .set("width", self.width - 1)
            .set("fill", "url(#gradient)")
            .set("stroke-opacity", "${this.hideBorder ? 0 : 1}");

        let g = Group::new()
            .set("data-testid", "main-card-body")
            .set(
                "transform",
                "translate(0, ${
                this.hideTitle ? this.paddingX : this.paddingY + 20
              })",
            )
            .add(body);

        let document = Document::new()
            .set("width", self.width)
            .set("height", self.height)
            .set("viewBox", (0, 0, self.width, self.height))
            .add(title)
            .add(desc)
            .add(style)
            .add(rect)
            .add(g);

        info!("{}", document.to_string());

        document
    }
}
