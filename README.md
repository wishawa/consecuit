# Consecuit

[<img alt="crates.io" src="https://img.shields.io/crates/v/consecuit?style=for-the-badge" height="20">](https://crates.io/crates/consecuit)
[<img alt="crates.io" src="https://img.shields.io/docsrs/consecuit?style=for-the-badge" height="20">](https://docs.rs/consecuit)

An **experimental** functional web UI framework that uses the Rust type system for hooks and more.

## How is this different from other frameworks?

**Other Functional UI Frameworks**:

* Require the developer to follow the [Rule of Hooks](https://reactjs.org/docs/hooks-rules.html) or something similar.
* Dynamically identify components/hooks and maintain their states by counting the order in which they are called.
* Dynamic design:

	* Components are dynamically created and mounted as their parents render.
	* Components may be mounted or unmounted depending on state.

**Consecuit**:

* Automatically enforce the Rule of Hooks using the Rust type system.
* Statically know every component that will be created and every hook call that will happen.
* Static-first design:

	* Components are statically mounted at the start of the app and remain mounted forever by default.
	* Mounting/unmounting based on state are considered special cases available through explicitly using `opt_comp`, `vec_comps`, etc.

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
```

There are more counter examples [here](https://github.com/wishawa/consecuit/tree/main/examples/counters/src/lib.rs)
(with live demo [here](https://wishawa.github.io/consecuit/counters/)),
including one without macro and one with logic extracted into a `use_counter` function.

## How do I start?

Follow our simple guide below:
<details>
<summary>Click to expand guide</summary>

Note: This guide is for you to get started as quickly as possible.
The WASM setup part of the guide is very basic.
You should read the [rustwasm book](https://rustwasm.github.io/wasm-bindgen/introduction.html) later on.

1. Initialize a new lib crate.

    ```shell
    cargo new --lib YOUR_CRATE_NAME_HERE
    cd YOUR_CRATE_NAME_HERE
    ```

1. Add this to your `Cargo.toml`:

    ```
    [lib]
    crate-type = ["cdylib"]

    [dependencies]
    wasm-bindgen = "0.2.74"
    consecuit = "0.2.0"
    consecuit_html = "0.2.0"
    ```

1. Create an `index.html` in the root of your project with this content.

    ```html
    <html>
    <head>
        <meta content="text/html;charset=utf-8" http-equiv="Content-Type" />
    </head>
    <body>
        <script type="module">
            import init from './pkg/YOUR_CRATE_NAME_HERE.js';
            init();
        </script>
    </body>
    </html>
    ```

    (replace `YOUR_CRATE_NAME_HERE` with the name of your crate.)

1. Install `wasm-pack`:

    ```shell
    cargo install wasm-pack
    ```

1. Write your code.
    
    You can copy-paste the [counters example above](https://github.com/wishawa/consecuit#what-does-it-look-like).
            
    Also take a look at [the examples directory](https://github.com/wishawa/consecuit/tree/main/examples/) and [the docs](https://docs.rs/consecuit).

1. Build it!

    ```shell
    wasm-pack build --dev --target web
    ```

1. Serve it!

    ```shell
    # Install a simple web server.
    cargo install microserver
    # And run it!
    microserver
    ```

Final code is in [examples/minimal_counter](https://github.com/wishawa/consecuit/tree/main/examples/minimal_counter).
</details>

[The docs](https://docs.rs/consecuit) have more info on creating components and hooks.

## What's next?

This library is still in an early stage. A lot of features are missing.
(For example, inline styling is not available right now; you need to use class names and write CSS.)

Contributions are welcomed!

**This crate uses unsafe.**

All publicly exposed functions are safe.