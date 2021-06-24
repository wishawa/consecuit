use reia::prelude::*;
use reia_html::prelude::*;

pub fn app_header(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    reia_tree!(
        <h1 {html_props().class_name("text-opacity-20 text-red-900 text-8xl text-center")}>
            "todos"
        </h1>
    )
}
