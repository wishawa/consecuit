use reia::{
    components::{button, div, span, ButtonProps, DivProps, SpanProps},
    hooks::{use_function, use_state, ReiaFunction},
    ComponentBuilder, ComponentReturn, HookBuilder, HookReturn,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    reia::mount(app);
    Ok(())
}

fn button_with_text(reia: ComponentBuilder, (text, onclick): (String, ReiaFunction)) -> impl ComponentReturn {
    let reia = reia.init();
    reia.node(button, ButtonProps {
        onclick,
    }).child(|reia| reia.node(span, SpanProps {text,}))
}

fn use_counter(reia: HookBuilder, initial: i32) -> impl HookReturn<(i32, ReiaFunction, ReiaFunction)> {
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
    reia.node(div, DivProps {})
        .child(move |reia| {
            reia
            .node(button_with_text, ("-".into(), decrement))
            .sibling(span, SpanProps { text: format!("{}", count)})
            .sibling(button_with_text, ("+".into(), increment))
        })
}
