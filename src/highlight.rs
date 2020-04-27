use syntect::parsing::{SyntaxSet, SyntaxReference, Scope};
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_string;
use std::collections::HashMap;

pub fn highlight(content: &str) -> String {
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();

    let style = "
        pre {
            font-size:13px;
            font-family: Consolas, \"Liberation Mono\", Menlo, Courier, monospace;
        }";
    let theme = &ts.themes["base16-ocean.dark"];
    let sref = ss.find_syntax_by_extension("rs").unwrap();
    let c = theme.settings.background.unwrap_or(Color::WHITE);
    let html = highlighted_html_for_string(content, &ss, &sref, theme);

    format!(r#"
        <body style="background-color:#{:02x}{:02x}{:02x};">
        {}
        </body>
    "#, c.r, c.g, c.b, html)
}
