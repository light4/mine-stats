use svg::{
    node::{
        self,
        element::{Circle, Group, Text, SVG},
    },
    Document, Node,
};

use super::{flex_layout, icons::*, CardBuilder};
use crate::github::UserGithubStats;

#[derive(Debug, Clone)]
pub struct StatItem {
    icon: Icon,
    label: String,
    value: i64,
}

impl StatItem {
    pub fn new(icon: Icon, label: &str, value: i64) -> Self {
        Self {
            icon,
            label: label.to_string(),
            value,
        }
    }

    pub fn create_text_node(&self, index: usize, show_icons: bool, bold: bool) -> Group {
        let stagger_delay = (index + 3) * 150;

        let icon_svg = SVG::new()
            .set("data-testid", "icon")
            .set("class", "icon")
            .set("viewBox", (0, 0, 16, 16))
            .set("version", "1.1")
            .set("width", 16)
            .set("height", 16)
            .add(self.icon.svg_path());

        let mut g = Group::new()
            .set("class", "stagger")
            .set("style", format!("animation-delay: {stagger_delay}ms"))
            .set("transform", "translate(25, 0)");
        if show_icons {
            // label offset
            g.append(icon_svg);
        }

        let text_class = if bold { "stat bold" } else { "stat not_bold" };
        let mut text = Text::new()
            .set("class", text_class)
            .set("y", 12.5)
            .add(node::Text::new(&self.label));
        if show_icons {
            text = text.set("x", 25);
        }

        let text_x = if show_icons { 140 + 79 } else { 120 + 79 };
        let text_2 = Text::new()
            .set("x", text_x)
            .set("y", 12.5)
            .set("class", text_class)
            .set("data-testid", self.icon.as_str())
            .add(node::Text::new(format!("{}", self.value)));
        g.add(text).add(text_2)
    }
}

pub fn form_stats_card(github: UserGithubStats, hide_rank: bool, show_icons: bool) -> Document {
    let line_height = 25;
    let rank_level = "A+";
    let width = 495;

    let stat_collections = get_stat_collections(&github);
    let height = std::cmp::max(
        45 + (stat_collections.len() as u16 + 1) * line_height,
        if hide_rank { 0 } else { 150 },
    );

    let rank_circle = if hide_rank {
        Group::new()
    } else {
        let rank_circle_rim = Circle::new()
            .set("class", "rank-circle-rim")
            .set("cx", "-10")
            .set("cy", "8")
            .set("r", "40");
        let rank_circle = Circle::new()
            .set("class", "rank-circle")
            .set("cx", "-10")
            .set("cy", "8")
            .set("r", "40");
        let rank_text = Text::new()
            .set("x", "-5")
            .set("y", "3")
            .set("alignment-baseline", "central")
            .set("dominant-baseline", "central")
            .set("text-anchor", "middle")
            .add(node::Text::new(rank_level));
        let g_rank_text = Group::new().set("class", "rank-text").add(rank_text);

        let rank_x_translation = {
            if width < 450 {
                width - 95 + (45 * (450 - 340)) / 110
            } else {
                width - 95
            }
        };

        Group::new()
            .set("data-testid", "rank-circle")
            .set(
                "transform",
                format!("translate({}, {})", rank_x_translation, height / 2 - 50),
            )
            .add(rank_circle_rim)
            .add(rank_circle)
            .add(g_rank_text)
    };
    // Accessibility Labels
    let desc = &stat_collections
        .iter()
        .map(|item| format!("{}: {}", item.label, item.value))
        .collect::<Vec<String>>()
        .join(", ");

    let mut stat_items = SVG::new().set("x", 0).set("y", 0);
    let stat_items_inner = stat_collections
        .into_iter()
        .enumerate()
        .map(|(idx, item)| item.create_text_node(idx, show_icons, true))
        .collect();
    for item in flex_layout(stat_items_inner, line_height, "column") {
        stat_items.append(item);
    }

    let body = vec![
        rank_circle.get_inner().to_owned(),
        stat_items.get_inner().to_owned(),
    ];

    CardBuilder::default()
        .with_width(width)
        .with_height(height)
        .with_title(format!("{}'s GitHub Stats", &github.name))
        .with_desc(desc)
        .with_theme(super::theme::ONEDARK)
        .build()
        .render(body)
}

pub fn get_stat_collections(github: &UserGithubStats) -> Vec<StatItem> {
    let mut result = vec![];
    for icon in Icon::all() {
        let item = match icon {
            Icon::Star => StatItem::new(icon, "Total Stars Earned: ", github.stars),
            Icon::Commits => StatItem::new(icon, "Total Commits (2022): ", github.commits),
            Icon::Prs => StatItem::new(icon, "Total PRs: ", github.prs),
            Icon::Issues => StatItem::new(icon, "Total Issues: ", github.issues),
            Icon::Contribs => StatItem::new(icon, "Contributed to (last year): ", github.contribs),
            _ => continue,
        };
        result.push(item)
    }

    result
}
