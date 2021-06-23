# Reia

An **experimental** functional web UI framework that uses the Rust type system for hooks and more.

## How is this different from other frameworks?

**Other Functional UI Frameworks**:

* Require the developer to follow the [Rule of Hooks](https://reactjs.org/docs/hooks-rules.html).
* Dynamically identify hooks and maintain their states by counting the order in which they are called.
* Dynamic design:

	* Components are dynamically created and mounted as their parents render.
	* Components may be mounted or unmounted depending on state.

**Reia**:

* Automatically enforce the Rule of Hooks using the Rust type system.
* Statically know every hook call that will happen.
* Static-first design:

	* Components are statically mounted at the start of the app and remain mounted forever by default.
	* Mounting/unmounting based on state are considered special cases,
	only available through explicitly using `opt_comp` or `vec_comps`.

## What does it look like?
Take a look at our
[TodoMVC](https://wishawa.github.io/reia/todomvc)
(and see its [source code](https://github.com/wishawa/reia/tree/main/examples/todomvc)).

Or if you want something simpler, here is the code for a counter.

```rust
use reia::*;
use reia_html::prelude::*;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn run() -> Result<(), JsValue> {
    mount(counter);
    Ok(())
}

fn counter(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
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
```
There are more counter examples [here](https://github.com/wishawa/reia/tree/main/examples/counters), including
one without macro and one with logic extracted into a `use_counter` function.



## ⚠ Use of Unsafe ⚠
**Don't panic! Everything publicly exported is safe.**

Reia uses unsafe in two ways:

* To cheat lifetime:

	Functional architecture and web system interop don't play well with lifetime.
	We transmute some internal things to `'static` and use locks to provide safety.

* To erase input type in the function signatures of components and hooks:

	Each Reia component gets a type-erased `ComponentBuilder` as input.
	The actual input type is encoded in the component's `impl Trait` return type.

	The caller (`reia.comp(...)`) and the component must agree on the return type of the component,
	so both know the component's true input type.
	
	The caller erases that type information from the input, but the component knows the type, so it transmutes back correctly.

	Without this trick,
	each hook/component's signature would need to list out every state-slot it and its descendants use.

