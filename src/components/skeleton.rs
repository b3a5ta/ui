use dioxus::prelude::*;

#[component]
pub fn Skeleton(
    #[props(default = "w-full".to_string())]
    width: String,
    #[props(default = "h-4".to_string())]
    height: String,
    #[props(default = "".to_string())]
    class: String,
) -> Element {
    rsx! {
        div {
            class: "animate-pulse bg-gray-200 dark:bg-gray-700 rounded-md {width} {height} {class}",
        }
    }
}
