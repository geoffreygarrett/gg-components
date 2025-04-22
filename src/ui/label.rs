use leptos::{prelude::*, html};
use tailwind_fuse::*;
use radix_leptos_label::primitive as LabelPrimitive;

#[derive(TwClass)]
pub(crate) struct LabelStyles {
    variant: LabelVariant,
}

#[derive(TwVariant)]
pub enum LabelVariant {
    #[tw(default, class="text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70")]
    Default,
}

#[component]
#[allow(non_snake_case)]
pub fn Label<C: IntoView + 'static>(
    children: TypedChildrenFn<C>,
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] node_ref: NodeRef<html::Label>,
) -> impl IntoView {
    let class = Signal::derive(move || LabelStyles::builder()
        .variant(LabelVariant::Default)
        .with_class(class.get().unwrap_or_default())
    );

    view! {
        <LabelPrimitive::Root node_ref=node_ref children=children attr:class=move || class.get() />
    }
}