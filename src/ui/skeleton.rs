use leptos::prelude::*;
use crate::cn;

#[component]
pub fn Skeleton(#[prop(into, optional)] class: MaybeProp<String>) -> impl IntoView {
    view! { <div class=cn!("animate-pulse rounded-md bg-primary/10", class.get()) /> }
}
