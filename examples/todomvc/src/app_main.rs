use im_rc::Vector;
use reia::*;
use reia_html::prelude::*;

use crate::model::{todos_from_str, todos_to_string, Todo, TodosReducer, TodosReduction};

fn use_text_edit(
    reia: HookBuilder,
    (submit, escape): (
        impl Fn(String) + Clone + 'static,
        impl Fn() + Clone + 'static,
    ),
) -> impl HookReturn<HtmlProps<web_sys::HtmlInputElement>> {
    let reia = reia.init();
    let (reia, input_ref) = reia.hook(use_ref, ());
    let cloned_input_ref = input_ref.clone();
    let submit = move || {
        let text = cloned_input_ref
            .visit_with(|opt: &Option<web_sys::HtmlInputElement>| {
                let inp = opt.as_ref().unwrap();
                let value = inp.value();
                inp.set_value("");
                value.trim().to_string()
            })
            .unwrap();
        submit(text);
    };
    let submit_cloned = submit.clone();
    let on_keypress = Callback::new(move |ev: web_sys::KeyboardEvent| match ev.key_code() {
        13 => submit_cloned(),
        27 => escape(),
        _ => {}
    });
    let submit_cloned = submit.clone();
    let on_blur = Callback::new(move |_ev| {
        submit_cloned();
    });
    let (reia, _) = reia.hook(
        use_effect,
        (
            |input_ref: ReiaRef<Option<web_sys::HtmlInputElement>>| {
                run_later(move || {
                    input_ref
                        .visit_with(|opt| opt.as_ref().unwrap().focus().unwrap())
                        .unwrap();
                });
                || {}
            },
            input_ref.clone(),
        ),
    );
    let props = HtmlProps::<web_sys::HtmlInputElement>::new()
        .reference(input_ref)
        .onkeydown(on_keypress)
        .onblur(on_blur);
    (reia, props)
}

fn top_box(
    reia: ComponentBuilder,
    (todos, reducer): (Vector<Todo>, TodosReducer),
) -> impl ComponentReturn {
    let reia = reia.init();
    let cloned_reducer = reducer.clone();
    let is_empty = todos.is_empty();
    let all_completed = todos.iter().all(|td| td.completed);
    let on_toggle_all = Callback::new(move |_ev| {
        cloned_reducer.reduce(TodosReduction::ToggleAll(!all_completed));
    });

    let (reia, input_props) = reia.hook(
        use_text_edit,
        (
            move |text: String| {
                if !text.is_empty() {
                    reducer.reduce(TodosReduction::Add(text));
                }
            },
            || {},
        ),
    );

    reia.comp(div, html_props().class_name("flex flex-row text-2xl h-16"))
        .child(|r| {
            r.comp(
                button,
                html_props().onclick(on_toggle_all).class_name({
                    if is_empty {
                        "w-12 invisible disabled"
                    } else if all_completed {
                        "w-12 text-green-500"
                    } else {
                        "w-12 text-gray-400"
                    }
                }),
            )
            .child(|r| r.comp(text_node, if all_completed { "▼" } else { "▽" }))
            .comp(
                input,
                input_props
                    .class_name("flex-1 min-w-0 p-2 placeholder-gray-300")
                    .placeholder("What needs to be done?"),
            )
        })
}

