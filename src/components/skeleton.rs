use dioxus::prelude::*;

#[derive(Props, Clone, PartialEq)]
pub struct SkeletonProps {
    #[props(default = "w-full".to_string())]
    pub width: String,
    #[props(default = "h-4".to_string())]
    pub height: String,
    #[props(default = "".to_string())]
    pub class: String,
}

#[component]
pub fn Skeleton(props: SkeletonProps) -> Element {
    rsx! {
        div {
            class: "animate-pulse bg-gray-200 dark:bg-gray-700 rounded-md {props.width} {props.height} {props.class}",
        }
    }
}
