use crate::shared::SharedProp;
use im_rc::Vector;
use reia::{
    hooks::{use_ref, ReiaRef},
    HookBuilder, HookReturn,
};
use web_sys::{window, Element, HtmlElement};

use wasm_bindgen::JsCast;

#[derive(Clone)]
struct WrappedElement<T: Clone + AsRef<HtmlElement>>(T);

#[sealed::sealed(pub(crate))]
pub trait PropEnum<T: AsRef<HtmlElement>>: Clone + PartialEq {
    fn set_on(&self, elem: &T);
    fn unset_on(&self, elem: &T);
}

#[sealed::sealed(pub(crate))]
pub trait HtmlComponent: 'static + Clone + AsRef<HtmlElement> + JsCast {
    type PropEnum: PropEnum<Self>;
}

#[derive(Clone)]
pub struct HtmlProps<E: HtmlComponent>(pub(crate) Vector<HtmlProp<E>>);

#[derive(Clone)]
pub(crate) enum HtmlProp<E: HtmlComponent> {
    Shared(SharedProp),
    Own(E::PropEnum),
}

impl<E: HtmlComponent> PartialEq for HtmlProp<E> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (HtmlProp::Shared(s1), HtmlProp::Shared(s2)) => s1 == s2,
            (HtmlProp::Own(o1), HtmlProp::Own(o2)) => o1 == o2,
            _ => false,
        }
    }
}

impl<E: HtmlComponent> PartialEq for HtmlProps<E> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

pub(crate) struct UseElementArgs<T>
where
    T: HtmlComponent,
{
    pub tag_name: &'static str,
    pub props: HtmlProps<T>,
    pub parent: Element,
}

pub(crate) fn use_element<T: HtmlComponent>(
    reia: HookBuilder,
    args: UseElementArgs<T>,
) -> impl HookReturn<T> {
    let reia = reia.init();
    let UseElementArgs {
        tag_name,
        props,
        parent,
    } = args;
    let (reia, store): (_, ReiaRef<Option<T>>) = reia.hook(use_ref, ());
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
                elem
            });
            elem.clone()
        })
        .unwrap();

    let (reia, last_props): (_, ReiaRef<Option<Vector<HtmlProp<T>>>>) = reia.hook(use_ref, ());
    last_props
        .visit_mut_with(|opt| {
            if let Some(last) = opt {
                for old_prop in last.iter() {
                    match old_prop {
                        HtmlProp::Shared(shared) => shared.unset_on(elem.as_ref()),
                        HtmlProp::Own(own) => own.unset_on(&elem),
                    }
                }
            }
            for new_prop in props.0.iter() {
                match new_prop {
                    HtmlProp::Shared(shared) => shared.set_on(elem.as_ref()),
                    HtmlProp::Own(own) => own.set_on(&elem),
                }
            }
            *opt = Some(props.0);
        })
        .unwrap();

    (reia, elem)
}

impl<E: HtmlComponent> HtmlProps<E> {
    pub fn new() -> Self {
        Self(Vector::new())
    }
}

pub fn html_props<E: HtmlComponent>() -> HtmlProps<E> {
    HtmlProps::new()
}
