use consecuit::prelude::*;
use consecuit_html::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    mount::mount_app(counters);
    Ok(())
}

fn counters(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    cc_tree!(
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

fn counter_basic(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let (cc, (count, setter)) = cc.hook(use_state, 0);

    let setter1 = setter.clone();
    let decrement = Callback::new(move |_ev| {
        setter1.update_with(|v| v - 1);
    });

    let increment = Callback::new(move |_ev| {
        setter.update_with(|v| v + 1);
    });

    cc_tree!(
        <button {html_props().onclick(decrement)}>"-"</button>
        {count.to_string()}
        <button {html_props().onclick(increment)}>"+"</button>
    )
}

fn counter_no_macro(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let (cc, (count, setter)) = cc.hook(use_state, 0);

    let setter1 = setter.clone();
    let decrement = Callback::new(move |_ev| {
        setter1.update_with(|v| v - 1);
    });

    let increment = Callback::new(move |_ev| {
        setter.update_with(|v| v + 1);
    });

    cc.comp(button, html_props().onclick(decrement))
        .child(|cc| cc.comp(text_node, "-"))
        .comp(text_node, count.to_string())
        .comp(button, html_props().onclick(increment))
        .child(|cc| cc.comp(text_node, "+"))
}

fn counter_memo(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let (cc, (count, setter)) = cc.hook(use_state, 0);

    let (cc, decrement) = cc.hook(
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

    let (cc, increment) = cc.hook(
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

    cc_tree!(
        <button {html_props().onclick(decrement)}>"-"</button>
        {count.to_string()}
        <button {html_props().onclick(increment)}>"+"</button>
    )
}

fn counter_hook_extracted(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    fn use_counter(
        cc: HookBuilder,
        _: (),
    ) -> impl HookReturn<(
        i32,
        Callback<web_sys::MouseEvent>,
        Callback<web_sys::MouseEvent>,
    )> {
        let (cc, (count, setter)) = cc.hook(use_state, 0);
        let setter1 = setter.clone();
        let decrement = Callback::new(move |_ev| {
            setter1.update_with(|v| v - 1);
        });

        let increment = Callback::new(move |_ev| {
            setter.update_with(|v| v + 1);
        });
        (cc, (count, decrement, increment))
    }
    let (cc, (count, decrement, increment)) = cc.hook(use_counter, ());
    cc_tree!(
        <button {html_props().onclick(decrement)}>"-"</button>
        {count.to_string()}
        <button {html_props().onclick(increment)}>"+"</button>
    )
}
