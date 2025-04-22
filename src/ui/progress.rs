// "use client"

// import * as React from "react"
// import * as ProgressPrimitive from "@radix-ui/react-progress"
//
// import { cn } from "@/lib/utils"
//
// const Progress = React.forwardRef<
//     React.ElementRef<typeof ProgressPrimitive.Root>,
// React.ComponentPropsWithoutRef<typeof ProgressPrimitive.Root>
// >(({ className, value, ...props }, ref) => (
// <ProgressPrimitive.Root
// ref={ref}
// className={cn(
// "relative h-2 w-full overflow-hidden rounded-full bg-primary/20",
// className
// )}
// {...props}
// >
// <ProgressPrimitive.Indicator
// className="h-full w-full flex-1 bg-primary transition-all"
// style={{ transform: `translateX(-${100 - (value || 0)}%)` }}
// />
// </ProgressPrimitive.Root>
// ))
// Progress.displayName = ProgressPrimitive.Root.displayName
//
// export { Progress }
use leptos::prelude::*;
use radix_leptos_progress::primitive as ProgressPrimitive;
use crate::cn;

#[component]
pub fn Progress(
    #[prop(into, optional)] class: MaybeProp<String>,
    #[prop(into, optional)] value: MaybeProp<f64>,
) -> impl IntoView {
    view! {
        <ProgressPrimitive::Root
            attr:class=cn!("relative h-2 w-full overflow-hidden rounded-full bg-primary/20", class.get().unwrap_or_default())
            value=value
        >
            <ProgressPrimitive::Indicator
                attr:class="h-full w-full flex-1 bg-primary transition-all"
                style:transform=move || format!("translateX(-{}%)", 100.0 - value.get().unwrap_or(0.0))
            />
        </ProgressPrimitive::Root>
    }
}