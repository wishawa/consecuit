use reia::*;
use reia_html::prelude::*;
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
    (text, onclick): (String, Callback),
) -> impl ComponentReturn {
    let reia = reia.init();
    reia.comp(button, HtmlProps::new().onclick(onclick))
        .child(|reia| reia.comp(text_node, text))
}

fn use_counter(reia: HookBuilder, initial: i32) -> impl HookReturn<(i32, Callback, Callback)> {
    let reia = reia.init();
    let (reia, (count, count_setter)) = reia.hook(use_state, initial);
    let (reia, increment) = reia.hook(
        use_memo,
        (
            |count_setter: StateSetter<i32>| {
                Callback::new(move || {
                    count_setter.update_with(|value| value + 1);
                })
            },
            count_setter.clone(),
        ),
    );
    let (reia, decrement) = reia.hook(
        use_memo,
        (
            |count_setter: StateSetter<i32>| {
                Callback::new(move || {
                    count_setter.update_with(|value| value - 1);
                })
            },
            count_setter.clone(),
        ),
    );
    (reia, (count, increment, decrement))
}

fn app(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, (count, increment, decrement)) = reia.hook(use_counter, 0);
    reia.comp(div, HtmlProps::new()).child(move |reia| {
        reia.comp(button_with_text, ("-".into(), decrement))
            .comp(text_node, format!("{}", count))
            .comp(button_with_text, ("+".into(), increment))
    })
}
