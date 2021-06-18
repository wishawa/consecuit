use reia::{
    hooks::{use_function, use_state, CallbackFunction},
    ComponentBuilder, ComponentReturn, HookBuilder, HookReturn,
};
use reia_html::components::{button, div, text_node, ButtonProps, DivProps};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    reia::mount(app);
    Ok(())
}

fn button_with_text(
    reia: ComponentBuilder,
    (text, onclick): (String, CallbackFunction),
) -> impl ComponentReturn {
    let reia = reia.init();
    reia.comp(button, ButtonProps { onclick })
        .child(|reia| reia.comp(text_node, text))
}

fn use_counter(
    reia: HookBuilder,
    initial: i32,
) -> impl HookReturn<(i32, CallbackFunction, CallbackFunction)> {
    let reia = reia.init();
    let (reia, (count, count_setter)) = reia.hook(use_state, initial);
    let count_setter_1 = count_setter.clone();
    let (reia, increment) = reia.hook(use_function, move || {
        count_setter_1.update_with(|value| value + 1);
    });
    let (reia, decrement) = reia.hook(use_function, move || {
        count_setter.update_with(|value| value - 1);
    });
    (reia, (count, increment, decrement))
}

fn app(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, (count, increment, decrement)) = reia.hook(use_counter, 0);
    reia.comp(div, DivProps {}).child(move |reia| {
        reia.comp(button_with_text, ("-".into(), decrement))
            .comp(text_node, format!("{}", count))
            .comp(button_with_text, ("+".into(), increment))
    })
}
