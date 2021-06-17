use reia::{
    components::{basic_text_label, button, div, BasicTextLabelProps, ButtonProps, DivProps},
    hooks::{use_function, use_state, ReiaFunction},
    ComponentBuilder, ComponentReturn, ContainerReturn, HookBuilder, HookReturn,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    println!("Hello, world!");
    use std::panic;
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    reia::mount(app);
    Ok(())
}

fn title(reia: ComponentBuilder, props: i32) -> impl ComponentReturn {
    let reia = reia.init();
    let label = format!("Counter value: {}", props);
    reia.node(basic_text_label, BasicTextLabelProps { text: label })
}

fn container(reia: ComponentBuilder, _: ()) -> impl ContainerReturn {
    let reia = reia.init();
    reia.node(div, DivProps {}).hole_here()
}

fn count_button(
    reia: ComponentBuilder,
    (increment, decrement): (ReiaFunction, ReiaFunction),
) -> impl ComponentReturn {
    let reia = reia.init();
    reia.node(button, ButtonProps { onclick: increment })
        .child(|reia| {
            reia.node(
                basic_text_label,
                BasicTextLabelProps {
                    text: "Increment Counter".to_string(),
                },
            )
        })
        .node(button, ButtonProps { onclick: decrement })
        .child(|reia| {
            reia.node(
                basic_text_label,
                BasicTextLabelProps {
                    text: "Decrement Counter".to_string(),
                },
            )
            .node(
                basic_text_label,
                BasicTextLabelProps {
                    text: "Yay".to_string(),
                },
            )
        })
}

fn use_counter(reia: HookBuilder, _: ()) -> impl HookReturn<(i32, ReiaFunction, ReiaFunction)> {
    let reia = reia.init();
    let (reia, (count, count_setter)) = reia.hook(use_state, 1);
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
    let (reia, (count, increment, decrement)) = reia.hook(use_counter, ());
    reia.node(container, ()).child(|reia| {
        reia.node(title, count)
            .node(count_button, (increment.clone(), decrement.clone()))
    })
}
