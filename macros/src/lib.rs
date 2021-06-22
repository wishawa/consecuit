use std::convert::TryFrom;

use proc_macro2::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, parse_quote, ExprBlock, ExprPath,
};
use syn_rsx::Node;

#[proc_macro]
pub fn reia_tree(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let nodes = parse_macro_input!(input as ReiaNodes);
    let tokens = nodes.build_top();
    tokens.into()
}

struct ReiaNodes(Vec<ReiaNode>);

struct ReiaNode {
    func: ExprPath,
    props: ExprBlock,
    children: ReiaNodes,
}

impl Parse for ReiaNodes {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let parser_cfg = syn_rsx::ParserConfig::default();
        let parser = syn_rsx::Parser::new(parser_cfg);
        let nodes: Vec<Node> = parser.parse(input)?;
        let nodes: syn::Result<Vec<ReiaNode>> = nodes.into_iter().map(ReiaNode::try_from).collect();
        let nodes = nodes?;
        Ok(ReiaNodes(nodes))
    }
}

impl TryFrom<Node> for ReiaNode {
    type Error = syn::Error;
    fn try_from(node: Node) -> syn::Result<Self> {
        fn expr_into_text_node(expr: syn::Expr) -> ReiaNode {
            let func = parse_quote!(::reia_html::misc_components::text_node);
            let props = parse_quote!({#expr});
            ReiaNode {
                func,
                props,
                children: ReiaNodes(Vec::new()),
            }
        }

        let Node {
            name,
            node_type,
            value,
            attributes,
            children,
        } = node;
        let res = match node_type {
            syn_rsx::NodeType::Element => {
                let name = name.unwrap();
                let func = if let syn_rsx::NodeName::Path(path) = &name {
                    Ok(path.clone())
                } else {
                    Err(syn::Error::new(name.span(), "Invalid component"))
                }?;
                let props = if let Some(attribute) = attributes.get(0) {
                    if let syn::Expr::Block(block) = attribute.value.clone().unwrap() {
                        Ok(block)
                    } else {
                        Err(syn::Error::new(name.span(), "Invalid props"))
                    }
                } else {
                    Ok(parse_quote!({ ::std::default::Default::default() }))
                }?;
                let children: syn::Result<Vec<Self>> =
                    children.into_iter().map(ReiaNode::try_from).collect();
                let children = ReiaNodes(children?);
                Self {
                    func,
                    props,
                    children,
                }
            }
            syn_rsx::NodeType::Attribute => panic!(),
            syn_rsx::NodeType::Text => expr_into_text_node(value.unwrap()),
            syn_rsx::NodeType::Comment => todo!(),
            syn_rsx::NodeType::Doctype => todo!(),
            syn_rsx::NodeType::Fragment => todo!(),
            syn_rsx::NodeType::Block => expr_into_text_node(value.unwrap()),
        };
        Ok(res)
    }
}

impl ReiaNodes {
    fn build_top(&self) -> TokenStream {
        let tokens = self.build();
        quote! {
            #[allow(unused_braces)]
            #tokens
        }
    }
    fn build(&self) -> TokenStream {
        let func = self.0.iter().map(|n| &n.func);
        let props = self.0.iter().map(|n| &n.props);
        let children = self.0.iter().map(|n| {
            let child_nodes = &n.children;
            if child_nodes.0.is_empty() {
                quote! {}
            } else {
                let inner = child_nodes.build();
                quote! {
                    .child(|reia| {
                        #inner
                    })
                }
            }
        });
        quote! {
            reia
            #(.comp(#func, #props)#children)*
        }
    }
}
