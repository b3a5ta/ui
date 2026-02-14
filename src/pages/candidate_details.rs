#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;
use crate::services::api::ApiService;
use crate::components::skeleton::Skeleton;

#[component]
pub fn CandidateDetails(id: String) -> Element {
    let navigator = use_navigator();

    let candidate_resource = use_resource(move || {
        let id_clone = id.clone();
        async move {
            let api = ApiService::new();
            api.get_candidate(&id_clone).await
        }
    });

    rsx! {
        div {
            class: "bg-background-light dark:bg-background-dark font-display text-gray-800 dark:text-gray-100 min-h-screen",

            // Mobile Container
            div {
                class: "max-w-md mx-auto min-h-screen relative pb-24 shadow-2xl bg-background-light dark:bg-background-dark overflow-hidden",

                // Header
                header {
                    class: "sticky top-0 z-30 bg-surface-light/90 dark:bg-surface-dark/90 backdrop-blur-md border-b border-primary/10 px-4 pt-12 pb-4 flex items-center justify-between",
                    button {
                        class: "p-2 -ml-2 rounded-full hover:bg-primary/10 text-gray-600 dark:text-gray-300 transition-colors",
                        onclick: move |_| { navigator.go_back(); },
                        span { class: "material-icons-round text-2xl", "arrow_back" }
                    }
                    h1 { class: "text-lg font-bold text-gray-900 dark:text-white", "Candidate Details" }
                    button {
                        class: "px-3 py-1 text-primary font-semibold text-sm hover:bg-primary/10 rounded-md transition-colors",
                        "Edit"
                    }
                }

                // Main Content
                main {
                    class: "p-4 space-y-4",

                    match &*candidate_resource.read() {
                        Some(Ok(candidate)) => {
                            let candidate = candidate.clone();
                            rsx! {
                                // Candidate Profile Card
                                div {
                                    class: "bg-surface-light dark:bg-surface-dark rounded-xl shadow-sm border border-primary/10 p-5 flex flex-col items-center text-center relative overflow-hidden",
                                    // Decorative background blob
                                    div { class: "absolute top-0 left-0 w-full h-20 bg-gradient-to-b from-primary/10 to-transparent" }
                                    div {
                                        class: "relative mt-2",
                                        // Using a placeholder/initials if no image
                                        div {
                                            class: "w-24 h-24 rounded-full bg-gray-200 flex items-center justify-center text-3xl font-bold text-gray-500 border-4 border-surface-light dark:border-surface-dark shadow-md",
                                            "{candidate.name.chars().next().unwrap_or('?')}"
                                        }
                                        span { class: "absolute bottom-1 right-1 w-5 h-5 bg-green-500 border-2 border-surface-light dark:border-surface-dark rounded-full" }
                                    }
                                    h2 { class: "mt-3 text-xl font-bold text-gray-900 dark:text-white", "{candidate.name}" }
                                    p { class: "text-sm text-gray-500 dark:text-gray-400 font-medium", "{candidate.current_designation.clone().unwrap_or_default()}" }
                                    div {
                                        class: "mt-3 flex items-center gap-2",
                                        span {
                                            class: "px-3 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-full border border-primary/20",
                                            "{candidate.status}"
                                        }
                                        if let Some(loc) = &candidate.location {
                                            span {
                                                class: "px-3 py-1 bg-gray-100 dark:bg-gray-800 text-gray-500 text-xs font-medium rounded-full",
                                                "{loc}"
                                            }
                                        }
                                    }
                                }

                                // Primary Action Bar
                                div {
                                    class: "grid grid-cols-4 gap-3",
                                    button {
                                        class: "col-span-3 bg-primary hover:bg-primary/90 text-white font-semibold py-3 px-4 rounded-lg shadow-sm shadow-primary/30 flex items-center justify-center gap-2 transition-all active:scale-[0.98]",
                                        span { class: "material-icons-round text-xl", "phone_in_talk" }
                                        " Call Confirmation"
                                    }
                                    button {
                                        class: "col-span-1 bg-surface-light dark:bg-surface-dark text-primary border border-primary/20 hover:bg-primary/5 rounded-lg flex items-center justify-center transition-colors",
                                        onclick: move |_| { navigator.push(Route::Interactions {}); },
                                        span { class: "material-icons-round text-2xl", "mail_outline" }
                                    }
                                }

                                // Tabs
                                div {
                                    class: "bg-surface-light dark:bg-surface-dark p-1 rounded-lg border border-primary/10 flex",
                                    button { class: "flex-1 py-2 text-sm font-semibold rounded-md bg-primary text-white shadow-sm transition-all", "Info" }
                                    button {
                                        class: "flex-1 py-2 text-sm font-medium text-gray-500 dark:text-gray-400 hover:text-primary transition-colors",
                                        onclick: move |_| { navigator.push(Route::Interactions {}); },
                                        "History"
                                    }
                                    button { class: "flex-1 py-2 text-sm font-medium text-gray-500 dark:text-gray-400 hover:text-primary transition-colors", "Notes" }
                                }

                                // Content Area (Active Tab: Info)
                                div {
                                    class: "space-y-4",

                                    // Contact Info
                                    div {
                                        class: "bg-surface-light dark:bg-surface-dark rounded-xl p-5 shadow-sm border border-primary/5",
                                        h3 { class: "text-sm font-bold text-gray-900 dark:text-white uppercase tracking-wider mb-4 opacity-70", "Contact Information" }
                                        div {
                                            class: "space-y-4",
                                            div {
                                                class: "flex items-start gap-3",
                                                div {
                                                    class: "p-2 bg-primary/10 rounded-lg text-primary",
                                                    span { class: "material-icons-round text-lg", "call" }
                                                }
                                                div {
                                                    p { class: "text-xs text-gray-400 font-medium", "Mobile" }
                                                    p { class: "text-sm font-semibold text-gray-800 dark:text-gray-200", "{candidate.phone}" }
                                                }
                                            }
                                            div {
                                                class: "flex items-start gap-3",
                                                div {
                                                    class: "p-2 bg-primary/10 rounded-lg text-primary",
                                                    span { class: "material-icons-round text-lg", "email" }
                                                }
                                                div {
                                                    p { class: "text-xs text-gray-400 font-medium", "Email" }
                                                    p { class: "text-sm font-semibold text-gray-800 dark:text-gray-200", "{candidate.email}" }
                                                }
                                            }
                                            if let Some(linkedin) = &candidate.linkedin_url {
                                                div {
                                                    class: "flex items-start gap-3",
                                                    div {
                                                        class: "p-2 bg-primary/10 rounded-lg text-primary",
                                                        span { class: "material-icons-round text-lg", "language" }
                                                    }
                                                    div {
                                                        p { class: "text-xs text-gray-400 font-medium", "LinkedIn" }
                                                        p { class: "text-sm font-semibold text-primary truncate w-64", "{linkedin}" }
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    // Skills
                                    if let Some(skills) = &candidate.skills {
                                        div {
                                            class: "bg-surface-light dark:bg-surface-dark rounded-xl p-5 shadow-sm border border-primary/5",
                                            h3 { class: "text-sm font-bold text-gray-900 dark:text-white uppercase tracking-wider mb-3 opacity-70", "Top Skills" }
                                            div {
                                                class: "flex flex-wrap gap-2",
                                                for skill in skills {
                                                    span { class: "px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-md text-sm font-medium", "{skill}" }
                                                }
                                            }
                                        }
                                    }
                                }
                            }},
                        Some(Err(err)) => rsx! {
                            div { class: "text-red-500 text-center p-4", "Error loading details: {err.message}" }
                        },
                        None => rsx! {
                            // Loading Skeleton
                            div {
                                class: "p-5 flex flex-col items-center",
                                Skeleton { width: "w-24", height: "h-24", class: "rounded-full mb-4" }
                                Skeleton { width: "w-48", height: "h-6", class: "mb-2" }
                                Skeleton { width: "w-32", height: "h-4", class: "mb-4" }
                                Skeleton { width: "w-full", height: "h-32", class: "rounded-xl mt-4" }
                            }
                        }
                    }
                }

                // Floating Action Button
                button {
                    class: "absolute bottom-6 right-6 w-14 h-14 bg-primary text-white rounded-full shadow-lg shadow-primary/40 flex items-center justify-center hover:bg-primary/90 hover:scale-105 active:scale-95 transition-all z-20",
                    span { class: "material-icons-round text-2xl", "add" }
                }
            }
        }
    }
}
