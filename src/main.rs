
#[macro_use]
extern crate log;

use handlebars::Handlebars;

mod env;

#[actix_rt::main]
async fn main() {
    // set up logger and other stuff
    env::init();

    let mut template_registry = Handlebars::new();
    template_registry.register_templates_directory(".hbs", "templates").unwrap();
}
