//! stats card themes

use std::{borrow::Cow, ops::Deref, path::Path};

use anyhow::Result;
use kdl::{KdlDocument, KdlNode};
use serde::{Deserialize, Serialize};
use tokio::fs::read_to_string;
use tracing::trace;

pub const DEFAULT: Theme = Theme {
    name: Cow::Borrowed("default"),
    title: Cow::Borrowed("#2f80ed"),
    icon: Cow::Borrowed("#4c71f2"),
    text: Cow::Borrowed("#434d58"),
    bg: Cow::Borrowed("#fffefe"),
    border: Some(Cow::Borrowed("#e4e2e2")),
    ring: Some(Cow::Borrowed("#2f80ed")),
};

pub const ONEDARK: Theme = Theme {
    name: Cow::Borrowed("onedark"),
    title: Cow::Borrowed("#e4bf7a"),
    icon: Cow::Borrowed("#8eb573"),
    text: Cow::Borrowed("#df6d74"),
    bg: Cow::Borrowed("#282c34"),
    border: None,
    ring: None,
};

/// use for cache
#[derive(Debug, Clone)]
pub struct Themes {
    inner: Vec<Theme>,
    default_idx: usize,
}

impl Deref for Themes {
    type Target = Vec<Theme>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl Default for Themes {
    fn default() -> Self {
        Self {
            inner: vec![DEFAULT, ONEDARK],
            default_idx: 0,
        }
    }
}

impl Themes {
    pub fn items(&self) -> &Vec<Theme> {
        &self.inner
    }

    pub fn default(&self) -> &Theme {
        &self.inner[self.default_idx]
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Theme {
    pub name: Cow<'static, str>,
    pub title: Cow<'static, str>,
    pub text: Cow<'static, str>,
    pub icon: Cow<'static, str>,
    pub bg: Cow<'static, str>,
    pub border: Option<Cow<'static, str>>,
    pub ring: Option<Cow<'static, str>>,
}

impl Default for Theme {
    fn default() -> Self {
        DEFAULT
    }
}

impl Themes {
    pub async fn init(path: impl AsRef<Path>) -> Result<Self> {
        fn node_get_color_string(node: &KdlNode) -> String {
            node.entries()
                .first()
                .unwrap()
                .value()
                .as_string()
                .unwrap()
                .to_string()
        }

        let mut result = vec![];
        let themes_str = read_to_string(path).await?;
        let doc: KdlDocument = themes_str.parse()?;

        let default_theme = doc
            .get_arg("default_theme")
            .and_then(|i| i.as_string())
            .map(|i| i.to_string())
            .unwrap_or("onedark".to_string());
        let mut default_idx = 0;
        let mut idx = 0;
        let themes = doc.get("themes").unwrap();
        if let Some(children) = themes.children() {
            for node in children.nodes() {
                // let entry = node.entries();
                let mut theme = Theme {
                    name: node.name().value().to_string().into(),
                    border: None,
                    ring: None,
                    ..Default::default()
                };
                if let Some(colors) = node.children() {
                    for color in colors.nodes() {
                        match color.name().value() {
                            "title" => theme.title = node_get_color_string(color).into(),
                            "icon" => theme.icon = node_get_color_string(color).into(),
                            "text" => theme.text = node_get_color_string(color).into(),
                            "bg" => theme.bg = node_get_color_string(color).into(),
                            "border" => theme.border = Some(node_get_color_string(color).into()),
                            "ring" => theme.ring = Some(node_get_color_string(color).into()),
                            _ => {}
                        }
                    }
                }
                if default_theme == theme.name {
                    default_idx = idx;
                }
                idx += 1;
                result.push(theme);
            }
        }
        trace!("{:#?}", result);

        Ok(Themes {
            inner: result,
            default_idx,
        })
    }

    pub fn find(&self, name: Option<impl AsRef<str>>) -> Theme {
        name.and_then(|i| {
            self.iter()
                .find(|t| t.name.as_ref() == i.as_ref())
                .map(|k| k.to_owned())
        })
        .unwrap_or_else(|| self.default().to_owned())
    }
}
