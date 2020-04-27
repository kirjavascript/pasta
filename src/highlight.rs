use syntect::parsing::{SyntaxSet};
use syntect::highlighting::{Color, ThemeSet};
use syntect::html::highlighted_html_for_string;
use std::collections::HashMap;

pub fn highlight(content: &str, filename: &str) -> String {
    let syntax = SyntaxSet::load_defaults_newlines();
    let theme = ThemeSet::load_defaults();

    let theme = &theme.themes["base16-ocean.dark"];
    let extension = crate::file::extension(filename).unwrap_or_else(|| "".to_string());

    if let Some(reference) = syntax.find_syntax_by_extension(&extension) {
        let color = theme.settings.background.unwrap_or(Color::WHITE);
        let html = highlighted_html_for_string(content, &syntax, &reference, theme);

        format!(r#"
            <style>
                pre {{
                    font-size: 13px;
                    font-family: Consolas, "Liberation Mono", Menlo, Courier, monospace;
                }}
            </style>
            <body style="background-color:#{:02x}{:02x}{:02x};">
            {}
            </body>
        "#, color.r, color.g, color.b, html)
    } else {
        format!(r#"
            <pre>{}</pre>
        "#, content)
    }
}
