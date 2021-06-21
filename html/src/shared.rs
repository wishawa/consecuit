use crate::{
    callback::Callback,
    elem::{HtmlComponent, HtmlProp, HtmlProps},
};
use web_sys::HtmlElement;
use web_sys::{
    AnimationEvent, DragEvent, Event, FocusEvent, InputEvent, KeyboardEvent, MouseEvent,
    PointerEvent, TouchEvent, TransitionEvent, UiEvent, WheelEvent,
};

#[allow(non_camel_case_types)]
#[derive(Clone, PartialEq)]
pub(crate) enum SharedProp {
    node_value(String),
    text_content(String),
    id(String),
    class_name(String),
    scroll_top(i32),
    scroll_left(i32),
    inner_html(String),
    outer_html(String),
    slot(String),
    title(String),
    scroll_height(i32),
    lang(String),
    dir(String),
    inner_text(String),
    hidden(bool),
    tab_index(i32),
    access_key(String),
    draggable(bool),
    content_editable(String),
    spellcheck(bool),
    oncopy(Callback),
    oncut(Callback),
    onpaste(Callback),
    onabort(Callback),
    onblur(Callback<FocusEvent>),
    onfocus(Callback<FocusEvent>),
    onauxclick(Callback<MouseEvent>),
    oncanplay(Callback),
    oncanplaythrough(Callback<Event>),
    onchange(Callback<Event>),
    onclick(Callback<MouseEvent>),
    onclose(Callback<Event>),
    oncontextmenu(Callback<MouseEvent>),
    ondblclick(Callback<MouseEvent>),
    ondrag(Callback<DragEvent>),
    ondragend(Callback<DragEvent>),
    ondragenter(Callback<DragEvent>),
    ondragexit(Callback),
    ondragleave(Callback<DragEvent>),
    ondragover(Callback<DragEvent>),
    ondragstart(Callback<DragEvent>),
    ondrop(Callback<DragEvent>),
    ondurationchange(Callback),
    onemptied(Callback),
    onended(Callback),
    oninput(Callback<InputEvent>),
    oninvalid(Callback<Event>),
    onkeydown(Callback<KeyboardEvent>),
    onkeypress(Callback<KeyboardEvent>),
    onkeyup(Callback<KeyboardEvent>),
    onload(Callback<Event>),
    onloadeddata(Callback),
    onloadedmetadata(Callback),
    onloadend(Callback),
    onloadstart(Callback),
    onmousedown(Callback<MouseEvent>),
    onmouseenter(Callback<MouseEvent>),
    onmouseleave(Callback<MouseEvent>),
    onmousemove(Callback<MouseEvent>),
    onmouseout(Callback<MouseEvent>),
    onmouseover(Callback<MouseEvent>),
    onmouseup(Callback<MouseEvent>),
    onwheel(Callback<WheelEvent>),
    onpause(Callback),
    onplay(Callback),
    onplaying(Callback),
    onprogress(Callback),
    onratechange(Callback),
    onreset(Callback<Event>),
    onresize(Callback<UiEvent>),
    onscroll(Callback<Event>),
    onseeked(Callback),
    onseeking(Callback),
    onselect(Callback<UiEvent>),
    onshow(Callback<Event>),
    onstalled(Callback),
    onsubmit(Callback),
    onsuspend(Callback),
    ontimeupdate(Callback),
    onvolumechange(Callback),
    onwaiting(Callback),
    onselectstart(Callback<FocusEvent>),
    ontoggle(Callback),
    onpointercancel(Callback<PointerEvent>),
    onpointerdown(Callback<PointerEvent>),
    onpointerup(Callback<PointerEvent>),
    onpointermove(Callback<PointerEvent>),
    onpointerout(Callback<PointerEvent>),
    onpointerover(Callback<PointerEvent>),
    onpointerenter(Callback<PointerEvent>),
    onpointerleave(Callback<PointerEvent>),
    ongotpointercapture(Callback<PointerEvent>),
    onlostpointercapture(Callback<PointerEvent>),
    onanimationcancel(Callback<AnimationEvent>),
    onanimationend(Callback<AnimationEvent>),
    onanimationiteration(Callback<AnimationEvent>),
    onanimationstart(Callback<AnimationEvent>),
    ontransitioncancel(Callback<TransitionEvent>),
    ontransitionend(Callback<TransitionEvent>),
    ontransitionrun(Callback<TransitionEvent>),
    ontransitionstart(Callback<TransitionEvent>),
    onwebkitanimationend(Callback),
    onwebkitanimationiteration(Callback),
    onwebkitanimationstart(Callback),
    onwebkittransitionend(Callback),
    onerror(Callback<Event>),
    ontouchstart(Callback<TouchEvent>),
    ontouchend(Callback<TouchEvent>),
    ontouchmove(Callback<TouchEvent>),
    ontouchcancel(Callback<TouchEvent>),
}

