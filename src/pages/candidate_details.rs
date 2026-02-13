#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn CandidateDetails(id: String) -> Element {
    let navigator = use_navigator();

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

                    // Candidate Profile Card
                    div {
                        class: "bg-surface-light dark:bg-surface-dark rounded-xl shadow-sm border border-primary/10 p-5 flex flex-col items-center text-center relative overflow-hidden",
                        // Decorative background blob
                        div { class: "absolute top-0 left-0 w-full h-20 bg-gradient-to-b from-primary/10 to-transparent" }
                        div {
                            class: "relative mt-2",
                            img {
                                alt: "Portrait of Sarah Jenkins smiling in professional attire",
                                class: "w-24 h-24 rounded-full object-cover border-4 border-surface-light dark:border-surface-dark shadow-md",
                                src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDYLniJN5R1ymXTMr1Awg6cwU_o0WfyRE5KDAOgJ0ldLrLn1VQVPlWxmmA3XUXFOoTeRQIwdWfUGkjsj9QdbFfKdzqgzFZjO0NzqGQtqiMqGzk5nbvRrowzw7vXkhnkkNioyuONBIXlWDRMOsbd45ie46B0sptF_NESWUNryO93mRB364VsyPTY4Jjl3t4o-7zjSb8oKJ0Ibqe_SYuT9hKZ71vWUvn-m7AG9unyhGlDw-OA8cVjs2faKnbz_0SdLCw87wF4cH7iany3"
                            }
                            span { class: "absolute bottom-1 right-1 w-5 h-5 bg-green-500 border-2 border-surface-light dark:border-surface-dark rounded-full" }
                        }
                        h2 { class: "mt-3 text-xl font-bold text-gray-900 dark:text-white", "Sarah Jenkins" }
                        p { class: "text-sm text-gray-500 dark:text-gray-400 font-medium", "Senior UX Designer" }
                        div {
                            class: "mt-3 flex items-center gap-2",
                            span {
                                class: "px-3 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-full border border-primary/20",
                                "Interviewing"
                            }
                            span {
                                class: "px-3 py-1 bg-gray-100 dark:bg-gray-800 text-gray-500 text-xs font-medium rounded-full",
                                "Remote"
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
                            onclick: move |_| { navigator.push(Route::Interactions {}); }, // Link to Interactions
                            span { class: "material-icons-round text-2xl", "mail_outline" }
                        }
                    }

                    // Tabs
                    div {
                        class: "bg-surface-light dark:bg-surface-dark p-1 rounded-lg border border-primary/10 flex",
                        button { class: "flex-1 py-2 text-sm font-semibold rounded-md bg-primary text-white shadow-sm transition-all", "Info" }
                        button {
                            class: "flex-1 py-2 text-sm font-medium text-gray-500 dark:text-gray-400 hover:text-primary transition-colors",
                            onclick: move |_| { navigator.push(Route::Interactions {}); }, // Link to History
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
                                        p { class: "text-sm font-semibold text-gray-800 dark:text-gray-200", "+1 (555) 012-3456" }
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
                                        p { class: "text-sm font-semibold text-gray-800 dark:text-gray-200", "sarah.j@example.com" }
                                    }
                                }
                                div {
                                    class: "flex items-start gap-3",
                                    div {
                                        class: "p-2 bg-primary/10 rounded-lg text-primary",
                                        span { class: "material-icons-round text-lg", "language" }
                                    }
                                    div {
                                        p { class: "text-xs text-gray-400 font-medium", "LinkedIn" }
                                        p { class: "text-sm font-semibold text-primary", "linkedin.com/in/sarahjenkins" }
                                    }
                                }
                            }
                        }

                        // Skills
                        div {
                            class: "bg-surface-light dark:bg-surface-dark rounded-xl p-5 shadow-sm border border-primary/5",
                            h3 { class: "text-sm font-bold text-gray-900 dark:text-white uppercase tracking-wider mb-3 opacity-70", "Top Skills" }
                            div {
                                class: "flex flex-wrap gap-2",
                                span { class: "px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-md text-sm font-medium", "Figma" }
                                span { class: "px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-md text-sm font-medium", "User Research" }
                                span { class: "px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-md text-sm font-medium", "Prototyping" }
                                span { class: "px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-md text-sm font-medium", "Design Systems" }
                                span { class: "px-3 py-1.5 bg-gray-100 dark:bg-gray-800 text-gray-700 dark:text-gray-300 rounded-md text-sm font-medium", "HTML/CSS" }
                            }
                        }

                        // Documents
                        div {
                            class: "bg-surface-light dark:bg-surface-dark rounded-xl p-5 shadow-sm border border-primary/5",
                            div {
                                class: "flex items-center justify-between mb-4",
                                h3 { class: "text-sm font-bold text-gray-900 dark:text-white uppercase tracking-wider opacity-70", "Documents" }
                                button { class: "text-primary text-xs font-bold hover:underline", "View All" }
                            }
                            div {
                                class: "flex items-center justify-between p-3 border border-gray-100 dark:border-gray-800 rounded-lg hover:bg-gray-50 dark:hover:bg-gray-800/50 transition-colors cursor-pointer group",
                                div {
                                    class: "flex items-center gap-3",
                                    div {
                                        class: "w-10 h-10 bg-red-50 text-red-500 rounded-lg flex items-center justify-center",
                                        span { class: "material-icons-round", "picture_as_pdf" }
                                    }
                                    div {
                                        p { class: "text-sm font-semibold text-gray-900 dark:text-white group-hover:text-primary transition-colors", "Resume_Sarah_v2.pdf" }
                                        p { class: "text-xs text-gray-400", "Added 2 days ago • 1.2 MB" }
                                    }
                                }
                                span { class: "material-icons-round text-gray-300 group-hover:text-primary", "download" }
                            }
                        }

                        // Recent Activity Preview
                        div {
                            class: "bg-surface-light dark:bg-surface-dark rounded-xl p-5 shadow-sm border border-primary/5",
                            div {
                                class: "flex items-center justify-between mb-4",
                                h3 { class: "text-sm font-bold text-gray-900 dark:text-white uppercase tracking-wider opacity-70", "Recent Activity" }
                            }
                            div {
                                class: "relative pl-4 border-l-2 border-primary/20 space-y-6",
                                div {
                                    class: "relative",
                                    div { class: "absolute -left-[21px] top-1 h-3 w-3 rounded-full bg-primary border-2 border-white dark:border-surface-dark" }
                                    p { class: "text-sm font-semibold text-gray-800 dark:text-gray-200", "Phone Screening Completed" }
                                    p { class: "text-xs text-gray-500 mt-1", "Yesterday by Mike Ross" }
                                }
                                div {
                                    class: "relative",
                                    div { class: "absolute -left-[21px] top-1 h-3 w-3 rounded-full bg-gray-300 border-2 border-white dark:border-surface-dark" }
                                    p { class: "text-sm font-semibold text-gray-800 dark:text-gray-200", "Application Received" }
                                    p { class: "text-xs text-gray-500 mt-1", "2 days ago via LinkedIn" }
                                }
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
