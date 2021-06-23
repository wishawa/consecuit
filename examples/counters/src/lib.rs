use reia::*;
use reia_html::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    mount_app(counters);
    Ok(())
}

fn counters(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    reia_tree!(
        <table>
            <tr>
                <td>
                    "basic"
                </td>
                <td>
                    <counter_basic />
                </td>
            </tr>
            <tr>
                <td>
                    "no macro"
                </td>
                <td>
                    <counter_no_macro />
                </td>
            </tr>
            <tr>
                <td>
                    "with memo"
                </td>
                <td>
                    <counter_memo />
                </td>
            </tr>
            <tr>
                <td>
                    "with hook extracted"
                </td>
                <td>
                    <counter_hook_extracted />
                </td>
            </tr>
        </table>
    )
}

fn counter_basic(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let (reia, (count, setter)) = reia.hook(use_state, 0);

    let setter1 = setter.clone();
    let decrement = Callback::new(move |_ev| {
        setter1.update_with(|v| v - 1);
    });

    let increment = Callback::new(move |_ev| {
        setter.update_with(|v| v + 1);
    });

    reia_tree!(
        <button {html_props().onclick(decrement)}>"-"</button>
        {count.to_string()}
        <button {html_props().onclick(increment)}>"+"</button>
    )
}

fn counter_no_macro(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let (reia, (count, setter)) = reia.hook(use_state, 0);

    let setter1 = setter.clone();
    let decrement = Callback::new(move |_ev| {
        setter1.update_with(|v| v - 1);
    });

    let increment = Callback::new(move |_ev| {
        setter.update_with(|v| v + 1);
    });

    reia.comp(button, html_props().onclick(decrement))
        .child(|r| r.comp(text_node, "-"))
        .comp(text_node, count.to_string())
        .comp(button, html_props().onclick(increment))
        .child(|r| r.comp(text_node, "+"))
}

fn counter_memo(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let (reia, (count, setter)) = reia.hook(use_state, 0);

    let (reia, decrement) = reia.hook(
        use_memo,
        (
            |setter: Updater<i32>| {
                Callback::new(move |_ev| {
                    setter.update_with(|v| v - 1);
                })
            },
            setter.clone(),
        ),
    );

    let (reia, increment) = reia.hook(
        use_memo,
        (
            |setter: Updater<i32>| {
                Callback::new(move |_ev| {
                    setter.update_with(|v| v + 1);
                })
            },
            setter.clone(),
        ),
    );

    reia_tree!(
        <button {html_props().onclick(decrement)}>"-"</button>
        {count.to_string()}
        <button {html_props().onclick(increment)}>"+"</button>
    )
}

fn counter_hook_extracted(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    fn use_counter(
        reia: HookBuilder,
        _: (),
    ) -> impl HookReturn<(
        i32,
        Callback<web_sys::MouseEvent>,
        Callback<web_sys::MouseEvent>,
    )> {
        let (reia, (count, setter)) = reia.hook(use_state, 0);
        let setter1 = setter.clone();
        let decrement = Callback::new(move |_ev| {
            setter1.update_with(|v| v - 1);
        });

        let increment = Callback::new(move |_ev| {
            setter.update_with(|v| v + 1);
        });
        (reia, (count, decrement, increment))
    }
    let (reia, (count, decrement, increment)) = reia.hook(use_counter, ());
    reia_tree!(
        <button {html_props().onclick(decrement)}>"-"</button>
        {count.to_string()}
        <button {html_props().onclick(increment)}>"+"</button>
    )
}
