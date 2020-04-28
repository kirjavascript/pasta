use syntect::easy::HighlightLines;
use syntect::parsing::{SyntaxSet, SyntaxReference};
use syntect::highlighting::{Color, ThemeSet, Theme};
use syntect::util::LinesWithEndings;
use syntect::html::*;
use once_cell::sync::Lazy;

static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());
static THEME_SET: Lazy<ThemeSet> = Lazy::new(|| ThemeSet::load_defaults());

pub fn highlight(content: &str, filename: &str) -> String {
    let syntax = &SYNTAX_SET;
    let theme = &THEME_SET;

    let theme = &theme.themes["base16-ocean.dark"];
    let extension = crate::file::extension(filename).unwrap_or_else(|| "txt".to_string());
    let reference = syntax.find_syntax_by_extension(&extension)
            .unwrap_or_else(|| syntax.find_syntax_by_extension("txt").unwrap());

    let color = theme.settings.background.unwrap_or(Color::WHITE);
    let (html, line_numbers) = highlighted_html_for_string(
        content,
        &syntax,
        &reference,
        theme
    );

    format!(r#"
            <html>
                <head>
                    <meta charset="UTF-8" />
                    <meta name="viewport" content="width=device-width" />
                    <title>{}</title>
                </head>
                <style>
                    * {{
                        box-sizing: border-box;
                    }}
                    body {{
                        margin: 0;
                    }}
                    pre, .lineNumber {{
                        font-size: 13px;
                        font-family: Consolas, "Liberation Mono", Menlo, Courier, monospace;
                    }}
                    pre, .lineNumbers {{
                        margin: 0;
                        padding: 5px 0;
                    }}
                    main {{
                        display: flex;
                    }}
                    .lineNumber {{
                        display: block;
                        user-select: none;
                        padding-right: 10px;
                        color: #96b5b4;
                    }}
                    .lineNumbers {{
                        width: 50px;
                        margin-right: 15px;
                        text-align: right;
                        background-color: #445;
                    }}
                </style>
                <body style="background-color:#{:02x}{:02x}{:02x};">
                    <main>
                        <div class="lineNumbers">{}</div>
                        <div>{}</div>
                    </main>
                </body>
            </html>
        "#, title(content), color.r, color.g, color.b, line_numbers, html)
}

pub fn highlighted_html_for_string(s: &str, ss: &SyntaxSet, syntax: &SyntaxReference, theme: &Theme) -> (String, String) {
    let mut highlighter = HighlightLines::new(syntax, theme);
    let (mut output, bg) = start_highlighted_html_snippet(theme);
    let mut line_numbers = String::new();

    for (i, line) in LinesWithEndings::from(s).enumerate() {
        let regions = highlighter.highlight(line, ss);
        append_highlighted_html_for_styled_line(&regions[..], IncludeBackground::IfDifferent(bg), &mut output);
        line_numbers.push_str(&format!(r#"<span class="lineNumber">{}</span>"#, i + 1));
    }
    output.push_str("</pre>\n");
    (output, line_numbers)
}

fn title(content: &str) -> String {
    let max_length = 100;
    let mut title = content[..content.len().min(max_length)].to_string();
    if title.len() == max_length {
        title.push_str("...");
    }
    title.replace("&", "&amp;").replace("<", "&lt;")
}

pub fn _print_extensions() {
    println!("{}",
        SyntaxSet::load_defaults_newlines()
            .syntaxes()
            .iter()
            .map(|x| x.file_extensions.join(","))
            .collect::<Vec<_>>()
            .join(",")
    );
}
