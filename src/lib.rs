/*!

# Consecuit
An experimental functional web UI framework that uses the Rust type system for hooks and more.

---

This crate is a library for building web UI with functional components and hooks.

See the [README](https://github.com/wishawa/consecuit) for an overview.

Also take a look at [our TodoMVC](https://wishawa.github.io/consecuit/todomvc) and [its source code](https://github.com/wishawa/consecuit/tree/main/examples/todomvc).

# Using Consecuit

Below, we show you how to

* Write and use hooks.
* Write and use components.
* Write and use container components.

The [the counters example](https://github.com/wishawa/consecuit/tree/main/examples/counters/src/lib.rs) is also good for getting started.

## Hooks

A Consecuit hook is a function that can call other Consecuit hooks.
So you can build your own hook by composing the basic hooks like [`use_state`][hooks::use_state], [`use_ref`][hooks::use_ref], and [`use_effect`][hooks::use_effect].

A Consecuit hook must be a function that takes 2 arguments:
Consecuit's [`HookBuilder`][construction::hook::HookBuilder], and the args, which can be any type.

The return type should not be named out concretely, but rather written as `impl Trait` of [`HookReturn`][construction::types::HookReturn], with the actual value as the generic parameter.

The signature for a `use_i32_state` hook might look something like this

```
use consecuit::prelude::*;
fn use_i32_state(cc: HookBuilder, initial_value: i32) -> impl HookReturn<(i32, Updater<i32>)>
```

In the function body, you can write code just like a normal function.

To use other hooks, call `cc.hook(<hook function>, <hook args>)`.

This will consume `cc` and return a new one, along with the return value of the hook, in a tuple like this `(cc. <hook return value>)`.

You then can do whatever you want with the `<hook return value>`, and you can use the use the returned `cc` to call more hooks.

```
let (cc, some_hook_result) = cc.hook(some_hook_function, some_hook_args);
println!("{:?}", some_hook_result + 1);
let (cc, another_hook_result) = cc.hook(another_hook_function, another_hook_args);
```

When you are done and want to return from your hook, return `(cc, <your hook return value>)`.

```
(cc, some_value)
```

Our `use_i32_state` might look like this in full:

```
use consecuit::prelude::*;
fn use_i32_state(cc: HookBuilder, initial_value: i32) -> impl HookReturn<(i32, Updater<i32>)> {
    // Use the `use_state` hook.
    let (cc, (number, updater)) = cc.hook(use_state, initial_value);
    // Print the value.
    prinln!("The state is {}", number.to_string());
    // Return
    (cc, (number, updater))
}
```

Take a look at [the counters example](https://github.com/wishawa/consecuit/tree/main/examples/counters/src/lib.rs),
specifically the function `counter_hook_extracted`, to see how we extract logic into a new hook.

## Components

A Consecuit component is a function that can call Consecuit hooks, and render other Consecuit components.
You can use the HTML components provided by the consecuit_html crate as building blocks for your components' UI.

A Consecuit component must be a function that takes 2 arguments:
Consecuit's [`ComponentBuilder`][construction::component::ComponentBuilder], and the props, which can be any type that is [`PartialEq`] and [`Clone`], and `'static` (i.e. not a borrow).
(a component can only have 1 props).

The return type should not be named out concretely, but rather written as `impl Trait` of [`ComponentReturn`][construction::types::ComponentReturn].
Here is an example:

```
use consecuit::prelude::*;
fn my_component(cc: ComponentBuilder, props: MyComponentProps) -> impl ComponentReturn
```

In the function body, you can write normal code, and call hooks the same way as for hook functions documented above.

You can also use the [`cc_tree`][consecuit_macros::cc_tree] macro to render other components. Like this:

```
use consecuit_html::prelude::*; // <- consecuit_html provides html components like div and span
cc_tree!(
    <div>
        <div {html_props().class_name("some-class-name").onclick(onclick_callback)}>
            <span>"I'm a string literal"</span>
            <span {html_props().class_name("span-text")}>{format!("anything that is AsRef<str> works")}</span>
            {props.some_text}
            <my_component { MyComponentProps {text: "hello"} } />
        </div>
        <div {html_props().class_name("nothing-here")} />
    </div>
)
```

As you might have figured out from the code above already:

* Use the name of the component function is the name of the tag.
* Put the props in curly braces beside the tag name.
If there are none, the macro will attempt to use the [`Default::default`] value (and error if the props doesn't implement Default).
* Use string literal or any [`AsRef<str>`] type in braces to create text node.

Here is an example of a component function:

```
use consecuit::prelude::*;
use consecuit_html::prelude::*;
fn show_plus_calculation(cc: ComponentBuilder, (lhs, rhs): (i32, i32)) -> impl ComponentReturn {
    let result = lhs + rhs;

    // These 2 lines are just to show how to use hooks
    let (cc, _some_example_1) = cc.hook(use_example_hook, 1234);
    let (cc, _some_example_2) = cc.hook(use_another_hook, 5678);

    // All there `h5`, `span`, `b` are from the `consecuit_html` crate.
    cc_tree!(
        <my_website_header />
        <h5>"Calculation Result:"</h5>
        <span>{lhs.to_string()}</span>
        <span {html_props().class_name("plus-sign")}>" + "</span>
        <span>{rhs.to_string()}</span>
        <span {html_props().class_name("equal-sign")}>" = "</span>
        <span><b>{result.to_string()}</b></span>
        <my_website_footer />
    )
}
```

## Container Components

Container components are components with 'hole', allowing you to give it children.
Most components in the `consecuit_html` crate are containers.

Container components return `impl ContainerReturn` rather than `impl ComponentReturn`

For example, you can give div and span childrens:

```
cc_tree!(
    <div>
        "This is a child I put in the div"
        "This is another child I put in the div"
        <span>"This is a child I put in the span in the div"</span>
    </div>
)
```

To create your own container component, add the `HOLE` attribute to the component you wish to hole.
This will "forward" the hole of that component.

For example:

```
fn my_container(cc: ComponentBuilder, _: ()) -> impl ContainerReturn {
    cc_tree!(
        <div>
            "The hole of the div below is forwarded to become the hole of my_container!"
            <div HOLE />
        </div>
    )
}
```

A container component must have exactly one hole.
*/

pub mod construction;
pub mod executor;
pub mod hooks;
pub mod locking;
mod stores;

pub use consecuit_macros;

pub use construction::mount;

pub mod prelude {
    /*! Just import me.

    */
    pub use crate::construction::{
        component::ComponentBuilder,
        hook::HookBuilder,
        subtrees::*,
        types::{ComponentReturn, ContainerReturn, DynComponentReturn, HookReturn},
    };
    pub use crate::executor::{batch_updates, run_later};
    pub use crate::hooks::*;
    pub use consecuit_macros::cc_tree;
}