#[sealed::sealed]
impl crate::elem::PropEnum<HtmlElement> for SharedProp {
    fn set_on(&self, elem: &HtmlElement) {
        match self {
            SharedProp::node_value(v) => elem.set_node_value(Some(v)),
            SharedProp::text_content(v) => elem.set_text_content(Some(v)),
            SharedProp::id(v) => elem.set_id(v),
            SharedProp::class_name(v) => elem.set_class_name(v),
            SharedProp::scroll_top(v) => elem.set_scroll_top(*v),
            SharedProp::scroll_left(v) => elem.set_scroll_left(*v),
            SharedProp::inner_html(v) => elem.set_inner_html(v),
            SharedProp::outer_html(v) => elem.set_outer_html(v),
            SharedProp::slot(v) => elem.set_slot(v),
            SharedProp::title(v) => elem.set_title(v),
            SharedProp::scroll_height(v) => elem.set_scroll_height(*v),
            SharedProp::lang(v) => elem.set_lang(v),
            SharedProp::dir(v) => elem.set_dir(v),
            SharedProp::inner_text(v) => elem.set_inner_text(v),
            SharedProp::hidden(v) => elem.set_hidden(*v),
            SharedProp::tab_index(v) => elem.set_tab_index(*v),
            SharedProp::access_key(v) => elem.set_access_key(v),
            SharedProp::draggable(v) => elem.set_draggable(*v),
            SharedProp::content_editable(v) => elem.set_content_editable(v),
            SharedProp::spellcheck(v) => elem.set_spellcheck(*v),
            SharedProp::oncopy(v) => elem.set_oncopy(Some(v.as_websys_function())),
            SharedProp::oncut(v) => elem.set_oncut(Some(v.as_websys_function())),
            SharedProp::onpaste(v) => elem.set_onpaste(Some(v.as_websys_function())),
            SharedProp::onabort(v) => elem.set_onabort(Some(v.as_websys_function())),
            SharedProp::onblur(v) => elem.set_onblur(Some(v.as_websys_function())),
            SharedProp::onfocus(v) => elem.set_onfocus(Some(v.as_websys_function())),
            SharedProp::onauxclick(v) => elem.set_onauxclick(Some(v.as_websys_function())),
            SharedProp::oncanplay(v) => elem.set_oncanplay(Some(v.as_websys_function())),
            SharedProp::oncanplaythrough(v) => {
                elem.set_oncanplaythrough(Some(v.as_websys_function()))
            }
            SharedProp::onchange(v) => elem.set_onchange(Some(v.as_websys_function())),
            SharedProp::onclick(v) => elem.set_onclick(Some(v.as_websys_function())),
            SharedProp::onclose(v) => elem.set_onclose(Some(v.as_websys_function())),
            SharedProp::oncontextmenu(v) => elem.set_oncontextmenu(Some(v.as_websys_function())),
            SharedProp::ondblclick(v) => elem.set_ondblclick(Some(v.as_websys_function())),
            SharedProp::ondrag(v) => elem.set_ondrag(Some(v.as_websys_function())),
            SharedProp::ondragend(v) => elem.set_ondragend(Some(v.as_websys_function())),
            SharedProp::ondragenter(v) => elem.set_ondragenter(Some(v.as_websys_function())),
            SharedProp::ondragexit(v) => elem.set_ondragexit(Some(v.as_websys_function())),
            SharedProp::ondragleave(v) => elem.set_ondragleave(Some(v.as_websys_function())),
            SharedProp::ondragover(v) => elem.set_ondragover(Some(v.as_websys_function())),
            SharedProp::ondragstart(v) => elem.set_ondragstart(Some(v.as_websys_function())),
            SharedProp::ondrop(v) => elem.set_ondrop(Some(v.as_websys_function())),
            SharedProp::ondurationchange(v) => {
                elem.set_ondurationchange(Some(v.as_websys_function()))
            }
            SharedProp::onemptied(v) => elem.set_onemptied(Some(v.as_websys_function())),
            SharedProp::onended(v) => elem.set_onended(Some(v.as_websys_function())),
            SharedProp::oninput(v) => elem.set_oninput(Some(v.as_websys_function())),
            SharedProp::oninvalid(v) => elem.set_oninvalid(Some(v.as_websys_function())),
            SharedProp::onkeydown(v) => elem.set_onkeydown(Some(v.as_websys_function())),
            SharedProp::onkeypress(v) => elem.set_onkeypress(Some(v.as_websys_function())),
            SharedProp::onkeyup(v) => elem.set_onkeyup(Some(v.as_websys_function())),
            SharedProp::onload(v) => elem.set_onload(Some(v.as_websys_function())),
            SharedProp::onloadeddata(v) => elem.set_onloadeddata(Some(v.as_websys_function())),
            SharedProp::onloadedmetadata(v) => {
                elem.set_onloadedmetadata(Some(v.as_websys_function()))
            }
            SharedProp::onloadend(v) => elem.set_onloadend(Some(v.as_websys_function())),
            SharedProp::onloadstart(v) => elem.set_onloadstart(Some(v.as_websys_function())),
            SharedProp::onmousedown(v) => elem.set_onmousedown(Some(v.as_websys_function())),
            SharedProp::onmouseenter(v) => elem.set_onmouseenter(Some(v.as_websys_function())),
            SharedProp::onmouseleave(v) => elem.set_onmouseleave(Some(v.as_websys_function())),
            SharedProp::onmousemove(v) => elem.set_onmousemove(Some(v.as_websys_function())),
            SharedProp::onmouseout(v) => elem.set_onmouseout(Some(v.as_websys_function())),
            SharedProp::onmouseover(v) => elem.set_onmouseover(Some(v.as_websys_function())),
            SharedProp::onmouseup(v) => elem.set_onmouseup(Some(v.as_websys_function())),
            SharedProp::onwheel(v) => elem.set_onwheel(Some(v.as_websys_function())),
            SharedProp::onpause(v) => elem.set_onpause(Some(v.as_websys_function())),
            SharedProp::onplay(v) => elem.set_onplay(Some(v.as_websys_function())),
            SharedProp::onplaying(v) => elem.set_onplaying(Some(v.as_websys_function())),
            SharedProp::onprogress(v) => elem.set_onprogress(Some(v.as_websys_function())),
            SharedProp::onratechange(v) => elem.set_onratechange(Some(v.as_websys_function())),
            SharedProp::onreset(v) => elem.set_onreset(Some(v.as_websys_function())),
            SharedProp::onresize(v) => elem.set_onresize(Some(v.as_websys_function())),
            SharedProp::onscroll(v) => elem.set_onscroll(Some(v.as_websys_function())),
            SharedProp::onseeked(v) => elem.set_onseeked(Some(v.as_websys_function())),
            SharedProp::onseeking(v) => elem.set_onseeking(Some(v.as_websys_function())),
            SharedProp::onselect(v) => elem.set_onselect(Some(v.as_websys_function())),
            SharedProp::onshow(v) => elem.set_onshow(Some(v.as_websys_function())),
            SharedProp::onstalled(v) => elem.set_onstalled(Some(v.as_websys_function())),
            SharedProp::onsubmit(v) => elem.set_onsubmit(Some(v.as_websys_function())),
            SharedProp::onsuspend(v) => elem.set_onsuspend(Some(v.as_websys_function())),
            SharedProp::ontimeupdate(v) => elem.set_ontimeupdate(Some(v.as_websys_function())),
            SharedProp::onvolumechange(v) => elem.set_onvolumechange(Some(v.as_websys_function())),
            SharedProp::onwaiting(v) => elem.set_onwaiting(Some(v.as_websys_function())),
            SharedProp::onselectstart(v) => elem.set_onselectstart(Some(v.as_websys_function())),
            SharedProp::ontoggle(v) => elem.set_ontoggle(Some(v.as_websys_function())),
            SharedProp::onpointercancel(v) => {
                elem.set_onpointercancel(Some(v.as_websys_function()))
            }
            SharedProp::onpointerdown(v) => elem.set_onpointerdown(Some(v.as_websys_function())),
            SharedProp::onpointerup(v) => elem.set_onpointerup(Some(v.as_websys_function())),
            SharedProp::onpointermove(v) => elem.set_onpointermove(Some(v.as_websys_function())),
            SharedProp::onpointerout(v) => elem.set_onpointerout(Some(v.as_websys_function())),
            SharedProp::onpointerover(v) => elem.set_onpointerover(Some(v.as_websys_function())),
            SharedProp::onpointerenter(v) => elem.set_onpointerenter(Some(v.as_websys_function())),
            SharedProp::onpointerleave(v) => elem.set_onpointerleave(Some(v.as_websys_function())),
            SharedProp::ongotpointercapture(v) => {
                elem.set_ongotpointercapture(Some(v.as_websys_function()))
            }
            SharedProp::onlostpointercapture(v) => {
                elem.set_onlostpointercapture(Some(v.as_websys_function()))
            }
            SharedProp::onanimationcancel(v) => {
                elem.set_onanimationcancel(Some(v.as_websys_function()))
            }
            SharedProp::onanimationend(v) => elem.set_onanimationend(Some(v.as_websys_function())),
            SharedProp::onanimationiteration(v) => {
                elem.set_onanimationiteration(Some(v.as_websys_function()))
            }
            SharedProp::onanimationstart(v) => {
                elem.set_onanimationstart(Some(v.as_websys_function()))
            }
            SharedProp::ontransitioncancel(v) => {
                elem.set_ontransitioncancel(Some(v.as_websys_function()))
            }
            SharedProp::ontransitionend(v) => {
                elem.set_ontransitionend(Some(v.as_websys_function()))
            }
            SharedProp::ontransitionrun(v) => {
                elem.set_ontransitionrun(Some(v.as_websys_function()))
            }
            SharedProp::ontransitionstart(v) => {
                elem.set_ontransitionstart(Some(v.as_websys_function()))
            }
            SharedProp::onwebkitanimationend(v) => {
                elem.set_onwebkitanimationend(Some(v.as_websys_function()))
            }
            SharedProp::onwebkitanimationiteration(v) => {
                elem.set_onwebkitanimationiteration(Some(v.as_websys_function()))
            }
            SharedProp::onwebkitanimationstart(v) => {
                elem.set_onwebkitanimationstart(Some(v.as_websys_function()))
            }
            SharedProp::onwebkittransitionend(v) => {
                elem.set_onwebkittransitionend(Some(v.as_websys_function()))
            }
            SharedProp::onerror(v) => elem.set_onerror(Some(v.as_websys_function())),
            SharedProp::ontouchstart(v) => elem.set_ontouchstart(Some(v.as_websys_function())),
            SharedProp::ontouchend(v) => elem.set_ontouchend(Some(v.as_websys_function())),
            SharedProp::ontouchmove(v) => elem.set_ontouchmove(Some(v.as_websys_function())),
            SharedProp::ontouchcancel(v) => elem.set_ontouchcancel(Some(v.as_websys_function())),
        }
    }
    fn unset_on(&self, elem: &HtmlElement) {
        match self {
            SharedProp::node_value(_) => elem.set_node_value(None),
            SharedProp::text_content(_) => elem.set_text_content(None),
            SharedProp::id(_) => elem.remove_attribute("id").unwrap(),
            SharedProp::class_name(_) => elem.remove_attribute("class_name").unwrap(),
            SharedProp::scroll_top(_) => elem.remove_attribute("scroll_top").unwrap(),
            SharedProp::scroll_left(_) => elem.remove_attribute("scroll_left").unwrap(),
            SharedProp::inner_html(_) => elem.remove_attribute("inner_html").unwrap(),
            SharedProp::outer_html(_) => elem.remove_attribute("outer_html").unwrap(),
            SharedProp::slot(_) => elem.remove_attribute("slot").unwrap(),
            SharedProp::title(_) => elem.remove_attribute("title").unwrap(),
            SharedProp::scroll_height(_) => elem.remove_attribute("scroll_height").unwrap(),
            SharedProp::lang(_) => elem.remove_attribute("lang").unwrap(),
            SharedProp::dir(_) => elem.remove_attribute("dir").unwrap(),
            SharedProp::inner_text(_) => elem.remove_attribute("inner_text").unwrap(),
            SharedProp::hidden(_) => elem.remove_attribute("hidden").unwrap(),
            SharedProp::tab_index(_) => elem.remove_attribute("tab_index").unwrap(),
            SharedProp::access_key(_) => elem.remove_attribute("access_key").unwrap(),
            SharedProp::draggable(_) => elem.remove_attribute("draggable").unwrap(),
            SharedProp::content_editable(_) => elem.remove_attribute("content_editable").unwrap(),
            SharedProp::spellcheck(_) => elem.remove_attribute("spellcheck").unwrap(),
            SharedProp::oncopy(_) => elem.set_oncopy(None),
            SharedProp::oncut(_) => elem.set_oncut(None),
            SharedProp::onpaste(_) => elem.set_onpaste(None),
            SharedProp::onabort(_) => elem.set_onabort(None),
            SharedProp::onblur(_) => elem.set_onblur(None),
            SharedProp::onfocus(_) => elem.set_onfocus(None),
            SharedProp::onauxclick(_) => elem.set_onauxclick(None),
            SharedProp::oncanplay(_) => elem.set_oncanplay(None),
            SharedProp::oncanplaythrough(_) => elem.set_oncanplaythrough(None),
            SharedProp::onchange(_) => elem.set_onchange(None),
            SharedProp::onclick(_) => elem.set_onclick(None),
            SharedProp::onclose(_) => elem.set_onclose(None),
            SharedProp::oncontextmenu(_) => elem.set_oncontextmenu(None),
            SharedProp::ondblclick(_) => elem.set_ondblclick(None),
            SharedProp::ondrag(_) => elem.set_ondrag(None),
            SharedProp::ondragend(_) => elem.set_ondragend(None),
            SharedProp::ondragenter(_) => elem.set_ondragenter(None),
            SharedProp::ondragexit(_) => elem.set_ondragexit(None),
            SharedProp::ondragleave(_) => elem.set_ondragleave(None),
            SharedProp::ondragover(_) => elem.set_ondragover(None),
            SharedProp::ondragstart(_) => elem.set_ondragstart(None),
            SharedProp::ondrop(_) => elem.set_ondrop(None),
            SharedProp::ondurationchange(_) => elem.set_ondurationchange(None),
            SharedProp::onemptied(_) => elem.set_onemptied(None),
            SharedProp::onended(_) => elem.set_onended(None),
            SharedProp::oninput(_) => elem.set_oninput(None),
            SharedProp::oninvalid(_) => elem.set_oninvalid(None),
            SharedProp::onkeydown(_) => elem.set_onkeydown(None),
            SharedProp::onkeypress(_) => elem.set_onkeypress(None),
            SharedProp::onkeyup(_) => elem.set_onkeyup(None),
            SharedProp::onload(_) => elem.set_onload(None),
            SharedProp::onloadeddata(_) => elem.set_onloadeddata(None),
            SharedProp::onloadedmetadata(_) => elem.set_onloadedmetadata(None),
            SharedProp::onloadend(_) => elem.set_onloadend(None),
            SharedProp::onloadstart(_) => elem.set_onloadstart(None),
            SharedProp::onmousedown(_) => elem.set_onmousedown(None),
            SharedProp::onmouseenter(_) => elem.set_onmouseenter(None),
            SharedProp::onmouseleave(_) => elem.set_onmouseleave(None),
            SharedProp::onmousemove(_) => elem.set_onmousemove(None),
            SharedProp::onmouseout(_) => elem.set_onmouseout(None),
            SharedProp::onmouseover(_) => elem.set_onmouseover(None),
            SharedProp::onmouseup(_) => elem.set_onmouseup(None),
            SharedProp::onwheel(_) => elem.set_onwheel(None),
            SharedProp::onpause(_) => elem.set_onpause(None),
            SharedProp::onplay(_) => elem.set_onplay(None),
            SharedProp::onplaying(_) => elem.set_onplaying(None),
            SharedProp::onprogress(_) => elem.set_onprogress(None),
            SharedProp::onratechange(_) => elem.set_onratechange(None),
            SharedProp::onreset(_) => elem.set_onreset(None),
            SharedProp::onresize(_) => elem.set_onresize(None),
            SharedProp::onscroll(_) => elem.set_onscroll(None),
            SharedProp::onseeked(_) => elem.set_onseeked(None),
            SharedProp::onseeking(_) => elem.set_onseeking(None),
            SharedProp::onselect(_) => elem.set_onselect(None),
            SharedProp::onshow(_) => elem.set_onshow(None),
            SharedProp::onstalled(_) => elem.set_onstalled(None),
            SharedProp::onsubmit(_) => elem.set_onsubmit(None),
            SharedProp::onsuspend(_) => elem.set_onsuspend(None),
            SharedProp::ontimeupdate(_) => elem.set_ontimeupdate(None),
            SharedProp::onvolumechange(_) => elem.set_onvolumechange(None),
            SharedProp::onwaiting(_) => elem.set_onwaiting(None),
            SharedProp::onselectstart(_) => elem.set_onselectstart(None),
            SharedProp::ontoggle(_) => elem.set_ontoggle(None),
            SharedProp::onpointercancel(_) => elem.set_onpointercancel(None),
            SharedProp::onpointerdown(_) => elem.set_onpointerdown(None),
            SharedProp::onpointerup(_) => elem.set_onpointerup(None),
            SharedProp::onpointermove(_) => elem.set_onpointermove(None),
            SharedProp::onpointerout(_) => elem.set_onpointerout(None),
            SharedProp::onpointerover(_) => elem.set_onpointerover(None),
            SharedProp::onpointerenter(_) => elem.set_onpointerenter(None),
            SharedProp::onpointerleave(_) => elem.set_onpointerleave(None),
            SharedProp::ongotpointercapture(_) => elem.set_ongotpointercapture(None),
            SharedProp::onlostpointercapture(_) => elem.set_onlostpointercapture(None),
            SharedProp::onanimationcancel(_) => elem.set_onanimationcancel(None),
            SharedProp::onanimationend(_) => elem.set_onanimationend(None),
            SharedProp::onanimationiteration(_) => elem.set_onanimationiteration(None),
            SharedProp::onanimationstart(_) => elem.set_onanimationstart(None),
            SharedProp::ontransitioncancel(_) => elem.set_ontransitioncancel(None),
            SharedProp::ontransitionend(_) => elem.set_ontransitionend(None),
            SharedProp::ontransitionrun(_) => elem.set_ontransitionrun(None),
            SharedProp::ontransitionstart(_) => elem.set_ontransitionstart(None),
            SharedProp::onwebkitanimationend(_) => elem.set_onwebkitanimationend(None),
            SharedProp::onwebkitanimationiteration(_) => elem.set_onwebkitanimationiteration(None),
            SharedProp::onwebkitanimationstart(_) => elem.set_onwebkitanimationstart(None),
            SharedProp::onwebkittransitionend(_) => elem.set_onwebkittransitionend(None),
            SharedProp::onerror(_) => elem.set_onerror(None),
            SharedProp::ontouchstart(_) => elem.set_ontouchstart(None),
            SharedProp::ontouchend(_) => elem.set_ontouchend(None),
            SharedProp::ontouchmove(_) => elem.set_ontouchmove(None),
            SharedProp::ontouchcancel(_) => elem.set_ontouchcancel(None),
        }
    }
}

