use consecuit::prelude::*;
use consecuit_html::prelude::*;

pub fn app_header(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    cc_tree!(
        <h1 {html_props().class_name("text-opacity-20 text-red-900 text-8xl text-center")}>
            "todos"
        </h1>
    )
}
