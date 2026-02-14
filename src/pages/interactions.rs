#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;
use crate::services::api::ApiService;
use crate::components::skeleton::Skeleton;

#[component]
pub fn Interactions() -> Element {
    let navigator = use_navigator();

    let interactions_resource = use_resource(move || async move {
        let api = ApiService::new();
        api.get_interactions().await
    });

    rsx! {
        div {
            class: "bg-gray-100 font-display flex items-center justify-center min-h-screen p-4",

            // Mobile Container
            div {
                class: "w-full max-w-md h-[850px] bg-background-light dark:bg-background-dark shadow-2xl rounded-[2.5rem] overflow-hidden relative flex flex-col border-4 border-white dark:border-gray-800",

                // Status Bar Area (Visual Only)
                div {
                    class: "h-12 w-full bg-background-light dark:bg-background-dark flex items-center justify-between px-6 pt-2 z-20 sticky top-0",
                    span { class: "text-sm font-semibold text-text-main dark:text-gray-200", "9:41" }
                    div {
                        class: "flex space-x-2 text-text-main dark:text-gray-200",
                        span { class: "material-icons-round text-sm", "signal_cellular_alt" }
                        span { class: "material-icons-round text-sm", "wifi" }
                        span { class: "material-icons-round text-sm", "battery_full" }
                    }
                }

                // Header
                header {
                    class: "px-6 pb-4 pt-2 bg-background-light dark:bg-background-dark z-20 sticky top-12 border-b border-gray-100 dark:border-gray-800",
                    div {
                        class: "flex items-center justify-between mb-4",
                        button {
                            class: "p-2 -ml-2 rounded-full hover:bg-primary/10 transition-colors text-text-main dark:text-gray-200",
                            onclick: move |_| { navigator.go_back(); },
                            span { class: "material-icons-round", "arrow_back_ios_new" }
                        }
                        div {
                            class: "text-center",
                            h1 { class: "text-lg font-bold text-text-main dark:text-white", "Interactions History" }
                            p { class: "text-xs text-primary font-medium", "Sarah Jenkins" }
                        }
                        button {
                            class: "p-2 -mr-2 rounded-full bg-primary/10 text-primary hover:bg-primary/20 transition-colors",
                            span { class: "material-icons-round", "add" }
                        }
                    }

                    // Filter Tabs
                    div {
                        class: "flex space-x-3 overflow-x-auto no-scrollbar pb-2",
                        button { class: "px-5 py-2 rounded-full bg-primary text-white text-sm font-semibold shadow-soft whitespace-nowrap transition-transform active:scale-95", "All" }
                        button { class: "px-5 py-2 rounded-full bg-white dark:bg-gray-800 text-text-main dark:text-gray-300 border border-gray-200 dark:border-gray-700 text-sm font-medium whitespace-nowrap hover:border-primary/50 transition-colors", "Calls" }
                        button { class: "px-5 py-2 rounded-full bg-white dark:bg-gray-800 text-text-main dark:text-gray-300 border border-gray-200 dark:border-gray-700 text-sm font-medium whitespace-nowrap hover:border-primary/50 transition-colors", "Emails" }
                        button { class: "px-5 py-2 rounded-full bg-white dark:bg-gray-800 text-text-main dark:text-gray-300 border border-gray-200 dark:border-gray-700 text-sm font-medium whitespace-nowrap hover:border-primary/50 transition-colors", "Meetings" }
                    }
                }

                // Content Area
                main {
                    class: "flex-1 overflow-y-auto px-6 py-4 relative timeline-line",

                    match &*interactions_resource.read() {
                        Some(Ok(interactions)) => {
                            let interactions = interactions.clone();
                            rsx! {
                                div {
                                    class: "relative z-10 mb-8",
                                    // Simplified grouping for demo - in real app, group by date
                                    h2 { class: "ml-14 text-xs font-bold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4", "Recent Activity" }

                                    for interaction in interactions {
                                        div {
                                            class: "flex mb-6 group",
                                        div {
                                            class: "flex-shrink-0 mr-4 relative",
                                            div {
                                                class: "w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center border-2 border-background-light dark:border-background-dark z-10 relative",
                                                span { class: "material-icons-round text-primary text-xl", "history" }
                                            }
                                        }
                                        div {
                                            class: "flex-1 bg-white dark:bg-[#232923] p-4 rounded-2xl shadow-card border border-gray-100 dark:border-gray-800 transition-all active:scale-[0.98]",
                                            div {
                                                class: "flex justify-between items-start mb-1",
                                                h3 { class: "font-bold text-text-main dark:text-white text-sm", "{interaction.title}" }
                                                span { class: "text-xs font-medium text-gray-400", "{interaction.time.clone().unwrap_or_default()}" }
                                            }
                                            p {
                                                class: "text-xs text-text-main/80 dark:text-gray-300 mb-2 leading-relaxed",
                                                "{interaction.description.clone().unwrap_or_default()}"
                                            }
                                        }
                                    }
                                }
                            }
                        }},
                        Some(Err(err)) => rsx! {
                            div { class: "text-red-500 text-center p-4", "Error loading interactions: {err.message}" }
                        },
                        None => rsx! {
                            // Skeleton Loading
                            for _ in 0..3 {
                                div {
                                    class: "flex mb-6",
                                    Skeleton { width: "w-12", height: "h-12", class: "rounded-full mr-4 shrink-0" }
                                    div {
                                        class: "flex-1 space-y-2",
                                        Skeleton { width: "w-full", height: "h-20", class: "rounded-2xl" }
                                    }
                                }
                            }
                        }
                    }

                    // End of list spacer
                    div {
                        class: "h-20 flex items-center justify-center text-gray-300 dark:text-gray-600",
                        span { class: "text-xs font-medium uppercase tracking-widest", "• End of History •" }
                    }
                }

                // Floating Action Button Area
                div {
                    class: "absolute bottom-6 right-6 z-30",
                    button {
                        class: "w-14 h-14 bg-primary text-white rounded-full shadow-lg shadow-primary/40 flex items-center justify-center hover:bg-primary-dark transition-all hover:scale-105 active:scale-95 group",
                        span { class: "material-icons-round text-2xl group-hover:rotate-90 transition-transform duration-300", "add" }
                    }
                }

                // Bottom Tab Bar (Visual Context)
                div {
                    class: "h-20 bg-white dark:bg-[#1f241f] border-t border-gray-100 dark:border-gray-800 flex items-center justify-around px-2 pb-4 z-20",
                    Link {
                        to: Route::Candidates {},
                        class: "flex flex-col items-center justify-center w-full h-full text-gray-400 cursor-pointer hover:text-primary transition-colors",
                        span { class: "material-icons-round mb-1", "dashboard" }
                        span { class: "text-[10px] font-medium", "Home" }
                    }
                    Link {
                        to: Route::Candidates {},
                        class: "flex flex-col items-center justify-center w-full h-full text-primary cursor-pointer",
                        span { class: "material-icons-round mb-1", "people" }
                        span { class: "text-[10px] font-medium", "Candidates" }
                    }
                    Link {
                        to: Route::Opportunities {},
                        class: "flex flex-col items-center justify-center w-full h-full text-gray-400 cursor-pointer hover:text-primary transition-colors",
                        span { class: "material-icons-round mb-1", "work" }
                        span { class: "text-[10px] font-medium", "Jobs" }
                    }
                    Link {
                        to: Route::Menu {},
                        class: "flex flex-col items-center justify-center w-full h-full text-gray-400 cursor-pointer hover:text-primary transition-colors",
                        span { class: "material-icons-round mb-1", "settings" }
                        span { class: "text-[10px] font-medium", "Settings" }
                    }
                }
            }
        }
    }
}
