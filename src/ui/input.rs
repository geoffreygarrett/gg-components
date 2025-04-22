use leptos::{
    html::Input as HtmlInput,
    prelude::*,
};
use crate::cn;

#[derive(Clone)]
#[allow(unused)]
pub enum InputBinding<V = String> {
    /// Text-like inputs (text, email, password, etc.) with a string signal
    Text {
        signal: RwSignal<String>,
        variant: TextVariant,
    },
    /// Numeric inputs (number, range) with numeric signal
    Number {
        signal: RwSignal<f64>,
        variant: NumberVariant,
    },
    /// Checkbox with boolean signal
    Checkbox(RwSignal<bool>),
    /// Radio with value and group signal
    Radio {
        value: V,
        group: RwSignal<V>,
    },
    /// File input (typically doesn't use value binding)
    File,
}

#[derive(Clone, Default)]
pub enum TextVariant {
    #[default]
    Text,
    Email,
    Password,
    Search,
    Tel,
    Url,
    Date,
    Time,
    Color,
}

#[derive(Clone, Default)]
pub enum NumberVariant {
    #[default]
    Number,
    Range,
}

impl<V> Default for InputBinding<V> {
    fn default() -> Self {
        Self::Text {
            signal: RwSignal::new(String::new()),
            variant: TextVariant::default(),
        }
    }
}


#[allow(unused)]
impl InputBinding {
    // Text variants
    pub fn text(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Text,
        }
    }

    pub fn email(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Email,
        }
    }

    pub fn password(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Password,
        }
    }

    pub fn search(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Search,
        }
    }

    pub fn tel(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Tel,
        }
    }

    pub fn url(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Url,
        }
    }

    pub fn date(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Date,
        }
    }

    pub fn time(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Time,
        }
    }

    pub fn color(signal: RwSignal<String>) -> Self {
        Self::Text {
            signal,
            variant: TextVariant::Color,
        }
    }

    // Number variants
    pub fn number(signal: RwSignal<f64>) -> Self {
        Self::Number {
            signal,
            variant: NumberVariant::Number,
        }
    }

    pub fn range(signal: RwSignal<f64>) -> Self {
        Self::Number {
            signal,
            variant: NumberVariant::Range,
        }
    }

    // Checkbox
    pub fn checkbox(signal: RwSignal<bool>) -> Self {
        Self::Checkbox(signal)
    }

    // Radio
    // pub fn radio<V: Clone>(value: V, group: RwSignal<V>) -> Self {
    //     Self::Radio { value, group }
    // }

    // File
    pub fn file() -> Self {
        Self::File
    }
}

// Convenient trait implementations for automatic conversion
#[allow(unused)]
pub trait IntoInputBinding<V = String> {
    fn into_binding(self) -> InputBinding<V>;
}

impl IntoInputBinding for RwSignal<String> {
    fn into_binding(self) -> InputBinding {
        InputBinding::text(self)
    }
}

impl IntoInputBinding for RwSignal<f64> {
    fn into_binding(self) -> InputBinding {
        InputBinding::number(self)
    }
}

impl IntoInputBinding for RwSignal<bool> {
    fn into_binding(self) -> InputBinding {
        InputBinding::checkbox(self)
    }
}

#[component]
#[allow(non_snake_case)]
pub fn Input(
    #[prop(into, optional)] node_ref: NodeRef<HtmlInput>,
    #[prop(into, optional)] class: MaybeProp<String>,
) -> impl IntoView {
    view! {
        <input
            node_ref=node_ref
            class=cn!(
                "flex h-9 w-full rounded-md border border-input bg-transparent px-3 py-1 text-base shadow-sm transition-colors file:border-0 file:bg-transparent file:text-sm file:font-medium file:text-foreground placeholder:text-muted-foreground focus-visible:outline-none focus-visible:ring-1 focus-visible:ring-ring disabled:cursor-not-allowed disabled:opacity-50 md:text-sm", class.get()
            )
        />
    }
}

