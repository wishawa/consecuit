use consecuit::prelude::*;
use consecuit_html::prelude::*;
use im_rc::Vector;

use crate::model::{todos_from_str, todos_to_string, Todo, TodosReducer, TodosReduction};

fn use_text_edit(
    cc: HookBuilder,
    (submit, escape): (
        impl Fn(String) + Clone + 'static,
        impl Fn() + Clone + 'static,
    ),
) -> impl HookReturn<HtmlProps<web_sys::HtmlInputElement>> {
    const ENTER_KEY: u32 = 13;
    const ESCAPE_KEY: u32 = 27;

    let (cc, input_ref) = cc.hook(use_ref, ());
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
        ENTER_KEY => submit_cloned(),
        ESCAPE_KEY => escape(),
        _ => {}
    });
    let submit_cloned = submit.clone();
    let on_blur = Callback::new(move |_ev| {
        submit_cloned();
    });
    let (cc, _) = cc.hook(
        use_effect,
        (
            |input_ref: Reference<Option<web_sys::HtmlInputElement>>| {
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
    (cc, props)
}

fn top_box(
    cc: ComponentBuilder,
    (todos, reducer): (Vector<Todo>, TodosReducer),
) -> impl ComponentReturn {
    let cloned_reducer = reducer.clone();
    let is_empty = todos.is_empty();
    let all_completed = todos.iter().all(|td| td.completed);
    let on_toggle_all = Callback::new(move |_ev| {
        cloned_reducer.reduce(TodosReduction::ToggleAll(!all_completed));
    });

    let (cc, input_props) = cc.hook(
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

    cc_tree!(
        <div {html_props().class_name("flex flex-row text-2xl h-16")}>
            <button {html_props().class_name({
                if is_empty {
                    "w-12 invisible disabled"
                } else if all_completed {
                    "w-12 text-green-500"
                } else {
                    "w-12 text-gray-400"
                }
            }).onclick(on_toggle_all)}>
                <span {html_props().class_name("material-icons")}>
                    "checklist"
                </span>
            </button>
            <input {input_props.class_name("flex-1 min-w-0 p-2 placeholder-gray-300").placeholder("What needs to be done?")}/>
        </div>
    )
}

fn one_todo(
    cc: ComponentBuilder,
    (todo, reducer, idx): (Todo, TodosReducer, usize),
) -> impl ComponentReturn {
    let reducer_cloned = reducer.clone();
    let toggle = Callback::new(move |_ev| {
        reducer_cloned.reduce(TodosReduction::Toggle(idx));
    });
    let reducer_cloned = reducer.clone();
    let remove = Callback::new(move |_ev| {
        reducer_cloned.reduce(TodosReduction::Remove(idx));
    });
    let (cc, (edit, edit_setter)) = cc.hook(use_state, false);
    let edit_setter_cloned = edit_setter.clone();
    let enter_edit = Callback::new(move |_ev| {
        edit_setter_cloned.set_to(true);
    });
    cc_tree!(
        <div {html_props().class_name("border-t-2 border-gray-100 text-2xl")}>
            <div {html_props().class_name(if edit {"hidden"} else {"flex flex-row group"})}>
                <button {html_props().onclick(toggle).class_name({
                    if todo.completed {
                        "w-12 text-green-500"
                    } else {
                        "w-12 text-gray-400"
                    }
                })}>
                    <span {html_props().class_name("material-icons")}>
                        {if todo.completed { "check_circle" } else { "circle" }}
                    </span>
                </button>
                <label {html_props().ondblclick(enter_edit).class_name({
                    const SHARED_CLASS: &str = "flex-1 min-w-0 flex items-center break-all p-2";
                    if todo.completed {
                        format!("{} text-gray-500 line-through", SHARED_CLASS)
                    } else {
                        format!("{} text-gray-800", SHARED_CLASS)
                    }
                })}>
                    {todo.name.clone()}
                </label>
                <button {html_props().onclick(remove).class_name("w-12 text-red-700 opacity-0 group-hover:opacity-100")}>
                    <span {html_props().class_name("material-icons")}>
                        "clear"
                    </span>
                </button>
            </div>
            <opt_comp {(todo_edit, if edit {
                Some((todo, reducer, idx, edit_setter))
            } else {
                None
            })} />
        </div>
    )
}

fn todo_edit(
    cc: ComponentBuilder,
    (todo, reducer, idx, edit_setter): (Todo, TodosReducer, usize, Updater<bool>),
) -> impl ComponentReturn {
    let edit_setter_cloned = edit_setter.clone();
    let (cc, input_props) = cc.hook(
        use_text_edit,
        (
            move |text: String| {
                if text.is_empty() {
                    reducer.reduce(TodosReduction::Remove(idx));
                } else {
                    reducer.reduce(TodosReduction::SetName(idx, text));
                }
                edit_setter_cloned.set_to(false);
            },
            move || {
                edit_setter.set_to(false);
            },
        ),
    );
    cc_tree!(
        <div {html_props().class_name("flex flex-row")}>
            <div {html_props().class_name("w-12")} />
            <input {input_props.value(todo.name).class_name("flex-1 p-2 shadow-inner")}/>
        </div>
    )
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
    cc: ComponentBuilder,
    (todos, reducer, filter): (Vector<Todo>, TodosReducer, ListFilter),
) -> impl ComponentReturn {
    let props: Vector<(Todo, TodosReducer, usize)> = todos
        .iter()
        .enumerate()
        .filter(|(_, td)| filter.filter(td))
        .map(|(idx, td)| (td.clone(), reducer.clone(), idx))
        .collect();
    cc.comp(vec_comps, (one_todo, props))
}

fn filter_button(
    cc: ComponentBuilder,
    (this, current): (ListFilter, ListFilter),
) -> impl ComponentReturn {
    const SHARED_CLASS: &str = "rounded border px-1 mx-2";
    let selected_classes: String = format!("{} border-red-900", SHARED_CLASS);
    let unselected_classes: String =
        format!("{} border-transparent hover:border-red-100", SHARED_CLASS);

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
    cc_tree!(
        <a {HtmlProps::<web_sys::HtmlAnchorElement>::new().href(href).class_name(if this == current {
            selected_classes
        } else {
            unselected_classes
        })}>
            {text}
        </a>
    )
}

fn bottom_controls(
    cc: ComponentBuilder,
    (todos, reducer, filter): (Vector<Todo>, TodosReducer, ListFilter),
) -> impl ComponentReturn {
    let on_clear = Callback::new(move |_ev| {
        reducer.reduce(TodosReduction::ClearCompleted());
    });
    cc_tree!(
        <div {html_props().class_name(if !todos.is_empty() {
            "flex flex-row items-center flex-wrap sm:flex-nowrap p-2 font-light border-t-2 border-gray-100"
        } else {
            "hidden"
        })}>
            <div {html_props().class_name("flex-1 flex flex-row order-1 justify-start")}>
                {
                    let count = todos.iter().filter(|td| !td.completed).count();
                    format!(
                        "{} {} left",
                        count,
                        if count == 1 { "item" } else { "items" }
                    )
                }
            </div>
            <div {html_props().class_name("flex-1 flex flex-row order-2 sm:order-3 justify-end")}>
                <button {html_props().onclick(on_clear).class_name({
                    if todos.iter().filter(|td| td.completed).count() > 0 {
                        "font-light hover:underline"
                    } else {
                        "invisible disabled"
                    }
                })}>
                    "Clear Completed"
                </button>
            </div>
            <div {html_props().class_name("flex-1 flex flex-row order-3 sm:order-2 min-w-full sm:min-w-min justify-center")}>
                <filter_button {(ListFilter::All, filter)} />
                <filter_button {(ListFilter::Active, filter)} />
                <filter_button {(ListFilter::Completed, filter)} />
            </div>
        </div>
    )
}

static TODOS_STORAGE_KEY: &str = "todos-consecuit";

fn use_todos(cc: HookBuilder, _: ()) -> impl HookReturn<(Vector<Todo>, TodosReducer)> {
    let (cc, (todos, setter)) = cc.init().hook(use_state_from, || {
        if let Some(storage) = web_sys::window().unwrap().local_storage().unwrap() {
            if let Some(s) = storage.get_item(TODOS_STORAGE_KEY).unwrap() {
                return todos_from_str(&s);
            }
        }
        Vector::new()
    });
    let todos_cloned = todos.clone();
    let on_beforeunload = Callback::new(move |_ev| {
        if let Some(storage) = web_sys::window().unwrap().local_storage().unwrap() {
            storage
                .set_item(TODOS_STORAGE_KEY, &todos_to_string(&todos_cloned))
                .unwrap();
        }
    });
    let (cc, _) = cc.hook(
        use_effect,
        (
            |on_beforeunload: Callback<web_sys::Event>| {
                web_sys::window()
                    .unwrap()
                    .set_onbeforeunload(Some(on_beforeunload.as_websys_function()));
                || {}
            },
            on_beforeunload,
        ),
    );
    (cc, (todos, TodosReducer(setter)))
}

fn use_filter(cc: HookBuilder, _: ()) -> impl HookReturn<ListFilter> {
    let get_filter = || match &web_sys::window().unwrap().location().hash().unwrap() as &str {
        "#/all" => ListFilter::All,
        "#/active" => ListFilter::Active,
        "#/completed" => ListFilter::Completed,
        _ => ListFilter::All,
    };
    let (cc, (filter, filter_setter)) = cc.hook(use_state_from, get_filter.clone());
    let on_hashchange = Callback::<web_sys::Event>::new(move |_ev| {
        let filter = get_filter();
        filter_setter.set_to(filter);
    });
    let (cc, _) = cc.hook(
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
    (cc, filter)
}

pub fn app_main(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let (cc, (todos, reducer)) = cc.hook(use_todos, ());
    let (cc, filter) = cc.hook(use_filter, ());
    cc_tree!(
        <main {html_props().class_name("mt-8 shadow-xl w-full max-w-prose bg-white")}>
            <top_box {(todos.clone(), reducer.clone())} />
            <main_list {(todos.clone(), reducer.clone(), filter)} />
            <bottom_controls {(todos, reducer, filter)} />
        </main>
    )
}
