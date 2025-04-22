use leptos::{prelude::*, html::{Div, H3, P}};
use crate::cn;

#[component]
#[allow(non_snake_case)]
pub fn Card(
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div
            node_ref=node_ref
            class=cn!("rounded-xl border bg-card text-card-foreground shadow", class.get())
        >
            {children()}
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CardHeader(
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=cn!("flex flex-col space-y-1.5 p-6", class.get())>
            {children()}
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CardTitle(
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<H3>,
) -> impl IntoView {
    view! {
        <h3 node_ref=node_ref class=cn!("font-semibold leading-none tracking-tight", class.get())>
            {children()}
        </h3>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CardDescription(
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<P>,
) -> impl IntoView {
    view! {
        <p node_ref=node_ref class=cn!("text-sm text-muted-foreground", class.get())>
            {children()}
        </p>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CardContent(
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=cn!("p-6 pt-0", class.get())>
            {children()}
        </div>
    }
}

#[component]
#[allow(non_snake_case)]
pub fn CardFooter(
    children: Children,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<Div>,
) -> impl IntoView {
    view! {
        <div node_ref=node_ref class=cn!("flex items-center p-6 pt-0", class.get())>
            {children()}
        </div>
    }
}