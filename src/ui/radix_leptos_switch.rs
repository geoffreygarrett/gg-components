use leptos::prelude::*;
use leptos_maybe_callback::MaybeCallback;
use leptos_node_ref::AnyNodeRef;
use leptos::html;
use leptos::context::Provider;

use radix_leptos_context::create_context;
use radix_leptos_primitive::Primitive;

/// A context holding the switch state (checked/disabled) + callback for toggling.
#[derive(Clone)]
struct SwitchContextValue {
    checked: RwSignal<bool>,
    disabled: bool,
    #[allow(dead_code)] // TODO: remove this
    on_checked_change: Callback<bool>,
}

// We define a name for the “Root” like in your Tabs code
const SWITCH_NAME: &str = "Switch";

create_context!(
    context_type: SwitchContextValue,
    provider: SwitchProvider,
    hook: use_switch_context,
    root: SWITCH_NAME
);

/// The main “Radix Switch” (Root). If `checked` is Some(...), it’s controlled;
/// otherwise it’s uncontrolled using `default_checked`. We pass down a context
/// so the `<Thumb>` can read `checked`, `disabled`, etc.
#[component]
pub fn Switch(
    /// If fully controlled, pass `checked=Some(...)`
    #[prop(into, optional)]
    checked: MaybeProp<bool>,
    /// If uncontrolled, pass `default_checked`
    #[prop(into, optional)]
    default_checked: MaybeProp<bool>,
    /// Fired when toggled
    #[prop(into, optional)]
    on_checked_change: MaybeCallback<bool>,
    /// If true, switch is disabled
    #[prop(optional, default = false)]
    disabled: bool,
    /// Extra classes, e.g. "bg-gray-500", etc.
    #[prop(optional, into)]
    class: MaybeProp<String>,
    /// The child elements, typically a single <Thumb>
    children: TypedChildrenFn<impl IntoView + 'static>,
    /// If you want a NodeRef
    #[prop(optional, into)]
    node_ref: AnyNodeRef,
) -> impl IntoView {
    // 1) figure out initial state
    let on_checked_change = on_checked_change.as_callback();
    let children = StoredValue::new(children.into_inner());
    let initial = checked.get().unwrap_or(default_checked.get().unwrap_or(false));
    let checked_signal = RwSignal::new(initial);

    // 2) If external “checked” changes, override local
    Effect::new({
        let checked_signal = checked_signal.clone();
        move |_| {
            if let Some(external_val) = checked.get() {
                checked_signal.set(external_val);
            }
        }
    });

    // 3) The on_click toggles if not disabled
    let handle_click = move |_| {
        if !disabled {
            let new_val = !checked_signal.get();
            checked_signal.set(new_val);
            on_checked_change.run(new_val);
        }
    };

    // 4) Provide context so <Thumb> can read from it
    // provide_context(SwitchContextValue {
    //     checked: checked_signal,
    //     disabled,
    //     on_checked_change,
    // });

    // 5) Render the “Root” as a <button>
    view! {
        <SwitchProvider value=SwitchContextValue {
            checked: checked_signal,
            disabled,
            on_checked_change,
        }>
            <Primitive
                element={html::button}
                node_ref={node_ref}
                {..}
                type="button"
                role="switch"
                data-state=move || if checked_signal.get() { "checked" } else { "unchecked" }
                data-disabled=move || if disabled { Some("") } else { None }
                aria-checked=move || checked_signal.get().to_string()
                disabled=disabled
                class=move || {
                    format!("radix-leptos-switch-root {}", class.get().unwrap_or_default())
                }
                on:click=handle_click
            >
                {children.with_value(|c| c())}
            </Primitive>
        </SwitchProvider>
    }
}

// We define a name for the “Thumb”
const THUMB_NAME: &str = "SwitchThumb";

/// The “Thumb” that shows ON/OFF visually. Reads from `SwitchContextValue`.
#[component]
pub fn SwitchThumb(
    #[prop(optional, into)]
    class: MaybeProp<String>,
) -> impl IntoView {
    let SwitchContextValue { checked, disabled, .. } = use_switch_context(THUMB_NAME);

    // We'll compute data-state = "checked"/"unchecked" for styling
    let is_on = Signal::derive(move || checked.get());

    view! {
        <span
            role="presentation"
            data-state=move || if is_on.get() { "checked" } else { "unchecked" }
            data-disabled=move || if disabled { Some("") } else { None }
            class=move || format!("radix-leptos-switch-thumb {}", class.get().unwrap_or_default())
        />
    }
}

// Re-export them under “primitive::Root” and “primitive::Thumb”
pub mod primitive {
    pub use super::Switch as Root;
    pub use super::SwitchThumb as Thumb;
}