fn one_todo(
    reia: ComponentBuilder,
    (todo, reducer, idx): (Todo, TodosReducer, usize),
) -> impl ComponentReturn {
    let reia = reia.init();
    let reducer_cloned = reducer.clone();
    let toggle = Callback::new(move |_ev| {
        reducer_cloned.reduce(TodosReduction::Toggle(idx));
    });
    let reducer_cloned = reducer.clone();
    let remove = Callback::new(move |_ev| {
        reducer_cloned.reduce(TodosReduction::Remove(idx));
    });
    let (reia, (edit, edit_setter)) = reia.hook(use_state, false);
    let edit_setter_cloned = edit_setter.clone();
    let enter_edit = Callback::new(move |_ev| {
        edit_setter_cloned.set(true);
    });
    reia.comp(
        div,
        html_props().class_name("border-t-2 border-gray-100 text-2xl"),
    )
    .child(|r| {
        r.comp(
            div,
            html_props().class_name(if edit {
                "hidden"
            } else {
                "flex flex-row group"
            }),
        )
        .child(|r| {
            r.comp(
                button,
                html_props().onclick(toggle).class_name({
                    if todo.completed {
                        "flex-shrink-0 w-12 text-green-500"
                    } else {
                        "flex-shrink-0 w-12 text-gray-400"
                    }
                }),
            )
            .child(|r| r.comp(text_node, if todo.completed { "☑" } else { "☐" }))
            .comp(
                label,
                html_props().ondblclick(enter_edit).class_name({
                    const SHARED_CLASS: &str = "flex-1 min-w-0 flex items-center break-all p-2";
                    if todo.completed {
                        format!("{} text-gray-500 line-through", SHARED_CLASS)
                    } else {
                        format!("{} text-gray-800", SHARED_CLASS)
                    }
                }),
            )
            .child(|r| r.comp(text_node, todo.name.clone()))
            .comp(
                button,
                html_props().onclick(remove).class_name(
                    "flex-shrink-0 w-12 text-red-700 opacity-0 group-hover:opacity-100",
                ),
            )
            .child(|r| r.comp(text_node, "🗴"))
        })
        .comp(
            opt_comp,
            (
                todo_edit,
                if edit {
                    Some((todo, reducer, idx, edit_setter))
                } else {
                    None
                },
            ),
        )
    })
}

fn todo_edit(
    reia: ComponentBuilder,
    (todo, reducer, idx, edit_setter): (Todo, TodosReducer, usize, Updater<bool>),
) -> impl ComponentReturn {
    let reia = reia.init();
    let edit_setter_cloned = edit_setter.clone();
    let (reia, input_props) = reia.hook(
        use_text_edit,
        (
            move |text: String| {
                if text.is_empty() {
                    reducer.reduce(TodosReduction::Remove(idx));
                } else {
                    reducer.reduce(TodosReduction::SetName(idx, text));
                }
                edit_setter_cloned.set(false);
            },
            move || {
                edit_setter.set(false);
            },
        ),
    );
    reia.comp(div, html_props().class_name("flex flex-row"))
        .child(|r| {
            r.comp(div, html_props().class_name("w-12")).comp(
                input,
                input_props
                    .value(todo.name)
                    .class_name("flex-1 p-2 shadow-inner"),
            )
        })
}

#[derive(Copy, Clone, PartialEq)]
enum ListFilter {
    All,
    Active,
    Completed,
}

impl ListFilter {
    fn filter(&self, td: &Todo) -> bool {
        let completed = td.completed;
        match self {
            &ListFilter::Completed => completed,
            &ListFilter::Active => !completed,
            &ListFilter::All => true,
        }
    }
}

fn main_list(
    reia: ComponentBuilder,
    (todos, reducer, filter): (Vector<Todo>, TodosReducer, ListFilter),
) -> impl ComponentReturn {
    let reia = reia.init();
    let props: Vector<(Todo, TodosReducer, usize)> = todos
        .iter()
        .enumerate()
        .filter(|(_, td)| filter.filter(td))
        .map(|(idx, td)| (td.clone(), reducer.clone(), idx))
        .collect();
    reia.comp(vec_comps, (one_todo, props))
}

fn filter_button(
    reia: ComponentBuilder,
    (this, current): (ListFilter, ListFilter),
) -> impl ComponentReturn {
    const SHARED_CLASS: &str = "rounded border px-1 mx-2";
    let selected_classes: String = format!("{} border-red-900", SHARED_CLASS);
    let unselected_classes: String =
        format!("{} border-transparent hover:border-red-900", SHARED_CLASS);
    let reia = reia.init();
    let href = match this {
        ListFilter::All => "#/all",
        ListFilter::Active => "#/active",
        ListFilter::Completed => "#/completed",
    };
    let text = match this {
        ListFilter::All => "All",
        ListFilter::Active => "Active",
        ListFilter::Completed => "Completed",
    };
    reia.comp(
        a,
        HtmlProps::<web_sys::HtmlAnchorElement>::new()
            .href(href)
            .class_name(if this == current {
                selected_classes
            } else {
                unselected_classes
            }),
    )
    .child(|r| r.comp(text_node, text))
}

