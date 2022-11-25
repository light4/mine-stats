use svg::{
    node::{
        self,
        element::{Circle, Group, Text, SVG},
    },
    Document, Node,
};

use super::{flex_layout, icons::*, CardBuilder};

pub fn create_text_node(icon: &Icon, show_icons: bool) -> Group {
    let icon_svg = SVG::new()
        .set("data-testid", "icon")
        .set("class", "icon")
        .set("viewBox", (0, 0, 16, 16))
        .set("version", "1.1")
        .set("width", 16)
        .set("height", 16)
        .add(icon.svg_path());

    let mut g = Group::new()
        .set("class", "stagger")
        .set("style", "animation-delay: ${staggerDelay}ms")
        .set("transform", "translate(25, 0)");
    if show_icons {
        g = g.add(icon_svg);
    }

    let text = Text::new()
        .set("class", r#"stat ${bold ? " bold" : "not_bold"}"#)
        .set("y", 12.5);

    let text_2 = Text::new()
        .set("class", r#"stat ${bold ? " bold" : "not_bold"}"#)
        .set("x", "${(showIcons ? 140 : 120) + shiftValuePos}")
        .set("y", 12.5)
        .set("data-testid", "id")
        .add(node::Text::new("kValue"));

    g.add(text).add(text_2)
}

pub fn form_stats_card(hide_rank: bool, show_icons: bool) -> Document {
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

    let stat_items = create_stat_items(show_icons);

    let body = vec![
        rank_circle.get_inner().to_owned(),
        stat_items.get_inner().to_owned(),
    ];
    CardBuilder::default()
        .with_title("test title")
        .build()
        .render(body)
}

pub fn create_stat_items(show_icons: bool) -> SVG {
    let mut stat_items = SVG::new().set("x", 0).set("y", 0);

    let items = Icon::all()
        .iter()
        .map(|i| create_text_node(i, show_icons))
        .collect();

    for item in flex_layout(items, 10, "column") {
        stat_items.append(item);
    }

    stat_items
}