impl<E: HtmlComponent> HtmlProps<E> {
    pub fn node_value(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::node_value(val)));
        self
    }
    pub fn text_content(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::text_content(val)));
        self
    }
    pub fn id(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Shared(SharedProp::id(val)));
        self
    }
    pub fn class_name(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::class_name(val)));
        self
    }
    pub fn scroll_top(mut self, val: i32) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::scroll_top(val)));
        self
    }
    pub fn scroll_left(mut self, val: i32) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::scroll_left(val)));
        self
    }
    pub fn inner_html(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::inner_html(val)));
        self
    }
    pub fn outer_html(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::outer_html(val)));
        self
    }
    pub fn slot(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Shared(SharedProp::slot(val)));
        self
    }
    pub fn title(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Shared(SharedProp::title(val)));
        self
    }
    pub fn scroll_height(mut self, val: i32) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::scroll_height(val)));
        self
    }
    pub fn lang(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Shared(SharedProp::lang(val)));
        self
    }
    pub fn dir(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0.push_back(HtmlProp::Shared(SharedProp::dir(val)));
        self
    }
    pub fn inner_text(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::inner_text(val)));
        self
    }
    pub fn hidden(mut self, val: bool) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::hidden(val)));
        self
    }
    pub fn tab_index(mut self, val: i32) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::tab_index(val)));
        self
    }
    pub fn access_key(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::access_key(val)));
        self
    }
    pub fn draggable(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::draggable(val)));
        self
    }
    pub fn content_editable(mut self, val: impl Into<String>) -> Self {
        let val = val.into();
        self.0
            .push_back(HtmlProp::Shared(SharedProp::content_editable(val)));
        self
    }
    pub fn spellcheck(mut self, val: bool) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::spellcheck(val)));
        self
    }
    pub fn oncopy(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::oncopy(val)));
        self
    }
    pub fn oncut(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::oncut(val)));
        self
    }
    pub fn onpaste(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onpaste(val)));
        self
    }
    pub fn onabort(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onabort(val)));
        self
    }
    pub fn onblur(mut self, val: Callback<FocusEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onblur(val)));
        self
    }
    pub fn onfocus(mut self, val: Callback<FocusEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onfocus(val)));
        self
    }
    pub fn onauxclick(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onauxclick(val)));
        self
    }
    pub fn oncanplay(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::oncanplay(val)));
        self
    }
    pub fn oncanplaythrough(mut self, val: Callback<Event>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::oncanplaythrough(val)));
        self
    }
    pub fn onchange(mut self, val: Callback<Event>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onchange(val)));
        self
    }
    pub fn onclick(mut self, val: Callback<MouseEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onclick(val)));
        self
    }
    pub fn onclose(mut self, val: Callback<Event>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onclose(val)));
        self
    }
    pub fn oncontextmenu(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::oncontextmenu(val)));
        self
    }
    pub fn ondblclick(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondblclick(val)));
        self
    }
    pub fn ondrag(mut self, val: Callback<DragEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::ondrag(val)));
        self
    }
    pub fn ondragend(mut self, val: Callback<DragEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondragend(val)));
        self
    }
    pub fn ondragenter(mut self, val: Callback<DragEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondragenter(val)));
        self
    }
    pub fn ondragexit(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondragexit(val)));
        self
    }
    pub fn ondragleave(mut self, val: Callback<DragEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondragleave(val)));
        self
    }
    pub fn ondragover(mut self, val: Callback<DragEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondragover(val)));
        self
    }
    pub fn ondragstart(mut self, val: Callback<DragEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondragstart(val)));
        self
    }
    pub fn ondrop(mut self, val: Callback<DragEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::ondrop(val)));
        self
    }
    pub fn ondurationchange(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ondurationchange(val)));
        self
    }
    pub fn onemptied(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onemptied(val)));
        self
    }
    pub fn onended(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onended(val)));
        self
    }
    pub fn oninput(mut self, val: Callback<InputEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::oninput(val)));
        self
    }
    pub fn oninvalid(mut self, val: Callback<Event>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::oninvalid(val)));
        self
    }
    pub fn onkeydown(mut self, val: Callback<KeyboardEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onkeydown(val)));
        self
    }
    pub fn onkeypress(mut self, val: Callback<KeyboardEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onkeypress(val)));
        self
    }
    pub fn onkeyup(mut self, val: Callback<KeyboardEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onkeyup(val)));
        self
    }
    pub fn onload(mut self, val: Callback<Event>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onload(val)));
        self
    }
    pub fn onloadeddata(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onloadeddata(val)));
        self
    }
    pub fn onloadedmetadata(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onloadedmetadata(val)));
        self
    }
    pub fn onloadend(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onloadend(val)));
        self
    }
    pub fn onloadstart(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onloadstart(val)));
        self
    }
    pub fn onmousedown(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onmousedown(val)));
        self
    }
    pub fn onmouseenter(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onmouseenter(val)));
        self
    }
    pub fn onmouseleave(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onmouseleave(val)));
        self
    }
    pub fn onmousemove(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onmousemove(val)));
        self
    }
    pub fn onmouseout(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onmouseout(val)));
        self
    }
    pub fn onmouseover(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onmouseover(val)));
        self
    }
    pub fn onmouseup(mut self, val: Callback<MouseEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onmouseup(val)));
        self
    }
    pub fn onwheel(mut self, val: Callback<WheelEvent>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onwheel(val)));
        self
    }
    pub fn onpause(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onpause(val)));
        self
    }
    pub fn onplay(mut self, val: Callback) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onplay(val)));
        self
    }
    pub fn onplaying(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onplaying(val)));
        self
    }
    pub fn onprogress(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onprogress(val)));
        self
    }
    pub fn onratechange(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onratechange(val)));
        self
    }
    pub fn onreset(mut self, val: Callback<Event>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onreset(val)));
        self
    }
    pub fn onresize(mut self, val: Callback<UiEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onresize(val)));
        self
    }
    pub fn onscroll(mut self, val: Callback<Event>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onscroll(val)));
        self
    }
    pub fn onseeked(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onseeked(val)));
        self
    }
    pub fn onseeking(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onseeking(val)));
        self
    }
    pub fn onselect(mut self, val: Callback<UiEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onselect(val)));
        self
    }
    pub fn onshow(mut self, val: Callback<Event>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onshow(val)));
        self
    }
    pub fn onstalled(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onstalled(val)));
        self
    }
    pub fn onsubmit(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onsubmit(val)));
        self
    }
    pub fn onsuspend(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onsuspend(val)));
        self
    }
    pub fn ontimeupdate(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontimeupdate(val)));
        self
    }
    pub fn onvolumechange(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onvolumechange(val)));
        self
    }
    pub fn onwaiting(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onwaiting(val)));
        self
    }
    pub fn onselectstart(mut self, val: Callback<FocusEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onselectstart(val)));
        self
    }
    pub fn ontoggle(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontoggle(val)));
        self
    }
    pub fn onpointercancel(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointercancel(val)));
        self
    }
    pub fn onpointerdown(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointerdown(val)));
        self
    }
    pub fn onpointerup(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointerup(val)));
        self
    }
    pub fn onpointermove(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointermove(val)));
        self
    }
    pub fn onpointerout(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointerout(val)));
        self
    }
    pub fn onpointerover(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointerover(val)));
        self
    }
    pub fn onpointerenter(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointerenter(val)));
        self
    }
    pub fn onpointerleave(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onpointerleave(val)));
        self
    }
    pub fn ongotpointercapture(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ongotpointercapture(val)));
        self
    }
    pub fn onlostpointercapture(mut self, val: Callback<PointerEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onlostpointercapture(val)));
        self
    }
    pub fn onanimationcancel(mut self, val: Callback<AnimationEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onanimationcancel(val)));
        self
    }
    pub fn onanimationend(mut self, val: Callback<AnimationEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onanimationend(val)));
        self
    }
    pub fn onanimationiteration(mut self, val: Callback<AnimationEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onanimationiteration(val)));
        self
    }
    pub fn onanimationstart(mut self, val: Callback<AnimationEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onanimationstart(val)));
        self
    }
    pub fn ontransitioncancel(mut self, val: Callback<TransitionEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontransitioncancel(val)));
        self
    }
    pub fn ontransitionend(mut self, val: Callback<TransitionEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontransitionend(val)));
        self
    }
    pub fn ontransitionrun(mut self, val: Callback<TransitionEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontransitionrun(val)));
        self
    }
    pub fn ontransitionstart(mut self, val: Callback<TransitionEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontransitionstart(val)));
        self
    }
    pub fn onwebkitanimationend(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onwebkitanimationend(val)));
        self
    }
    pub fn onwebkitanimationiteration(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onwebkitanimationiteration(
                val,
            )));
        self
    }
    pub fn onwebkitanimationstart(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onwebkitanimationstart(val)));
        self
    }
    pub fn onwebkittransitionend(mut self, val: Callback) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::onwebkittransitionend(val)));
        self
    }
    pub fn onerror(mut self, val: Callback<Event>) -> Self {
        self.0.push_back(HtmlProp::Shared(SharedProp::onerror(val)));
        self
    }
    pub fn ontouchstart(mut self, val: Callback<TouchEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontouchstart(val)));
        self
    }
    pub fn ontouchend(mut self, val: Callback<TouchEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontouchend(val)));
        self
    }
    pub fn ontouchmove(mut self, val: Callback<TouchEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontouchmove(val)));
        self
    }
    pub fn ontouchcancel(mut self, val: Callback<TouchEvent>) -> Self {
        self.0
            .push_back(HtmlProp::Shared(SharedProp::ontouchcancel(val)));
        self
    }
}

#[derive(Clone, PartialEq)]
pub enum EmptyPropEnum {}

#[sealed::sealed]
impl crate::elem::PropEnum<HtmlElement> for EmptyPropEnum {
    fn set_on(&self, _elem: &HtmlElement) {}
    fn unset_on(&self, _elem: &HtmlElement) {}
}

#[sealed::sealed]
impl crate::elem::HtmlComponent for HtmlElement {
    type PropEnum = EmptyPropEnum;
}
