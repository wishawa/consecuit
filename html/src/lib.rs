pub mod callback;
pub mod components;
mod interfaces;
mod shared;

use im_rc::Vector;
use reia::{
    hooks::{use_ref, ReiaRef},
    HookBuilder, HookReturn,
};
use shared::SharedProp;
use web_sys::{window, Element, HtmlElement};

use wasm_bindgen::JsCast;

#[derive(Clone)]
struct WrappedElement<T: Clone + AsRef<HtmlElement>>(T);

pub trait PropEnum<T: AsRef<HtmlElement>>: Clone + PartialEq {
    fn set_on(&self, elem: &T);
    fn unset_on(&self, elem: &T);
}

pub trait ElementComponent: 'static + Clone + AsRef<HtmlElement> + JsCast {
    type PropEnum: PropEnum<Self>;
}

#[derive(Clone)]
pub struct ElementProps<E: ElementComponent>(Vector<ElementProp<E>>);

#[derive(Clone)]
enum ElementProp<E: ElementComponent> {
    Shared(SharedProp),
    Own(E::PropEnum),
}

impl<E: ElementComponent> PartialEq for ElementProp<E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (ElementProp::Shared(s1), ElementProp::Shared(s2)) => s1 == s2,
            (ElementProp::Own(o1), ElementProp::Own(o2)) => o1 == o2,
            _ => false,
        }
    }
}

impl<E: ElementComponent> PartialEq for ElementProps<E> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub(crate) struct UseElementArgs<T>
where
    T: ElementComponent,
{
    tag_name: &'static str,
    props: ElementProps<T>,
    parent: Element,
}

pub(crate) fn use_element<T: ElementComponent>(
    reia: HookBuilder,
    args: UseElementArgs<T>,
) -> impl HookReturn<T> {
    let reia = reia.init();
    let UseElementArgs {
        tag_name,
        props,
        parent,
    } = args;
    let (reia, store): (_, ReiaRef<Option<WrappedElement<T>>>) = reia.hook(use_ref, ());
    let elem = store
        .visit_mut_with(|opt| {
            let elem = opt.get_or_insert_with(|| {
                let document = window().unwrap().document().unwrap();
                let elem: T = document
                    .create_element(tag_name)
                    .unwrap()
                    .dyn_into()
                    .unwrap();
                let html_element: &HtmlElement = elem.as_ref();
                parent.append_child(html_element.as_ref()).unwrap();
                WrappedElement(elem)
            });
            elem.0.clone()
        })
        .unwrap();

    let (reia, last_props): (_, ReiaRef<Option<Vector<ElementProp<T>>>>) = reia.hook(use_ref, ());
    last_props
        .visit_mut_with(|opt| {
            if let Some(last) = opt {
                for old_prop in last.iter() {
                    match old_prop {
                        ElementProp::Shared(shared) => shared.unset_on(elem.as_ref()),
                        ElementProp::Own(own) => own.unset_on(&elem),
                    }
                }
            }
            for new_prop in props.0.iter() {
                match new_prop {
                    ElementProp::Shared(shared) => shared.set_on(elem.as_ref()),
                    ElementProp::Own(own) => own.set_on(&elem),
                }
            }
            *opt = Some(props.0);
        })
        .unwrap();

    (reia, elem)
}

impl<E: ElementComponent> ElementProps<E> {
    pub fn new() -> Self {
        Self(Vector::new())
    }
}
