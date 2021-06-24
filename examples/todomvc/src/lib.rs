use consecuit::prelude::*;
use consecuit_html::prelude::*;
use wasm_bindgen::prelude::*;
mod app_header;
use app_header::app_header;
mod app_main;
use app_main::app_main;
mod app_footer;
use app_footer::app_footer;
mod model;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    println!("Hello, world!");
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    consecuit::mount::mount_app(app);
    Ok(())
}

fn container(cc: ComponentBuilder, _: ()) -> impl ContainerReturn {
    cc_tree!(
        <div {html_props().class_name("bg-gray-50 min-h-screen font-sans")}>
            <div {html_props().class_name("container mx-auto flex flex-col items-center")} HOLE />
        </div>
    )
}

fn sections(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    cc_tree!(
        <app_header />
        <app_main />
        <app_footer />
    )
}

fn app(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    cc_tree!(
        <container>
            <sections />
        </container>
    )
}
