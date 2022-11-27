use svg::{
    node,
    node::element::{Circle, Group, Text, SVG},
    Document, Node,
};
use tracing::info;

use super::{theme::ONEDARK, CardBuilder};
use crate::github::top_langs::{Lang, TopLangs};

const DEFAULT_CARD_WIDTH: usize = 300;
const MIN_CARD_WIDTH: u16 = 230;
const DEFAULT_LANGS_COUNT: usize = 5;
const DEFAULT_LANG_COLOR: &str = "#858585";
const CARD_PADDING: usize = 25;

fn create_progress_text_node(width: u16, name: &str, color: &str, progress: u8) -> Group {
    let padding_right = 95;
    let progress_text_x = width - padding_right + 10;
    let progress_width = width - padding_right;

    let name_text = Text::new()
        .set("data-testid", "lang-name")
        .set("x", 2)
        .set("y", 15)
        .set("class", "lang-name")
        .add(node::Text::new(name));
    let progress_text = Text::new()
        .set("x", progress_text_x)
        .set("y", "34")
        .set("class", "lang-name")
        .add(node::Text::new(format!("{}%", progress)));
    let progress_node =
        super::progress::create_progress_node(0, 25, progress_width, color, progress, "#ddd");

    Group::new()
        .add(name_text)
        .add(progress_text)
        .add(progress_node)
}

fn create_compact_lang_node(lang: Lang, total_size: usize) -> Group {
    let percentage = lang.size * 100 / total_size;
    let color = lang.color.unwrap_or("#858585".to_string());

    let circle = Circle::new()
        .set("cx", 5)
        .set("cy", 6)
        .set("r", 5)
        .set("fill", color);
    let lang_text = Text::new()
        .set("data-testid", "lang-name")
        .set("x", 15)
        .set("y", 10)
        .set("class", "lang-name")
        .add(node::Text::new(format!("{} {}%", lang.name, percentage)));
    Group::new().add(circle).add(lang_text)
}

fn render_normal_layout(langs: Vec<Lang>, width: u16) -> Vec<Group> {
    let total_language_size: usize = langs.iter().map(|i| i.size).sum();
    let items = langs
        .iter()
        .map(|lang| {
            let color = lang.color.clone().unwrap_or(DEFAULT_LANG_COLOR.to_owned());
            let progress: usize = lang.size * 100 / total_language_size;
            create_progress_text_node(width, &lang.name, &color, progress as u8)
        })
        .collect();
    super::flex_layout(items, 40, "column")
}

fn calculate_normal_layout_height(total_langs: u16) -> u16 {
    45 + (total_langs + 1) * 40
}

fn use_languages(top_langs: TopLangs, hide: Vec<String>, langs_count: usize) -> Vec<Lang> {
    let langs_count = langs_count.clamp(1, 10);
    let langs_to_hide: Vec<String> = hide
        .into_iter()
        .map(|i| i.trim().to_ascii_lowercase())
        .collect();

    let mut result: Vec<Lang> = top_langs.langs.into_iter().map(|(_, v)| v).collect();
    result.sort_by(|a, b| b.size.cmp(&a.size));
    result
        .into_iter()
        .filter(|lang| !langs_to_hide.contains(&lang.name.trim().to_ascii_lowercase()))
        .take(langs_count)
        .collect()
}

pub fn form_top_langs_card(
    top_langs: TopLangs,
    hide: Vec<String>,
    langs_count: usize,
    card_width: u16,
) -> Document {
    let langs = use_languages(top_langs, hide, langs_count);
    info!("{:?}", langs);
    // DEFAUT
    let width = if card_width < MIN_CARD_WIDTH {
        MIN_CARD_WIDTH
    } else {
        card_width
    };
    let height = calculate_normal_layout_height(langs.len() as u16);
    let final_layout = render_normal_layout(langs, width);

    let mut body = SVG::new()
        .set("data-testid", "lang-items")
        .set("x", CARD_PADDING);
    for node in final_layout {
        body.append(node);
    }

    let theme = ONEDARK;
    CardBuilder::default()
        .with_width(width)
        .with_height(height)
        .with_title("Most Used Languages")
        .with_theme(theme)
        .with_animations(false)
        // .set_hide_border(hide_border)
        // .set_hide_title(hide_title)
        .with_css(r###".lang-name { font: 400 11px 'Segoe UI', Ubuntu, Sans-Serif; fill: ${colors.textColor} }"###,)
        .build()
        .render([body])
}
