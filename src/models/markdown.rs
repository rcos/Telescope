use pulldown_cmark::{html::push_html, Options, Parser};

// TODO: test for XSS
/// Render markdown to HTML.
///
/// Should escape properly.
pub fn render(s: &str) -> String {
    let mut output: String = String::new();
    let parser = Parser::new_ext(s, Options::all());
    push_html(&mut output, parser);
    output
}
