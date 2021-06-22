use reia::*;
use reia_html::prelude::*;

pub fn app_header(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    reia.init().comp(header, html_props()).child(|r| {
        r.comp(
            h1,
            html_props().class_name("text-opacity-20 text-red-900 text-8xl text-center"),
        )
        .child(|r| r.comp(text_node, "todos"))
    })
}
