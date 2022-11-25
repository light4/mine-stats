use svg::{
    node::{
        self,
        element::{Description, Group, Rectangle, Style, Title},
    },
    Document, Node,
};
use tracing::info;

mod icons;
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
    pub fn render<I>(&self, body: I) -> Document
    where
        I: IntoIterator,
        I::Item: Node,
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

        let mut g = Group::new().set("data-testid", "main-card-body").set(
            "transform",
            "translate(0, ${
                this.hideTitle ? this.paddingX : this.paddingY + 20
              })",
        );
        for i in body {
            g.append(i);
        }

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

pub fn flex_layout(items: Vec<Group>, gap: usize, direction: &str) -> Vec<Group> {
    let mut last_size = 0;
    // filter() for filtering out empty strings
    items
        .into_iter()
        .enumerate()
        .map(|(_i, item)| {
            // let size = sizes.get(i).copied().unwrap_or(0);
            let size = 0;
            let transform = {
                if direction == "column" {
                    format!("translate(0, ${last_size})")
                } else {
                    format!("translate(${last_size}, 0)")
                }
            };
            last_size += size + gap;
            Group::new().set("transform", transform).add(item)
        })
        .collect()
}
