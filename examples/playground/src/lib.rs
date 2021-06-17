use reia::{
    components::{
        basic_text_label, button, div, dyn_vec_comps, text_node, BasicTextLabelProps, ButtonProps,
        DivProps,
    },
    hooks::{use_effect, use_function, use_state, ReiaFunction, StateSetter},
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

fn use_counter(
    reia: HookBuilder,
    level_setter: StateSetter<i32>,
) -> impl HookReturn<(i32, ReiaFunction, ReiaFunction)> {
    let reia = reia.init();
    let (reia, (count, count_setter)) = reia.hook(use_state, 0);
    let count_setter_1 = count_setter.clone();
    let (reia, increment) = reia.hook(use_function, move || {
        count_setter_1.update_with(|value| value + 1);
    });
    let count_setter_2 = count_setter.clone();
    let (reia, decrement) = reia.hook(use_function, move || {
        count_setter_2.update_with(|value| value - 1);
    });
    let (reia, _) = reia.hook(
        use_effect,
        (
            &|(count, count_setter, level_setter): (i32, StateSetter<i32>, StateSetter<i32>)| {
                if count > 15 {
                    count_setter.set(0);
                    level_setter.update_with(|lvl| lvl + 1);
                } else if count < -15 {
                    count_setter.set(0);
                    level_setter.update_with(|lvl| lvl.max(1) - 1);
                }
            },
            (count, count_setter, level_setter),
        ),
    );
    (reia, (count, increment, decrement))
}

fn level_history(reia: ComponentBuilder, level: i32) -> impl ComponentReturn {
    let reia = reia.init();
    reia.node(text_node, format!("You Reached Level: {}", level))
}

fn levels_history(reia: ComponentBuilder, level: i32) -> impl ComponentReturn {
    let reia = reia.init();
    reia.node(text_node, format!("Current level: {}\nHistory:", level))
        .node(dyn_vec_comps, (level_history, (1..=level.max(0)).collect()))
}

fn app(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, (level, level_setter)) = reia.hook(use_state, 0);
    let (reia, (count, increment, decrement)) = reia.hook(use_counter, level_setter);
    reia.node(container, ()).child(|reia| {
        reia.node(title, count)
            .node(count_button, (increment.clone(), decrement.clone()))
            .node(levels_history, level)
    })
}
