#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;
use crate::services::api::ApiService;
use crate::components::skeleton::Skeleton;
use crate::models::candidate::Candidate;
use crate::models::common::ApiError;

#[component]
pub fn Candidates() -> Element {
    let navigator = use_navigator();

    // Resource for fetching candidates
    let candidates_resource = use_resource(move || async move {
        let api = ApiService::new();
        api.get_candidates().await
    });

    rsx! {
        div {
            class: "bg-background-light dark:bg-background-dark text-slate-800 dark:text-slate-100 font-display antialiased min-h-screen relative flex justify-center bg-gray-50",

            // Mobile Container
            main {
                class: "w-full max-w-md mx-auto bg-white dark:bg-background-dark h-screen overflow-hidden relative shadow-2xl flex flex-col",

                // Status Bar Area (Mock)
                div {
                    class: "h-12 w-full flex items-end justify-between px-6 pb-2 text-xs font-semibold text-slate-900 dark:text-white bg-white/90 dark:bg-background-dark/90 backdrop-blur-md z-20 sticky top-0",
                    span { "9:41" }
                    div {
                        class: "flex gap-1.5 items-center",
                        span { class: "material-icons-round text-sm", "signal_cellular_alt" }
                        span { class: "material-icons-round text-sm", "wifi" }
                        span { class: "material-icons-round text-lg rotate-90", "battery_full" }
                    }
                }

                // Header Section
                header {
                    class: "px-6 pt-2 pb-4 bg-white dark:bg-background-dark z-10",
                    div {
                        class: "flex justify-between items-center mb-6",
                        h1 { class: "text-3xl font-extrabold text-slate-900 dark:text-white tracking-tight", "Candidates" }
                        button {
                            class: "p-2 rounded-full bg-sage-surface dark:bg-white/5 hover:bg-primary/10 transition-colors",
                            span { class: "material-icons-round text-primary text-2xl", "notifications_none" }
                        }
                    }

                    // Search and Filter
                    div {
                        class: "flex gap-3",
                        div {
                            class: "relative flex-1",
                            span { class: "material-icons-round absolute left-3 top-1/2 -translate-y-1/2 text-slate-400 text-xl", "search" }
                            input {
                                class: "w-full pl-10 pr-4 py-3 bg-slate-50 dark:bg-white/5 border border-sage-border rounded-xl focus:ring-2 focus:ring-primary focus:border-primary focus:outline-none placeholder-slate-400 text-sm transition-all shadow-sm",
                                placeholder: "Search name or role...",
                                r#type: "text"
                            }
                        }
                        button {
                            class: "bg-primary text-white p-3 rounded-xl shadow-soft hover:bg-primary/90 transition-colors flex items-center justify-center aspect-square",
                            span { class: "material-icons-round", "tune" }
                        }
                    }

                    // Quick Stats / Tabs
                    div {
                        class: "flex gap-2 mt-6 overflow-x-auto no-scrollbar pb-2",
                        button { class: "px-5 py-2 rounded-full bg-primary text-white text-sm font-semibold whitespace-nowrap shadow-md shadow-primary/20", "All" }
                        button { class: "px-5 py-2 rounded-full bg-white border border-sage-border text-slate-600 dark:text-slate-300 dark:bg-white/5 dark:border-white/10 text-sm font-medium whitespace-nowrap hover:bg-sage-surface transition-colors", "New" }
                        button { class: "px-5 py-2 rounded-full bg-white border border-sage-border text-slate-600 dark:text-slate-300 dark:bg-white/5 dark:border-white/10 text-sm font-medium whitespace-nowrap hover:bg-sage-surface transition-colors", "Interviewing" }
                        button { class: "px-5 py-2 rounded-full bg-white border border-sage-border text-slate-600 dark:text-slate-300 dark:bg-white/5 dark:border-white/10 text-sm font-medium whitespace-nowrap hover:bg-sage-surface transition-colors", "Offer" }
                    }
                }

                // Scrollable Content List
                div {
                    class: "flex-1 overflow-y-auto px-6 pb-24 space-y-4 no-scrollbar",

                    match &*candidates_resource.read() {
                        Some(Ok(candidates)) => {
                            let candidates = candidates.clone();
                            rsx! {
                                for candidate in candidates {
                                    div {
                                        class: "bg-white dark:bg-white/5 border border-sage-border dark:border-white/10 rounded-2xl p-4 shadow-sm hover:shadow-soft transition-shadow flex items-center gap-4 group cursor-pointer relative overflow-hidden",
                                        onclick: {
                                            let id = candidate.id.clone();
                                            move |_| { navigator.push(Route::CandidateDetails { id: id.clone() }); }
                                        },

                                    // Status Indicator Line (Color based on status logic could be added here)
                                    div { class: "absolute w-1 h-full left-0 top-0 bg-primary" }

                                    div {
                                        class: "relative",
                                        // Placeholder Avatar or from URL if available
                                        div {
                                            class: "w-14 h-14 rounded-xl bg-gray-100 dark:bg-gray-800 text-gray-500 flex items-center justify-center text-xl font-bold border border-gray-200 dark:border-gray-700",
                                            "{candidate.name.chars().next().unwrap_or('?')}"
                                        }
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        div {
                                            class: "flex justify-between items-start",
                                            h3 { class: "font-bold text-slate-900 dark:text-white text-lg truncate", "{candidate.name}" }
                                            span { class: "text-xs font-semibold text-primary bg-sage-surface dark:bg-primary/20 px-2 py-1 rounded-lg", "{candidate.status}" }
                                        }
                                        p { class: "text-sm text-slate-500 dark:text-slate-400 truncate mb-1", "{candidate.current_designation.clone().unwrap_or_default()}" }
                                        div {
                                            class: "flex items-center gap-2 text-xs text-slate-400 dark:text-slate-500",
                                            span { class: "material-icons-round text-[14px]", "schedule" }
                                            span { "{candidate.created_at.clone().unwrap_or_default()}" }
                                            span { "•" }
                                            span { "{candidate.current_company.clone().unwrap_or_default()}" }
                                        }
                                    }
                                }
                            }
                        }},
                        Some(Err(err)) => rsx! {
                            div { class: "text-red-500 text-center p-4", "Error loading candidates: {err.message}" }
                        },
                        None => rsx! {
                            // Skeleton Loaders
                            for _ in 0..5 {
                                div {
                                    class: "bg-white dark:bg-white/5 border border-sage-border dark:border-white/10 rounded-2xl p-4 shadow-sm flex items-center gap-4 relative overflow-hidden",
                                    div { class: "absolute w-1 h-full left-0 top-0 bg-gray-200 dark:bg-gray-700 animate-pulse" }
                                    div {
                                        class: "relative",
                                        Skeleton { width: "w-14", height: "h-14", class: "rounded-xl" }
                                    }
                                    div {
                                        class: "flex-1 min-w-0 space-y-2",
                                        div {
                                            class: "flex justify-between items-start",
                                            Skeleton { width: "w-32", height: "h-5", class: "rounded" }
                                            Skeleton { width: "w-20", height: "h-5", class: "rounded-lg" }
                                        }
                                        Skeleton { width: "w-24", height: "h-4", class: "rounded" }
                                        div {
                                            class: "flex items-center gap-2",
                                            Skeleton { width: "w-16", height: "h-3", class: "rounded" }
                                            Skeleton { width: "w-16", height: "h-3", class: "rounded" }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // Floating Action Button
                div {
                    class: "absolute bottom-24 right-6 z-30",
                    button {
                        class: "bg-primary hover:bg-primary/90 text-white w-14 h-14 rounded-full shadow-lg shadow-primary/30 flex items-center justify-center transition-transform hover:scale-105 active:scale-95",
                        span { class: "material-icons-round text-3xl", "add" }
                    }
                }

                // Bottom Navigation Bar
                nav {
                    class: "h-[88px] w-full bg-white dark:bg-background-dark border-t border-slate-100 dark:border-white/5 flex justify-around items-start pt-4 px-2 z-20 absolute bottom-0",
                    Link {
                        to: Route::Candidates {},
                        class: "flex flex-col items-center gap-1 group w-16",
                        span { class: "material-icons-round text-primary text-2xl group-hover:scale-110 transition-transform", "people" }
                        span { class: "text-[10px] font-bold text-primary", "Candidates" }
                    }
                    Link {
                        to: Route::Opportunities {},
                        class: "flex flex-col items-center gap-1 group w-16",
                        span { class: "material-icons-round text-slate-400 group-hover:text-primary text-2xl transition-colors", "work_outline" }
                        span { class: "text-[10px] font-medium text-slate-400 group-hover:text-primary transition-colors", "Jobs" }
                    }
                    Link {
                        to: Route::ResumeMatcher {},
                        class: "flex flex-col items-center gap-1 group w-16 relative",
                        div {
                            class: "relative",
                            span { class: "material-icons-round text-slate-400 group-hover:text-primary text-2xl transition-colors", "chat_bubble_outline" }
                            span { class: "absolute -top-1 -right-1 w-3 h-3 bg-red-500 rounded-full border-2 border-white dark:border-background-dark" }
                        }
                        span { class: "text-[10px] font-medium text-slate-400 group-hover:text-primary transition-colors", "Messages" }
                    }
                    Link {
                        to: Route::Menu {},
                        class: "flex flex-col items-center gap-1 group w-16",
                        span { class: "material-icons-round text-slate-400 group-hover:text-primary text-2xl transition-colors", "person_outline" }
                        span { class: "text-[10px] font-medium text-slate-400 group-hover:text-primary transition-colors", "Profile" }
                    }
                }

                // iOS Home Indicator
                div { class: "absolute bottom-1 left-1/2 -translate-x-1/2 w-32 h-1 bg-slate-900/20 dark:bg-white/20 rounded-full z-30" }
            }
        }
    }
}
