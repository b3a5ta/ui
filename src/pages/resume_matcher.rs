#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn ResumeMatcher() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "bg-background-light dark:bg-background-dark font-display antialiased text-gray-800 dark:text-gray-100 min-h-screen flex justify-center bg-gray-50",

            // Mobile Device Container
            div {
                class: "w-full max-w-md h-screen bg-white dark:bg-neutral-900 shadow-2xl relative overflow-hidden flex flex-col",

                // Status Bar Placeholder (iOS Style)
                div {
                    class: "h-12 w-full bg-white dark:bg-neutral-900 flex items-end justify-between px-6 pb-2 z-20",
                    span { class: "text-xs font-semibold", "9:41" }
                    div {
                        class: "flex gap-1.5 items-center",
                        span { class: "material-icons text-[16px]", "signal_cellular_alt" }
                        span { class: "material-icons text-[16px]", "wifi" }
                        span { class: "material-icons text-[16px]", "battery_full" }
                    }
                }

                // Header
                header {
                    class: "px-6 py-4 flex items-center justify-between bg-white dark:bg-neutral-900 z-10",
                    h1 { class: "text-xl font-bold tracking-tight text-gray-900 dark:text-white", "Resume Matcher" }
                    button {
                        class: "relative p-2 rounded-full hover:bg-gray-100 dark:hover:bg-neutral-800 transition-colors",
                        img {
                            alt: "User Profile",
                            class: "w-8 h-8 rounded-full object-cover border border-gray-200 dark:border-neutral-700",
                            src: "https://lh3.googleusercontent.com/aida-public/AB6AXuChX_jpeiqDSA5_H_GWTdB8kUMOS29TqHPGCxWaXtsi-8NXHAgUChRmLGCCsW3Zhxshu4MD8Jyf_cLwH4dd3hYAIpjHb-yzx9tcHj96phzDpQGFkbvtdI47FWZ5tUfrY8v5wZCMWrTKw96C1odNVRrPJROM5ozEpV2Us9ZZtonB4NGNNGLUXJxS6kzZR422GOxI4uaKq33e0J3fUyyDN_MH8roTOmCUUOUYFogqijg5p174Z3Q10gA4MgToA2P3oY9h9oGFGcE28XZ9"
                        }
                        span { class: "absolute top-2 right-2 w-2.5 h-2.5 bg-red-500 rounded-full border-2 border-white dark:border-neutral-900" }
                    }
                }

                // Navigation Tabs
                div {
                    class: "px-6 pb-2 bg-white dark:bg-neutral-900",
                    div {
                        class: "flex p-1 bg-gray-100 dark:bg-neutral-800 rounded-lg",
                        button { class: "flex-1 py-2 px-3 bg-white dark:bg-neutral-700 text-primary dark:text-primary font-semibold text-sm rounded shadow-sm transition-all text-center", "Upload" }
                        button { class: "flex-1 py-2 px-3 text-gray-500 dark:text-gray-400 font-medium text-sm rounded hover:bg-gray-50 dark:hover:bg-neutral-700 transition-all text-center", "Job Desc" }
                        button {
                            class: "flex-1 py-2 px-3 text-gray-500 dark:text-gray-400 font-medium text-sm rounded hover:bg-gray-50 dark:hover:bg-neutral-700 transition-all text-center",
                            onclick: move |_| navigator.push(Route::MatchResults {}), // Link to Match Results
                            "Match"
                        }
                    }
                }

                // Main Content Scroll Area
                main {
                    class: "flex-1 overflow-y-auto no-scrollbar px-6 pt-6 pb-24",

                    // Upload Zone
                    div {
                        class: "w-full relative group cursor-pointer",
                        div { class: "absolute inset-0 bg-primary/5 rounded-xl transform transition-transform group-hover:scale-[1.02]" }
                        div {
                            class: "relative w-full aspect-[4/5] sm:aspect-square bg-white dark:bg-neutral-800 border-2 border-dashed border-primary/40 dark:border-primary/30 rounded-xl flex flex-col items-center justify-center p-8 text-center hover:border-primary transition-colors",
                            div {
                                class: "w-20 h-20 rounded-full bg-primary/10 flex items-center justify-center mb-6",
                                span { class: "material-icons text-primary text-4xl", "cloud_upload" }
                            }
                            h2 { class: "text-xl font-bold text-gray-900 dark:text-white mb-2", "Upload Resumes" }
                            p { class: "text-gray-500 dark:text-gray-400 text-sm mb-6 max-w-[200px]", "Tap to browse your files or import directly from cloud storage." }
                            div {
                                class: "flex gap-3 mb-8",
                                span { class: "px-3 py-1 bg-gray-100 dark:bg-neutral-700 text-xs font-medium text-gray-600 dark:text-gray-300 rounded", "PDF" }
                                span { class: "px-3 py-1 bg-gray-100 dark:bg-neutral-700 text-xs font-medium text-gray-600 dark:text-gray-300 rounded", "DOCX" }
                                span { class: "px-3 py-1 bg-gray-100 dark:bg-neutral-700 text-xs font-medium text-gray-600 dark:text-gray-300 rounded", "TXT" }
                            }
                            button {
                                class: "bg-primary text-white font-semibold py-3 px-8 rounded-lg shadow-lg shadow-primary/20 active:scale-95 transition-transform flex items-center gap-2",
                                span { class: "material-icons text-sm", "add" }
                                "Select Files"
                            }
                            p { class: "mt-4 text-xs text-gray-400 dark:text-neutral-500", "Max file size: 10MB" }
                        }
                    }

                    // Recent Activity Section
                    div {
                        class: "mt-8",
                        div {
                            class: "flex items-center justify-between mb-4",
                            h3 { class: "text-lg font-bold text-gray-900 dark:text-white", "Recent Uploads" }
                            button { class: "text-primary text-sm font-semibold", "View All" }
                        }
                        div {
                            class: "space-y-3",

                            // File Item 1
                            div {
                                class: "flex items-center p-3 bg-white dark:bg-neutral-800 rounded-lg shadow-sm border border-gray-100 dark:border-neutral-700",
                                div { class: "w-10 h-10 rounded bg-red-50 dark:bg-red-900/20 flex items-center justify-center mr-3 text-red-500 font-bold text-xs", "PDF" }
                                div {
                                    class: "flex-1 min-w-0",
                                    h4 { class: "text-sm font-semibold text-gray-900 dark:text-white truncate", "Sarah_Jenkins_CV.pdf" }
                                    p { class: "text-xs text-gray-500 dark:text-gray-400", "2.4 MB • Just now" }
                                }
                                div {
                                    class: "w-6 h-6 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center",
                                    span { class: "material-icons text-green-600 text-xs", "check" }
                                }
                            }

                            // File Item 2
                            div {
                                class: "flex items-center p-3 bg-white dark:bg-neutral-800 rounded-lg shadow-sm border border-gray-100 dark:border-neutral-700",
                                div { class: "w-10 h-10 rounded bg-blue-50 dark:bg-blue-900/20 flex items-center justify-center mr-3 text-blue-500 font-bold text-xs", "DOC" }
                                div {
                                    class: "flex-1 min-w-0",
                                    h4 { class: "text-sm font-semibold text-gray-900 dark:text-white truncate", "Michael_Chen_Resume_2023.docx" }
                                    p { class: "text-xs text-gray-500 dark:text-gray-400", "1.8 MB • 2 hrs ago" }
                                }
                                div {
                                    class: "w-6 h-6 rounded-full bg-green-100 dark:bg-green-900/30 flex items-center justify-center",
                                    span { class: "material-icons text-green-600 text-xs", "check" }
                                }
                            }
                        }
                    }
                }

                // Floating Action Button Area (Sticky Bottom)
                div {
                    class: "absolute bottom-[80px] left-0 w-full px-6 pointer-events-none",
                    // Bulk Upload Button
                    button {
                        class: "w-full bg-gray-900 dark:bg-white text-white dark:text-gray-900 font-bold py-4 rounded-xl shadow-xl flex items-center justify-center gap-3 pointer-events-auto active:scale-[0.98] transition-transform",
                        span { class: "material-icons", "layers" }
                        "Start Bulk Processing"
                    }
                }

                // Bottom Navigation Bar
                nav {
                    class: "h-20 bg-white dark:bg-neutral-900 border-t border-gray-100 dark:border-neutral-800 flex items-start justify-around pt-3 pb-6 z-20",
                    Link {
                        to: Route::Candidates {},
                        class: "flex flex-col items-center gap-1 group w-16",
                        span { class: "material-icons text-gray-400 dark:text-gray-500 group-hover:text-primary transition-colors", "home" }
                        span { class: "text-[10px] font-medium text-gray-400 dark:text-gray-500 group-hover:text-primary", "Home" }
                    }
                    Link {
                        to: Route::ResumeMatcher {},
                        class: "flex flex-col items-center gap-1 group w-16",
                        div {
                            class: "w-12 h-8 bg-primary/10 rounded-full flex items-center justify-center",
                            span { class: "material-icons text-primary", "person_search" }
                        }
                        span { class: "text-[10px] font-bold text-primary", "Matcher" }
                    }
                    Link {
                        to: Route::Opportunities {},
                        class: "flex flex-col items-center gap-1 group w-16",
                        span { class: "material-icons text-gray-400 dark:text-gray-500 group-hover:text-primary transition-colors", "work_outline" }
                        span { class: "text-[10px] font-medium text-gray-400 dark:text-gray-500 group-hover:text-primary", "Jobs" }
                    }
                    Link {
                        to: Route::Menu {},
                        class: "flex flex-col items-center gap-1 group w-16",
                        span { class: "material-icons text-gray-400 dark:text-gray-500 group-hover:text-primary transition-colors", "settings" }
                        span { class: "text-[10px] font-medium text-gray-400 dark:text-gray-500 group-hover:text-primary", "Settings" }
                    }
                }
            }
        }
    }
}
