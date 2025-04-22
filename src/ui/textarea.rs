use leptos::html;
use leptos::prelude::*;
use crate::cn;

#[component]
#[allow(non_snake_case)]
pub fn Textarea(
    #[prop(into, optional)] value: MaybeProp<String>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<html::Textarea>,
) -> impl IntoView {
    view! {
        <textarea
            node_ref=node_ref
            class=cn!(
                "flex min-h-[60px] w-full rounded-md border border-input bg-transparent px-3 py-2 text-sm shadow-sm placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50", class.get()
            )
        >
            {move || value.get().unwrap_or_default()}
        </textarea>
    }
}