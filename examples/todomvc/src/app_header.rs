use reia::*;
use reia_html::prelude::*;

pub fn app_header(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    reia.init()
        .comp(
            h1,
            html_props().class_name("text-red-600 text-8xl text-center"),
        )
        .child(|r| r.comp(text_node, "todos"))
}
