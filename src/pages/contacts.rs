#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;
use crate::services::api::ApiService;
use crate::components::skeleton::Skeleton;

#[component]
pub fn Contacts() -> Element {
    let _navigator = use_navigator();

    let contacts_resource = use_resource(move || async move {
        let api = ApiService::new();
        api.get_contacts().await
    });

    rsx! {
        div {
            class: "bg-background-light dark:bg-background-dark font-display text-text-main dark:text-gray-100 min-h-screen flex justify-center",

            // Mobile Container
            div {
                class: "w-full max-w-md bg-background-light dark:bg-background-dark min-h-screen relative flex flex-col shadow-2xl mx-auto overflow-hidden",

                // Status Bar Area (Visual Spacer)
                div { class: "h-12 w-full bg-background-light dark:bg-background-dark shrink-0" }

                // Header
                header {
                    class: "px-6 pb-4 pt-2 flex items-center justify-between shrink-0 bg-background-light dark:bg-background-dark sticky top-0 z-20",
                    h1 { class: "text-3xl font-extrabold tracking-tight text-text-main dark:text-white", "Contacts" }
                    button {
                        class: "p-2 rounded-full hover:bg-primary/10 active:bg-primary/20 transition-colors text-primary",
                        span { class: "material-icons", "filter_list" }
                    }
                }

                // Search Bar
                div {
                    class: "px-6 pb-6 shrink-0 bg-background-light dark:bg-background-dark z-10",
                    div {
                        class: "relative group",
                        span {
                            class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                            span { class: "material-icons text-text-muted text-xl group-focus-within:text-primary transition-colors", "search" }
                        }
                        input {
                            class: "block w-full pl-10 pr-3 py-3 border-none ring-1 ring-gray-200 dark:ring-gray-700 rounded-lg bg-white dark:bg-surface-dark text-sm placeholder-text-muted focus:ring-2 focus:ring-primary focus:outline-none transition-shadow shadow-sm",
                            placeholder: "Search name, role, or company...",
                            r#type: "text"
                        }
                    }
                }

                // Scrollable Content
                main {
                    class: "flex-1 overflow-y-auto hide-scrollbar px-6 pb-24",

                    // Quick Actions / Top Contacts (Static for now)
                    section {
                        class: "mb-6",
                        h3 { class: "text-xs font-semibold uppercase tracking-wider text-text-muted mb-3 ml-1", "Recent" }
                        div {
                            class: "flex space-x-4 overflow-x-auto pb-4 hide-scrollbar -mx-6 px-6",
                            div {
                                class: "flex flex-col items-center space-y-2 min-w-[72px]",
                                div {
                                    class: "w-16 h-16 rounded-full bg-gradient-to-br from-primary to-primary-dark p-0.5 shadow-soft",
                                    div {
                                        class: "w-full h-full rounded-full border-2 border-white dark:border-surface-dark overflow-hidden flex items-center justify-center bg-primary/10",
                                        span { class: "text-primary font-bold text-lg", "S" }
                                    }
                                }
                                span { class: "text-xs font-medium text-center truncate w-full", "Sarah" }
                            }
                        }
                    }

                    // Contacts List
                    div {
                        class: "space-y-6",

                        match &*contacts_resource.read() {
                            Some(Ok(contacts)) => {
                                let contacts = contacts.clone();
                                rsx! {
                                    div {
                                        class: "bg-white dark:bg-surface-dark rounded-xl shadow-sm border border-gray-100 dark:border-gray-800 divide-y divide-gray-50 dark:divide-gray-800",
                                        for contact in contacts {
                                            div {
                                                class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                            div {
                                                class: "h-12 w-12 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold text-lg shrink-0 mr-4 group-hover:bg-primary group-hover:text-white transition-colors duration-300",
                                                "{contact.name.chars().take(2).collect::<String>().to_uppercase()}"
                                            }
                                            div {
                                                class: "flex-1 min-w-0",
                                                h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "{contact.name}" }
                                                p { class: "text-xs font-medium text-text-muted truncate", "{contact.job_title.clone().unwrap_or_default()}" }
                                                p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "{contact.email}" }
                                            }
                                            button {
                                                class: "text-gray-300 hover:text-primary dark:text-gray-600 dark:hover:text-primary transition-colors",
                                                span { class: "material-icons text-xl", "chevron_right" }
                                            }
                                        }
                                    }
                                }
                            }},
                            Some(Err(err)) => rsx! {
                                div { class: "text-red-500 text-center p-4", "Error loading contacts: {err.message}" }
                            },
                            None => rsx! {
                                // Skeleton Loaders
                                for _ in 0..5 {
                                    div {
                                        class: "flex items-center p-4",
                                        Skeleton { width: "w-12", height: "h-12", class: "rounded-full mr-4" }
                                        div {
                                            class: "flex-1 space-y-2",
                                            Skeleton { width: "w-32", height: "h-4", class: "rounded" }
                                            Skeleton { width: "w-24", height: "h-3", class: "rounded" }
                                        }
                                    }
                                }
                            }
                        }
                    }

                    div { class: "h-24" } // Spacer for FAB
                }

                // Floating Action Button
                button {
                    class: "absolute bottom-24 right-6 w-14 h-14 bg-primary text-white rounded-full shadow-lg shadow-primary/40 flex items-center justify-center hover:bg-primary-dark active:scale-95 transition-all duration-200 z-30 group",
                    span { class: "material-icons text-3xl group-hover:rotate-90 transition-transform", "add" }
                }

                // Bottom Navigation
                nav {
                    class: "bg-white dark:bg-surface-dark border-t border-gray-100 dark:border-gray-800 shrink-0 pb-6 pt-3 px-6 shadow-nav z-40",
                    ul {
                        class: "flex justify-between items-center",
                        li {
                            Link {
                                to: Route::Candidates {},
                                class: "flex flex-col items-center space-y-1 text-text-muted hover:text-primary transition-colors group",
                                span { class: "material-icons text-2xl group-hover:-translate-y-0.5 transition-transform", "people_outline" }
                                span { class: "text-[10px] font-medium", "Candidates" }
                            }
                        }
                        li {
                            Link {
                                to: Route::Contacts {},
                                class: "flex flex-col items-center space-y-1 text-primary relative",
                                div { class: "absolute -top-3 w-8 h-1 bg-primary rounded-full opacity-0" } // Active Indicator Placeholder
                                span { class: "material-icons text-2xl", "contacts" }
                                span { class: "text-[10px] font-bold", "Contacts" }
                            }
                        }
                        li {
                            Link {
                                to: Route::Opportunities {},
                                class: "flex flex-col items-center space-y-1 text-text-muted hover:text-primary transition-colors group",
                                span { class: "material-icons text-2xl group-hover:-translate-y-0.5 transition-transform", "work_outline" }
                                span { class: "text-[10px] font-medium", "Jobs" }
                            }
                        }
                        li {
                            Link {
                                to: Route::Menu {},
                                class: "flex flex-col items-center space-y-1 text-text-muted hover:text-primary transition-colors group",
                                span { class: "material-icons text-2xl group-hover:-translate-y-0.5 transition-transform", "settings" }
                                span { class: "text-[10px] font-medium", "Settings" }
                            }
                        }
                    }
                }

                // Bottom Home Indicator (iOS Style)
                div {
                    class: "absolute bottom-1 left-0 right-0 flex justify-center pointer-events-none z-50",
                    div { class: "w-1/3 h-1.5 bg-black/20 dark:bg-white/20 rounded-full" }
                }
            }
        }
    }
}
