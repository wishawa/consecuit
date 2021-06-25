use consecuit::prelude::*;
use consecuit_html::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    consecuit::mount::mount_app(counter);
    Ok(())
}

fn counter(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
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