// use leptos::attr::{checked, r#type, value};
// use leptos::prelude::*;
// use leptos::html;
// use leptos::tachys::reactive_graph::bind::bind;
// use leptos::web_sys::console::group;
// use leptos_node_ref::AnyNodeRef;
//
// // #[component]
// // #[allow(non_snake_case)]
// // pub fn Label<C: IntoView + 'static>(
// //     children: TypedChildren<C>,
// //     #[prop(into, optional)] r#for: MaybeProp<String>,
// //     #[prop(into, optional)] node_ref: AnyNodeRef,
// // ) -> impl IntoView {
// //     view! {
// //         <label
// //             node_ref={node_ref}
// //             r#for={r#for}
// //             class=("text-sm", true)
// //             class=("font-medium", true)
// //             class=("leading-none", true)
// //             class=("peer-disabled:cursor-not-allowed", true)
// //             class=("peer-disabled:opacity-70", true)
// //         >
// //             {children.into_inner()()}
// //         </label>
// //     }
// // }
// use leptos::*;
// use leptos::attr::any_attribute::IntoAnyAttribute;
// use leptos::html::Input as HtmlInput;
//
// #[derive(Clone)]
// pub enum InputBinding<V = String> {
//     /// Checkbox binding
//     Checked(RwSignal<bool>),
//     /// Radio binding (we usually need `value` + `bind:group`)
//     Radio {
//         value: V,
//         group: RwSignal<V>,
//     },
//     /// Text-like binding
//     Value(RwSignal<String>),
// }
//
//
// impl<V> Default for InputBinding<V> {
//     fn default() -> Self {
//         Self::Value(RwSignal::new(String::new()))
//     }
// }
//
// // use leptos::*;
// // use leptos::html::Input as HtmlInput;
//
// use leptos::attr::NextAttribute;
//
// #[component]
// #[allow(non_snake_case)]
// pub fn Input(
//     /// Optional node ref if needed
//     #[prop(into, optional)] node_ref: NodeRef<HtmlInput>,
//     /// The "type" of binding we want
//     #[prop(into, optional)] binding: InputBinding,
// ) -> impl IntoView {
//     view! {
//         <input
//             node_ref=node_ref
//             class=([
//                 "flex",
//                 "h-9",
//                 "w-full",
//                 "rounded-md",
//                 "border",
//                 "border-input",
//                 "bg-transparent",
//                 "px-3",
//                 "py-1",
//                 "text-base",
//                 "shadow-sm",
//                 "transition-colors",
//                 "file:border-0",
//                 "file:bg-transparent",
//                 "file:text-sm",
//                 "file:font-medium",
//                 "file:text-foreground",
//                 "placeholder:text-muted-foreground",
//                 "focus-visible:outline-none",
//                 "focus-visible:ring-1",
//                 "focus-visible:ring-ring",
//                 "disabled:cursor-not-allowed",
//                 "disabled:opacity-50",
//                 "md:text-sm"
//             ], true)
//             {..match binding {
//                 InputBinding::Checked(checked_signal) => {
//                     checked(checked_signal)
//                         .add_any_attr(r#type("checkbox"))
//                         .into_any_attr()
//                 }
//                 InputBinding::Radio { value: radio_value, group } => {
//                     r#type("radio")
//                         .add_any_attr(value(radio_value.clone()))
//                         .add_any_attr(checked(move || group.get() == radio_value))
//                         .into_any_attr()
//                 }
//                 InputBinding::Value(text_signal) => {
//                     value(text_signal)
//                         .add_any_attr(r#type("text"))
//                         .into_any_attr()
//                 }
//             }}
//         />
//     }
// }
// // #[allow(non_snake_case)]
// // pub fn FormField<C: IntoView + 'static>(
// //     #[prop(into, optional)] id: MaybeProp<String>,
// //     #[prop(into, optional)] label: MaybeProp<String>,
// //     #[prop(into, optional)] error: MaybeProp<String>,
// //     #[prop(into, optional)] required: MaybeProp<bool>,
// //     #[prop(into, optional)] value: MaybeProp<String>,
// //     #[prop(into, optional)] on_input: MaybeProp<Callback<String>>,
// //     #[prop(into, optional)] node_ref: NodeRef<html::Div>,
// // ) -> impl IntoView {
// //     view! {
// //         <div
// //             node_ref={node_ref}
// //             class=("grid", true)
// //             class=("gap-2", true)
// //         >
// //             <Show
// //                 when=move || label.get().is_some()
// //                 fallback=|| ()
// //             >
// //                 <Label r#for={id.get().unwrap_or_default()}>
// //                     {label.get().unwrap_or_default()}
// //                     <Show
// //                         when=move || required.get().unwrap_or_default()
// //                         fallback=|_| ()
// //                     >
// //                         <span class=("text-destructive", true)>"*"</span>
// //                     </Show>
// //                 </Label>
// //             </Show>
// //
// //             <Input
// //                 {..}
// //                 id={id.get().unwrap_or_default()}
// //                 required={required.get().unwrap_or_default()}
// //                 value={value.get().unwrap_or_default()}
// //                 on:input={on_input.get().unwrap_or_default()}
// //                 class=("border-destructive", move || error.get().is_some())
// //             />
// //
// //             <Show
// //                 when=move || error.get().is_some()
// //                 fallback=|| ()
// //             >
// //                 <p class=("text-sm", true) class=("text-destructive", true)>
// //                     {error.get().unwrap_or_default()}
// //                 </p>
// //             </Show>
// //         </div>
// //     }
// // }