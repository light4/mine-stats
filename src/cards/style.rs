use std::f32::consts::PI;

use super::theme::Theme;

pub fn get_animations() -> &'static str {
    r#"/* Animations */
      @keyframes scaleInAnimation {
        from {
          transform: translate(-5px, 5px) scale(0);
        }
        to {
          transform: translate(-5px, 5px) scale(1);
        }
      }
      @keyframes fadeInAnimation {
        from {
          opacity: 0;
        }
        to {
          opacity: 1;
        }
      }
    "#
}

pub fn get_styles(theme: &Theme, show_icons: bool, progress: f32) -> String {
    format!(
        r#"
      .stat {{
        font: 600 14px 'Segoe UI', Ubuntu, "Helvetica Neue", Sans-Serif; fill: {};
      }}
      @supports(-moz-appearance: auto) {{
        /* Selector detects Firefox */
        .stat {{ font-size:12px; }}
      }}
      .stagger {{
        opacity: 0;
        animation: fadeInAnimation 0.3s ease-in-out forwards;
      }}
      .rank-text {{
        font: 800 24px 'Segoe UI', Ubuntu, Sans-Serif; fill: {};
        animation: scaleInAnimation 0.3s ease-in-out forwards;
      }}

      .not_bold {{ font-weight: 400 }}
      .bold {{ font-weight: 700 }}
      .icon {{
        fill: {};
        display: {};
      }}
      .rank-circle-rim {{
        stroke: {};
        fill: none;
        stroke-width: 6;
        opacity: 0.2;
      }}
      .rank-circle {{
        stroke: {};
        stroke-dasharray: 250;
        fill: none;
        stroke-width: 6;
        stroke-linecap: round;
        opacity: 0.8;
        transform-origin: -10px 8px;
        transform: rotate(-90deg);
        animation: rankAnimation 1s forwards ease-in-out;
      }}
      {}
    "#,
        theme.text,
        theme.text,
        theme.icon,
        if show_icons { "block" } else { "none" },
        theme.ring,
        theme.ring,
        get_progress_animation(progress),
    )
}

fn get_progress_animation(progress: f32) -> String {
    format!(
        r#"
      @keyframes rankAnimation {{
        from {{
          stroke-dashoffset: {};
        }}
        to {{
          stroke-dashoffset: {};
        }}
      }}
    "#,
        calculate_circle_progress(0.),
        calculate_circle_progress(progress)
    )
}

fn calculate_circle_progress(value: f32) -> f32 {
    let radius = 40.;
    let c = PI * (radius * 2.);

    let new_value = {
        if value < 0. {
            return 0.;
        };
        if value > 100. {
            return 100.;
        };
        value
    };

    ((100. - new_value) / 100.) * c
}
