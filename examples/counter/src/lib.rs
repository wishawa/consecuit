use reia::{
    components::{button, div, span, ButtonProps, SpanProps},
    hooks::{use_function, use_state, ReiaFunction},
    ComponentBuilder, ContainerReturn, HookBuilder, HookReturn, LeafReturn,
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

fn title(reia: ComponentBuilder, props: i32) -> impl LeafReturn {
    let reia = reia.init();
    let label = format!("Counter value: {}", props);
    reia.node(span, SpanProps { text: label })
}

fn container(reia: ComponentBuilder, _: ()) -> impl ContainerReturn {
    let reia = reia.init();
    reia.node(div, ()).hole_here()
}

fn count_button(
    reia: ComponentBuilder,
    (increment, decrement): (ReiaFunction, ReiaFunction),
) -> impl LeafReturn {
    let reia = reia.init();
    reia.node(button, ButtonProps { onclick: increment })
        .child(|reia| {
            reia.node(
                span,
                SpanProps {
                    text: "Increment Counter".to_string(),
                },
            )
        })
        .sibling(button, ButtonProps { onclick: decrement })
        .child(|reia| {
            reia.node(
                span,
                SpanProps {
                    text: "Decrement Counter".to_string(),
                },
            )
            .sibling(
                span,
                SpanProps {
                    text: "Yay".to_string(),
                },
            )
            /*.child(|reia| reia.node(span, SpanProps {
                text: "Yay2".to_string(),
            }))*/
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

fn app(reia: ComponentBuilder, _: ()) -> impl LeafReturn {
    let reia = reia.init();
    let (reia, (count, increment, decrement)) = reia.hook(use_counter, ());
    reia.node(container, ())
        .child(|reia| {
            reia.node(title, count)
                .sibling(count_button, (increment.clone(), decrement.clone()))
        })
        .sibling(
            span,
            SpanProps {
                text: "Yay2".to_string(),
            },
        )
}
