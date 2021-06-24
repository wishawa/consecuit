# Consecuit

[<img alt="crates.io" src="https://img.shields.io/crates/v/consecuit?style=for-the-badge" height="20">](https://crates.io/crates/consecuit)
[<img alt="crates.io" src="https://img.shields.io/docsrs/consecuit?style=for-the-badge" height="20">](https://docs.rs/consecuit)

An **experimental** functional web UI framework that uses the Rust type system for hooks and more.

## How is this different from other frameworks?

**Other Functional UI Frameworks**:

* Require the developer to follow the [Rule of Hooks](https://reactjs.org/docs/hooks-rules.html).
* Dynamically identify hooks and maintain their states by counting the order in which they are called.
* Dynamic design:

	* Components are dynamically created and mounted as their parents render.
	* Components may be mounted or unmounted depending on state.

**Consecuit**:

* Automatically enforce the Rule of Hooks using the Rust type system.
* Statically know every hook call that will happen.
* Static-first design:

	* Components are statically mounted at the start of the app and remain mounted forever by default.
	* Mounting/unmounting based on state are considered special cases,
	only available through explicitly using `opt_comp` or `vec_comps`.

## What does it look like?
Take a look at our
[TodoMVC](https://wishawa.github.io/consecuit/todomvc)
(and see [its source code](https://github.com/wishawa/consecuit/tree/main/examples/todomvc)).

Or if you want something simpler, here is the code for a counter.

```rust
use consecuit::prelude::*;
use consecuit_html::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    mount(counter);
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
```
There are more counter examples [here](https://github.com/wishawa/consecuit/tree/main/examples/counters), including
one without macro and one with logic extracted into a `use_counter` function.

[The docs](https://docs.rs/consecuit) have more info on creating components and hooks.

**This crate uses unsafe.**
Don't worry. All publicly exposed function is safe.