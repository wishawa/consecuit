/*!
An experimental functional web UI framework that uses the Rust type system for hooks and more.

This crate is a library for building web UI with functional components and hooks.

See the [README](https://github.com/wishawa/reia) for an overview.

Reia uses functional components and hooks, not unlike React and many other libraries.

## Hooks

A Reia hook is a function that can call other Reia hooks.
So you can build your own hook by composing the basic hooks like [`use_state`][hooks::use_state], [`use_ref`][hooks::use_ref], and [`use_effect`][hooks::use_effect].

A Reia hook must be a function that takes 2 arguments:
Reia's [`HookBuilder`][construction::hook::HookBuilder], and the args, which can be any type.

The return type should not be named out concretely, but rather written as `impl Trait` of [`HookReturn`][construction::types::HookReturn], with the actual value as the generic parameter.

The signature for a `use_i32_state` hook might look something like this

```
use reia::prelude::*;
fn use_i32_state(reia: HookBuilder, initial_value: i32) -> impl HookReturn<(i32, Updater<i32>)>
```

In the function body, you can write code just like a normal function.

To use other hooks, call `reia.hook(<hook function>, <hook args>)`.

This will consume `reia` and return a new one, along with the return value of the hook, in a tuple like this `(reia. <hook return value>)`.

You then can do whatever you want with the `<hook return value>`, and you can use the use the returned `reia` to call more hooks.

```
let (reia, some_hook_result) = reia.hook(some_hook_function, some_hook_args);
println!("{:?}", some_hook_result + 1);
let (reia, another_hook_result) = reia.hook(another_hook_function, another_hook_args);
```

When you are done and want to return from your hook, return `(reia, <your hook return value>)`.

```
(reia, some_value)
```

Our `use_i32_state` might look like this in full:

```
use reia::prelude::*;
fn use_i32_state(reia: HookBuilder, initial_value: i32) -> impl HookReturn<(i32, Updater<i32>)> {
    // Use the `use_state` hook.
    let (reia, (number, updater)) = reia.hook(use_state, initial_value);
    // Print the value.
    prinln!("The state is {}", number.to_string());
    // Return
    (reia, (number, updater))
}
```

Take a look at [the counters example](https://github.com/wishawa/reia/tree/main/examples/counters/src/lib.rs),
specifically the function `counter_hook_extracted`, to see how we extract logic into a new hook.

## Components

A Reia component is a function that can call Reia hooks, and render other Reia components.
You can use the HTML components provided by the reia_html crate as building blocks for your components' UI.

A Reia component must be a function that takes 2 arguments:
Reia's [`ComponentBuilder`][construction::component::ComponentBuilder], and the props, which can be any type that is [`PartialEq`] and [`Clone`], and `'static` (i.e. not a borrow).
(a component can only have 1 props).

The return type should not be named out concretely, but rather written as `impl Trait` of [`ComponentReturn`][construction::types::ComponentReturn].
Here is an example:

```
use reia::prelude::*;
fn my_component(reia: ComponentBuilder, props: MyComponentProps) -> impl ComponentReturn
```

In the function body, you can write normal code, and call hooks the same way as for hook functions documented above.

You can also use the [`reia_tree`][reia_macros::reia_tree] macro to render other components. Like this:

```
reia_tree!(
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

As you might have figurede out from the code above already:

* Use the name of the component function is the name of the tag.
* Put the props in a curly braces beside the tag name.
If there are none, the macro will attempt to use the [`Default::default`] value (and error if the props doesn't implement Default).
* Use string literal or any [`AsRef<str>`] type in braces to create text node.

Here is an example of a full component function:

```
fn show_plus_calculation(reia: ComponentBuilder, (lhs, rhs): (i32, i32)) -> impl ComponentReturn {
    let result = lhs + rhs;
    reia_tree!(
        <h5>"Calculation Result:"</h5>
        <span>{lhs.to_string()}</span>
        <span {html_props().class_name("plus-sign")}>" + "</span>
        <span>{rhs.to_string()}</span>
        <span {html_props().class_name("equal-sign")}>" = "</span>
        <span><b>{result.to_string()}</b></span>
    )
}
```

## Container Components

Container components are components with 'hole', allowing you to give it children.
Most components in the `reia_html` crate are containers.

Container components return `impl ContainerReturn` rather than `impl ComponentReturn`

For example, you can give div and span childrens:

```
reia_tree!(
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
fn my_container(reia: ComponentBuilder, _: ()) -> impl ContainerReturn {
    reia_tree!(
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
pub use reia_macros;

pub mod prelude {
    /*! Just import me.

    */
    pub use crate::construction::{
        component::ComponentBuilder,
        hook::HookBuilder,
        mount,
        subtrees::*,
        types::{ComponentReturn, ContainerReturn, DynComponentReturn, HookReturn},
    };
    pub use crate::executor::{batch_updates, run_later};
    pub use crate::hooks::*;
    pub use reia_macros::reia_tree;
}
