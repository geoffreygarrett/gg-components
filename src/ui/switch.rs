use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;
use crate::cn;
use crate::ui::radix_leptos_switch::primitive as SwitchPrimitive;

#[component]
#[allow(non_snake_case)]
pub fn Switch(
    #[prop(into, optional)] checked: MaybeProp<bool>,
    #[prop(into, optional)] default_checked: MaybeProp<bool>,
    #[prop(into, optional)] on_checked_change: MaybeCallback<bool>,
    #[prop(optional, default=false)] disabled: bool,
    #[prop(optional, into)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <SwitchPrimitive::Root
            checked=checked
            default_checked=default_checked
            on_checked_change=on_checked_change
            disabled=disabled
            class=cn!(
                "peer inline-flex h-6 w-11 shrink-0 cursor-pointer items-center rounded-full border-2 border-transparent transition-colors focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring focus-visible:ring-offset-2 focus-visible:ring-offset-background disabled:cursor-not-allowed disabled:opacity-50 data-[state=checked]:bg-primary data-[state=unchecked]:bg-input",
                    class.get().unwrap_or_default()
            )
        >
            <SwitchPrimitive::Thumb class="pointer-events-none block h-5 w-5 rounded-full bg-background shadow-lg ring-0 transition-transform data-[state=checked]:translate-x-5 data-[state=unchecked]:translate-x-0" />
        </SwitchPrimitive::Root>
    }
}
