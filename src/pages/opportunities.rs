#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;
use crate::services::api::ApiService;
use crate::components::skeleton::Skeleton;

#[component]
pub fn Opportunities() -> Element {
    let navigator = use_navigator();

    let opportunities_resource = use_resource(move || async move {
        let api = ApiService::new();
        api.get_opportunities().await
    });

    rsx! {
        div {
            class: "bg-background-light text-slate-800 font-display antialiased h-screen flex justify-center bg-gray-100",

            // Mobile Container
            div {
                class: "w-full max-w-md h-full bg-background-light relative flex flex-col overflow-hidden shadow-2xl mx-auto border-x border-gray-200",

                // Header Section
                header {
                    class: "pt-12 pb-4 px-6 bg-white sticky top-0 z-20 border-b border-gray-100 shadow-[0_2px_10px_-4px_rgba(0,0,0,0.05)]",
                    div {
                        class: "flex items-center justify-between mb-4",
                        button {
                            class: "p-2 -ml-2 rounded-full hover:bg-gray-50 text-slate-500 transition-colors",
                            onclick: move |_| { navigator.go_back(); },
                            span { class: "material-icons text-2xl", "arrow_back_ios_new" }
                        }
                        div {
                            class: "flex gap-3",
                            button {
                                class: "p-2 rounded-full hover:bg-gray-50 text-slate-500 transition-colors",
                                span { class: "material-icons text-2xl", "tune" }
                            }
                            button {
                                class: "relative p-2 rounded-full hover:bg-gray-50 text-slate-500 transition-colors",
                                span { class: "material-icons text-2xl", "notifications_none" }
                                span { class: "absolute top-2 right-2 w-2 h-2 bg-red-500 rounded-full border border-white" }
                            }
                        }
                    }
                    h1 { class: "text-3xl font-extrabold text-slate-900 tracking-tight mb-4", "Opportunities" }

                    // Search Bar
                    div {
                        class: "relative group",
                        div {
                            class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                            span { class: "material-icons text-gray-400 group-focus-within:text-primary transition-colors", "search" }
                        }
                        input {
                            class: "block w-full pl-10 pr-3 py-3 border border-gray-200 rounded-xl leading-5 bg-gray-50 placeholder-gray-400 focus:outline-none focus:bg-white focus:ring-2 focus:ring-primary/20 focus:border-primary transition duration-150 ease-in-out sm:text-sm",
                            placeholder: "Search positions, companies...",
                            r#type: "text"
                        }
                    }
                }

                // Main Content Area
                main {
                    class: "flex-1 overflow-y-auto scrollbar-hide p-6 space-y-4",

                    match &*opportunities_resource.read() {
                        Some(Ok(opportunities)) => {
                            let opportunities = opportunities.clone();
                            rsx! {
                                for opp in opportunities {
                                    div {
                                        class: "bg-white rounded-xl p-5 shadow-sm border border-gray-100 relative group active:scale-[0.99] transition-transform duration-100",
                                    div {
                                        class: "flex justify-between items-start mb-3",
                                        div {
                                            class: "flex items-center gap-3",
                                            div {
                                                class: "w-10 h-10 rounded-lg bg-gray-50 flex items-center justify-center border border-gray-100 overflow-hidden",
                                                span { class: "material-icons text-gray-400", "business" }
                                            }
                                            div {
                                                h3 { class: "text-lg font-bold text-slate-900 leading-tight", "{opp.title}" }
                                                p { class: "text-xs text-slate-500 font-medium mt-0.5", "{opp.company_name.clone().unwrap_or_default()}" }
                                            }
                                        }
                                        span {
                                            class: "inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-bold bg-green-50 text-green-700 border border-green-100",
                                            "{opp.status}"
                                        }
                                    }
                                    div {
                                        class: "flex items-center gap-4 mt-4 border-t border-gray-50 pt-3",
                                        span { class: "text-xs text-slate-400", "Posted: {opp.created_at.clone().unwrap_or_default()}" }
                                    }
                                }
                            }
                        }},
                        Some(Err(err)) => rsx! {
                            div { class: "text-red-500 text-center p-4", "Error loading opportunities: {err.message}" }
                        },
                        None => rsx! {
                            // Skeleton Loading
                            for _ in 0..3 {
                                div {
                                    class: "bg-white rounded-xl p-5 shadow-sm border border-gray-100",
                                    div {
                                        class: "flex justify-between mb-3",
                                        div {
                                            class: "flex items-center gap-3",
                                            Skeleton { width: "w-10", height: "h-10", class: "rounded-lg" }
                                            div {
                                                class: "space-y-2",
                                                Skeleton { width: "w-32", height: "h-5", class: "rounded" }
                                                Skeleton { width: "w-20", height: "h-3", class: "rounded" }
                                            }
                                        }
                                        Skeleton { width: "w-16", height: "h-6", class: "rounded-lg" }
                                    }
                                    Skeleton { width: "w-full", height: "h-8", class: "mt-4 rounded" }
                                }
                            }
                        }
                    }
                }

                // Floating Action Button
                button {
                    class: "absolute bottom-24 right-6 w-14 h-14 bg-primary rounded-full shadow-lg shadow-primary/40 flex items-center justify-center text-white active:scale-95 transition-transform duration-200 z-30",
                    span { class: "material-icons text-3xl", "add" }
                }

                // Tab Bar
                nav {
                    class: "bg-white border-t border-gray-100 px-6 py-3 pb-6 flex justify-between items-center z-20",
                    Link {
                        to: Route::Candidates {},
                        class: "flex flex-col items-center gap-1 text-slate-400 hover:text-primary transition-colors group",
                        span { class: "material-icons text-2xl group-hover:scale-110 transition-transform", "people_outline" }
                        span { class: "text-[10px] font-semibold", "Candidates" }
                    }
                    Link {
                        to: Route::Contacts {},
                        class: "flex flex-col items-center gap-1 text-slate-400 hover:text-primary transition-colors group",
                        span { class: "material-icons text-2xl group-hover:scale-110 transition-transform", "contacts" }
                        span { class: "text-[10px] font-semibold", "Contacts" }
                    }
                    // Active State
                    Link {
                        to: Route::Opportunities {},
                        class: "flex flex-col items-center gap-1 text-primary transition-colors group",
                        div {
                            class: "bg-primary/10 px-4 py-1 rounded-full mb-1",
                            span { class: "material-icons text-2xl", "work" }
                        }
                    }
                    Link {
                        to: Route::Interactions {},
                        class: "flex flex-col items-center gap-1 text-slate-400 hover:text-primary transition-colors group",
                        span { class: "material-icons text-2xl group-hover:scale-110 transition-transform", "chat_bubble_outline" }
                        span { class: "text-[10px] font-semibold", "Messages" }
                    }
                    Link {
                        to: Route::Menu {},
                        class: "flex flex-col items-center gap-1 text-slate-400 hover:text-primary transition-colors group",
                        span { class: "material-icons text-2xl group-hover:scale-110 transition-transform", "more_horiz" }
                        span { class: "text-[10px] font-semibold", "More" }
                    }
                }
            }
        }
    }
}
