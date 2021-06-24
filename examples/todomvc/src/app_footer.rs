use consecuit::prelude::*;
use consecuit_html::prelude::*;

fn footer_span(cc: ComponentBuilder, _: ()) -> impl ContainerReturn {
    let cc = cc.init();
    cc_tree! (
        <span {html_props().class_name("m-0.5 text-xs text-center text-gray-500")} HOLE />
    )
}

fn hoverable_link(
    cc: ComponentBuilder,
    (text, href): (&'static str, &'static str),
) -> impl ComponentReturn {
    let cc = cc.init();
    cc_tree!(
        <a {HtmlProps::<web_sys::HtmlAnchorElement>::new().class_name("hover:underline").href(href)}>
            {text}
        </a>
    )
}

pub fn app_footer(cc: ComponentBuilder, _: ()) -> impl ComponentReturn {
    let cc = cc.init();
    cc_tree!(
        <footer {html_props().class_name("mt-16 p-4 flex flex-col")}>
            <footer_span>
                "Written by "
                <hoverable_link {("Wisha Wa", "https://github.com/wishawa/")} />
            </footer_span>
            <footer_span>
                "with "
                <hoverable_link {("Consecuit", "https://github.com/wishawa/consecuit")} />
                " and "
                <hoverable_link {("Tailwind CSS", "https://tailwindcss.com/")} />
            </footer_span>
            <footer_span>
                "to the "
                <hoverable_link {("TodoMVC", "https://todomvc.com/")} />
                " "
                <hoverable_link {("specification", "https://github.com/tastejs/todomvc/blob/master/app-spec.md")} />
            </footer_span>
        </footer>
    )
}
