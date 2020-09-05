
use pulldown_cmark::{
    Parser,
    Options,
    html::push_html
};

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
