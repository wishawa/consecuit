use im_rc::Vector;
use reia::*;
use reia_html::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct Todo {
    name: String,
    completed: bool,
}

#[derive(Clone, PartialEq)]
struct TodosReducer(StateSetter<Vector<Todo>>);

enum TodosReduction {
    Add(String),
    Remove(usize),
    Toggle(usize),
    SetName(usize, String),
    ToggleAll(),
}

impl TodosReducer {
    fn reduce(&self, reduction: TodosReduction) {
        self.0.update_with(|mut v| {
            match reduction {
                TodosReduction::Add(name) => {
                    v.push_back(Todo {
                        name,
                        completed: false,
                    });
                }
                TodosReduction::Remove(idx) => {
                    v.remove(idx);
                }
                TodosReduction::Toggle(idx) => {
                    let current = v.get_mut(idx).unwrap();
                    current.completed = !current.completed;
                }
                TodosReduction::SetName(idx, name) => {
                    let current = v.get_mut(idx).unwrap();
                    current.name = name;
                }
                TodosReduction::ToggleAll() => {
                    let to = !v.iter().all(|td| td.completed);
                    for td in v.iter_mut() {
                        td.completed = to;
                    }
                }
            }
            v
        })
    }
}

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
                value
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
        .onkeyup(on_keypress)
        .onblur(on_blur);
    (reia, props)
}

fn top_box(reia: ComponentBuilder, reducer: TodosReducer) -> impl ComponentReturn {
    let reia = reia.init();
    let cloned_reducer = reducer.clone();
    let on_toggle_all = Callback::new(move |_ev| {
        cloned_reducer.reduce(TodosReduction::ToggleAll());
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

    reia.comp(div, html_props().class_name("flex flex-row"))
        .child(|r| {
            r.comp(button, html_props().onclick(on_toggle_all))
                .child(|r| r.comp(text_node, "⯆"))
                .comp(input, input_props)
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
    reia.comp(div, html_props().class_name("border-2"))
        .child(|r| {
            r.comp(
                div,
                html_props().class_name(if edit {
                    "hidden"
                } else {
                    "flex flex-row justify-between"
                }),
            )
            .child(|r| {
                r.comp(button, html_props().onclick(toggle))
                    .child(|r| r.comp(text_node, if todo.completed { "☑" } else { "☐" }))
                    .comp(div, html_props().ondblclick(enter_edit))
                    .child(|r| r.comp(text_node, todo.name.clone()))
                    .comp(button, html_props().onclick(remove))
                    .child(|r| r.comp(text_node, "␡"))
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
    (todo, reducer, idx, edit_setter): (Todo, TodosReducer, usize, StateSetter<bool>),
) -> impl ComponentReturn {
    let reia = reia.init();
    let edit_setter_cloned = edit_setter.clone();
    let (reia, input_props) = reia.hook(
        use_text_edit,
        (
            move |text: String| {
                reducer.reduce(TodosReduction::SetName(idx, text));
                edit_setter_cloned.set(false);
            },
            move || {
                edit_setter.set(false);
            },
        ),
    );
    reia.comp(div, html_props().class_name("flex flex-row"))
        .child(|r| r.comp(input, input_props.value(todo.name)))
}

fn main_list(
    reia: ComponentBuilder,
    (todos, reducer): (Vector<Todo>, TodosReducer),
) -> impl ComponentReturn {
    let reia = reia.init();
    let props: Vector<(Todo, TodosReducer, usize)> = todos
        .iter()
        .enumerate()
        .map(|(idx, td)| (td.clone(), reducer.clone(), idx))
        .collect();
    reia.comp(vec_comps, (one_todo, props))
}

fn use_todos(reia: HookBuilder, _: ()) -> impl HookReturn<(Vector<Todo>, TodosReducer)> {
    let (reia, (todos, setter)) = reia.init().hook(use_state, Vector::new());
    (reia, (todos, TodosReducer(setter)))
}

pub fn app_main(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, (todos, reducer)) = reia.hook(use_todos, ());
    reia.comp(div, html_props()).child(|r| {
        r.comp(top_box, reducer.clone())
            .comp(main_list, (todos, reducer.clone()))
    })
}
