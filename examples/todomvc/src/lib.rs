use reia::*;
use reia_html::prelude::*;
use wasm_bindgen::prelude::*;
mod app_header;
use app_header::app_header;
mod app_main;
use app_main::app_main;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    println!("Hello, world!");
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    reia::mount(app);
    Ok(())
}

fn container(reia: ComponentBuilder, _: ()) -> impl ContainerReturn {
    reia.init()
        .comp(
            div,
            html_props().class_name("container mx-auto flex flex-col items-center"),
        )
        .hole_here()
}

fn sections(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    reia.init()
        .comp(header, html_props())
        .child(|r| r.comp(app_header, ()))
        .comp(app_main, ())
}

fn app(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    reia.init()
        .comp(container, ())
        .child(|r| r.comp(sections, ()))
}
