use svg::{
    node::{
        self,
        element::{Description, Element, Group, Path, Rectangle, Style, Text, Title, SVG},
    },
    Document, Node,
};
use tracing::info;

mod icons;
mod stats;
mod style;
mod theme;
mod top_langs;

pub use stats::form_stats_card;
use theme::Theme;

use crate::cards::theme::DEFAULT;

#[derive(Debug, Clone, Default)]
pub struct Color {}

#[derive(Debug, Clone, Default)]
pub struct Card {
    width: u16,
    height: u16,
    border_radius: f32,
    theme: Theme,
    title: String,
    desc: String,
    hide_border: bool,
    hide_title: bool,
    padding_x: usize,
    padding_y: usize,
    animations: bool,
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
                hide_border: false,
                hide_title: false,
                padding_x: 25,
                padding_y: 35,
                animations: true,
                ..Default::default()
            },
        }
    }
}

impl CardBuilder {
    #[inline]
    pub fn with_width(mut self, width: u16) -> Self {
        self.inner.width = width;
        self
    }

    #[inline]
    pub fn with_height(mut self, height: u16) -> Self {
        self.inner.height = height;
        self
    }

    #[inline]
    pub fn with_title<T: Into<String>>(mut self, title: T) -> Self {
        self.inner.title = title.into();
        self
    }

    #[inline]
    pub fn with_desc<T: Into<String>>(mut self, desc: T) -> Self {
        self.inner.desc = desc.into();
        self
    }

    #[inline]
    pub fn with_theme(mut self, theme: Theme) -> Self {
        self.inner.theme = theme;
        self
    }

    #[inline]
    pub fn build(self) -> Card {
        self.inner
    }
}

impl Card {
    pub fn render_title(&self, title_prefix_icon: Path) -> Group {
        let title = Text::new()
            .set("x", 0)
            .set("y", 0)
            .set("class", "header")
            .set("data-testid", "header")
            .add(node::Text::new(&self.title));

        let prefix_icon = SVG::new()
            .set("class", "icon")
            .set("x", 0)
            .set("y", -13)
            .set("viewBox", "0 0 16 16")
            .set("version", "1.1")
            .set("width", 16)
            .set("height", 16)
            .add(title_prefix_icon);

        let mut g = Group::new().set("data-testid", "card-title").set(
            "transform",
            format!("translate({}, {})", self.padding_x, self.padding_y),
        );

        let items: Vec<Element> = vec![prefix_icon.into(), title.into()];
        for item in flex_layout(items, 25, "") {
            g.append(item);
        }
        g
    }

    pub fn render<I>(&self, body: I) -> Document
    where
        I: IntoIterator,
        I::Item: Node,
    {
        let title = Title::new()
            .set("id", "titleId")
            .add(node::Text::new(&self.title));
        let desc = Description::new()
            .set("id", "descId")
            .add(node::Text::new(&self.desc));
        let style = Style::new(format!(
            r#"
          .header {{
            font: 600 18px 'Segoe UI', Ubuntu, Sans-Serif;
            fill: {};
            animation: fadeInAnimation 0.8s ease-in-out forwards;
          }}
          @supports(-moz-appearance: auto) {{
            /* Selector detects Firefox */
            .header {{ font-size: 15.5px; }}
          }}
          {}

          {}
          {}
        "#,
            self.theme.title,
            style::get_styles(&self.theme, true, 100. - 60.),
            style::get_animations(),
            if self.animations {
                r#"* { animation-duration: 0s !important; animation-delay: 0s !important; }"#
            } else {
                ""
            }
        ));
        let rect = Rectangle::new()
            .set("data-testid", "card-bg")
            .set("x", 0.5)
            .set("y", 0.5)
            .set("rx", self.border_radius)
            .set("height", "99%")
            .set(
                "stroke",
                self.theme
                    .border
                    .as_ref()
                    .unwrap_or(&DEFAULT.border.unwrap())
                    .as_ref(),
            )
            .set("width", self.width - 1)
            .set("fill", self.theme.bg.as_ref())
            .set("stroke-opacity", if self.hide_border { 0 } else { 1 });

        let mut g = Group::new().set("data-testid", "main-card-body").set(
            "transform",
            format!(
                "translate(0, {})",
                if self.hide_title {
                    self.padding_x
                } else {
                    self.padding_y + 20
                }
            ),
        );
        for i in body {
            g.append(i);
        }

        let document = Document::new()
            .set("width", self.width)
            .set("height", self.height)
            .set("viewBox", (0, 0, self.width, self.height))
            .set("fill", "none")
            .set("role", "img")
            .set("aria-labelledby", "descId")
            .add(title)
            .add(desc)
            .add(style)
            .add(rect)
            .add(self.render_title(icons::Icon::Contribs.svg_path()))
            .add(g);

        info!("{}", document.to_string());

        document
    }
}

pub fn flex_layout<T>(items: Vec<T>, gap: u16, direction: &str) -> Vec<Group>
where
    T: Into<Element>,
{
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
                    format!("translate(0, {last_size})")
                } else {
                    format!("translate({last_size}, 0)")
                }
            };
            last_size += size + gap;
            Group::new().set("transform", transform).add(item.into())
        })
        .collect()
}
