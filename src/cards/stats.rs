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

        let mut text = Text::new()
            .set("class", r#"stat ${bold ? " bold" : "not_bold"}"#)
            .set("y", 12.5)
            .add(node::Text::new(&self.label));
        if show_icons {
            text = text.set("x", 25);
        }

        let text_class = if bold { "stat bold" } else { "stat not_bold" };
        let text_x = if show_icons { 140 + 79 } else { 120 + 79 };
        let text_2 = Text::new()
            .set("x", text_x)
            .set("y", 12.5)
            .set("class", text_class)
            .set("data-testid", "id")
            .add(node::Text::new(format!("{}", self.value)));
        g.add(text).add(text_2)
    }
}

pub fn form_stats_card(github: UserGithubStats, hide_rank: bool, show_icons: bool) -> Document {
    let rank_level = "A+";

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

        Group::new()
            .set("data-testid", "rank-circle")
            .set(
                "transform",
                "translate(${calculateRankXTranslation()}, ${height / 2 - 50})",
            )
            .add(rank_circle_rim)
            .add(rank_circle)
            .add(g_rank_text)
    };

    let stat_items = create_stat_items(&github, show_icons);

    let body = vec![
        rank_circle.get_inner().to_owned(),
        stat_items.get_inner().to_owned(),
    ];
    CardBuilder::default()
        .with_width(495)
        .with_height(195)
        .with_title(format!("{}'s GitHub Stats", &github.name))
        .with_desc("Total Stars Earned: 37, Total Commits in 2022 : 37, Total PRs: 77, Total Issues: 29, Contributed to (last year): 20")
        .build()
        .render(body)
}

pub fn create_stat_items(github: &UserGithubStats, show_icons: bool) -> SVG {
    let mut stat_items = SVG::new().set("x", 0).set("y", 0);

    let mut items = vec![];
    for (idx, icon) in Icon::all().iter().enumerate() {
        let stat_item = match icon {
            Icon::Star => StatItem::new(*icon, "Total Stars Earned: ", github.stars),
            Icon::Commits => StatItem::new(*icon, "Total Commits (2022): ", github.commits),
            Icon::Prs => StatItem::new(*icon, "Total PRs: ", github.prs),
            Icon::Issues => StatItem::new(*icon, "Total Issues: ", github.issues),
            Icon::Contribs => StatItem::new(*icon, "Contributed to (last year): ", github.contribs),
            _ => continue,
        };
        items.push(stat_item.create_text_node(idx, show_icons, true))
    }

    for item in flex_layout(items, 10, "column") {
        stat_items.append(item);
    }

    stat_items
}
