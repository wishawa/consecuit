use im_rc::Vector;
use reia::*;
use reia_html::prelude::*;

#[derive(Clone, PartialEq, Debug)]
struct Todo {
    name: String,
    completed: bool,
}

type TodosVector = Vector<Todo>;

fn top_box(reia: ComponentBuilder, todos_setter: StateSetter<TodosVector>) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, (text, text_setter)) = reia.hook(use_state, String::new());
    let (reia, on_input) = reia.hook(
        use_memo,
        (
            |text_setter: StateSetter<String>| {
                Callback::new(move |ev: web_sys::InputEvent| {
                    if let Some(new_text) = ev.data() {
                        text_setter.set(new_text);
                    }
                })
            },
            text_setter.clone(),
        ),
    );
    let (reia, on_keypress) = reia.hook(
        use_memo,
        (
            |(setter, text): (StateSetter<TodosVector>, String)| {
                Callback::new(move |ev: web_sys::KeyboardEvent| {
                    if ev.key_code() == 13 {
                        setter.update_with(|mut v| {
                            let todo = Todo {
                                name: text.clone(),
                                completed: false,
                            };
                            v.push_back(todo);
                            v
                        });
                    }
                })
            },
            (todos_setter.clone(), text.clone()),
        ),
    );
    let (reia, on_blur) = reia.hook(
        use_memo,
        (
            |(setter, text): (StateSetter<TodosVector>, String)| {
                Callback::new(move |_ev| {
                    setter.update_with(|mut v| {
                        let todo = Todo {
                            name: text.clone(),
                            completed: false,
                        };
                        v.push_back(todo);
                        v
                    });
                })
            },
            (todos_setter.clone(), text.clone()),
        ),
    );
    let (reia, on_toggle_all) = reia.hook(
        use_memo,
        (
            |setter: StateSetter<TodosVector>| {
                Callback::new(move |_ev| {
                    setter.update_with(|v| {
                        let toggle_to = !v.iter().all(|td| td.completed);
                        v.into_iter()
                            .map(|td| Todo {
                                completed: toggle_to,
                                ..td
                            })
                            .collect()
                    })
                })
            },
            todos_setter.clone(),
        ),
    );

    let input_props: HtmlProps<web_sys::HtmlInputElement> = html_props()
        .onblur(on_blur)
        .onkeypress(on_keypress)
        .oninput(on_input)
        .class_name("border-2");
    let input_props = input_props.value(text);

    reia.comp(div, html_props().class_name("flex-row"))
        .child(|r| {
            r.comp(button, html_props().onclick(on_toggle_all))
                .child(|r| r.comp(text_node, "Toggle"))
                .comp(input, input_props)
        })
}

fn todo_view(
    reia: ComponentBuilder,
    (todo, setter, idx): (Todo, StateSetter<TodosVector>, usize),
) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, toggle) = reia.hook(
        use_memo,
        (
            |(setter, idx): (StateSetter<TodosVector>, usize)| {
                Callback::new(move |_ev| {
                    setter.update_with(|mut v| {
                        v[idx].completed = !v[idx].completed;
                        v
                    })
                })
            },
            (setter.clone(), idx),
        ),
    );
    let (reia, remove) = reia.hook(
        use_memo,
        (
            |(setter, idx): (StateSetter<TodosVector>, usize)| {
                Callback::new(move |_ev| {
                    setter.update_with(|mut v| {
                        v.remove(idx);
                        v
                    })
                })
            },
            (setter, idx),
        ),
    );
    reia.comp(div, html_props().class_name("border-2"))
        .child(|r| {
            r.comp(button, html_props().onclick(toggle))
                .child(|r| r.comp(text_node, if todo.completed { "☑" } else { "☐" }))
                .comp(text_node, todo.name)
                .comp(button, html_props().onclick(remove))
                .child(|r| r.comp(text_node, "␡"))
        })
}

fn main_list(
    reia: ComponentBuilder,
    (todos, setter): (TodosVector, StateSetter<TodosVector>),
) -> impl ComponentReturn {
    let reia = reia.init();
    let props: Vector<(Todo, StateSetter<TodosVector>, usize)> = todos
        .iter()
        .enumerate()
        .map(|(idx, td)| (td.clone(), setter.clone(), idx))
        .collect();
    reia.comp(vec_comps, (todo_view, props))
}

pub fn app_main(reia: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let reia = reia.init();
    let (reia, (todos, todos_setter)) = reia.hook(use_state, Vector::new());
    //web_sys::console::debug_1(&format!("{:?}", todos).into());
    reia.comp(div, html_props()).child(|r| {
        r.comp(top_box, todos_setter.clone())
            .comp(main_list, (todos, todos_setter.clone()))
    })
}