fn bottom_controls(
    reia: ComponentBuilder,
    (todos, reducer, filter): (Vector<Todo>, TodosReducer, ListFilter),
) -> impl ComponentReturn {
    let reia = reia.init();
    let on_clear = Callback::new(move |_ev| {
        reducer.reduce(TodosReduction::ClearCompleted());
    });
    reia.comp(
        div,
        html_props().class_name(if !todos.is_empty() {
            "flex flex-row items-center flex-wrap p-2 font-light border-t-2 border-gray-100"
        } else {
            "hidden"
        }),
    )
    .child(|r| {
        r.comp(
            div,
            html_props().class_name("flex-1 flex flex-row justify-start order-1"),
        )
        .child(|r| {
            r.comp(text_node, {
                let count = todos.iter().filter(|td| !td.completed).count();
                format!(
                    "{} {} left",
                    count,
                    if count == 1 { "item" } else { "items" }
                )
            })
        })
        .comp(
            div,
            html_props().class_name("flex-1 flex flex-row justify-end order-2 sm:order-3"),
        )
        .child(|r| {
            r.comp(
                button,
                html_props().onclick(on_clear).class_name({
                    if todos.iter().filter(|td| td.completed).count() > 0 {
                        "font-light hover:underline"
                    } else {
                        "invisible disabled"
                    }
                }),
            )
            .child(|r| r.comp(text_node, "Clear Completed"))
        })
        .comp(
            div,
            html_props().class_name(
                "flex-1 flex flex-row justify-center order-3 sm:order-2 min-w-full sm:min-w-min",
            ),
        )
        .child(|r| {
            r.comp(filter_button, (ListFilter::All, filter))
                .comp(filter_button, (ListFilter::Active, filter))
                .comp(filter_button, (ListFilter::Completed, filter))
        })
    })
}

static TODOS_STORAGE_KEY: &str = "todos-reia";

fn use_todos(reia: HookBuilder, _: ()) -> impl HookReturn<(Vector<Todo>, TodosReducer)> {
    let (reia, (todos, setter)) = reia.init().hook(use_state_from, || {
        if let Some(storage) = web_sys::window().unwrap().local_storage().unwrap() {
            if let Some(s) = storage.get_item(TODOS_STORAGE_KEY).unwrap() {
                return todos_from_str(&s);
            }
        }
        Vector::new()
    });
    let (reia, _) = reia.hook(
        use_effect,
        (
            |todos: Vector<Todo>| {
                if let Some(storage) = web_sys::window().unwrap().local_storage().unwrap() {
                    storage
                        .set_item(TODOS_STORAGE_KEY, &todos_to_string(&todos))
                        .unwrap();
                }
                || {}
            },
            todos.clone(),
        ),
    );
    (reia, (todos, TodosReducer(setter)))
}

pub fn app_main(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, (todos, reducer)) = reia.hook(use_todos, ());
    let get_filter = || match &web_sys::window().unwrap().location().hash().unwrap() as &str {
        "#/all" => ListFilter::All,
        "#/active" => ListFilter::Active,
        "#/completed" => ListFilter::Completed,
        _ => ListFilter::All,
    };
    let (reia, (filter, filter_setter)) = reia.hook(use_state_from, get_filter.clone());
    let on_hashchange = Callback::<web_sys::Event>::new(move |_ev| {
        let filter = get_filter();
        filter_setter.set(filter);
    });
    let (reia, _) = reia.hook(
        use_effect,
        (
            |on_hashchange: Callback<web_sys::Event>| {
                web_sys::window()
                    .unwrap()
                    .set_onhashchange(Some(on_hashchange.as_websys_function()));
                || {}
            },
            on_hashchange,
        ),
    );
    reia.comp(
        main,
        html_props().class_name("mt-8 shadow-xl w-full max-w-prose bg-white"),
    )
    .child(|r| {
        r.comp(top_box, (todos.clone(), reducer.clone()))
            .comp(main_list, (todos.clone(), reducer.clone(), filter))
            .comp(bottom_controls, (todos.clone(), reducer.clone(), filter))
    })
}
