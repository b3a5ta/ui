#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Menu() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "bg-background-light dark:bg-background-dark font-display text-accent dark:text-gray-100 min-h-screen flex justify-center",

            // Mobile Container
            div {
                class: "w-full max-w-md bg-background-light dark:bg-background-dark min-h-screen relative flex flex-col shadow-2xl mx-auto overflow-hidden",

                // Header
                header {
                    class: "pt-12 pb-4 px-6 flex justify-between items-center bg-surface-light/80 dark:bg-surface-dark/80 backdrop-blur-md sticky top-0 z-10 border-b border-primary/10 dark:border-white/5",
                    h1 { class: "text-3xl font-extrabold text-accent dark:text-white tracking-tight", "Menu" }
                    button {
                        class: "p-2 rounded-full hover:bg-primary/10 transition-colors",
                        span { class: "material-icons-round text-primary text-2xl", "notifications_none" }
                    }
                }

                // Main Content
                main {
                    class: "flex-1 px-5 pt-6 pb-24 overflow-y-auto space-y-8",

                    // User Profile Summary Card
                    section {
                        class: "bg-surface-light dark:bg-surface-dark rounded-2xl p-5 shadow-soft border border-primary/5 dark:border-white/5 flex items-center gap-4 group cursor-pointer hover:border-primary/30 transition-all",
                        div {
                            class: "relative",
                            img {
                                alt: "Professional recruiter profile photo",
                                class: "w-16 h-16 rounded-full object-cover border-2 border-primary p-0.5",
                                src: "https://lh3.googleusercontent.com/aida-public/AB6AXuBe8PP5QrShToA6240gziMReWehDjtkX2uoyYQjrSu-2aKqv4f6Ljt-bB_qXFp2w2xkvjng_NYH7fv78mhXn4KiZaA4h0FZyurryAWmkBV_yR-bZZKvFGiOxAOUoybENj9Yj9DtgQjva1kMcSKiFUUkiVOpf7V5mV_rUstUQtkfcMPMgx5kD3rtiKQ3RicbNH3sNQ0D_hmFZSw7pCE5igqFlU-RmXLnUNtqZd6ZnVmQahIuepmfyEE78egUJCbePZMZ8XX0ElCHRQ-5"
                            }
                            div { class: "absolute bottom-0 right-0 w-4 h-4 bg-primary border-2 border-white dark:border-surface-dark rounded-full" }
                        }
                        div {
                            class: "flex-1 min-w-0",
                            h2 { class: "text-xl font-bold text-accent dark:text-white truncate", "Jane Doe" }
                            p { class: "text-sm text-gray-500 dark:text-gray-400 font-medium", "Senior Recruiter" }
                            div {
                                class: "flex items-center gap-1 mt-1",
                                span { class: "w-2 h-2 rounded-full bg-green-500 animate-pulse" }
                                span { class: "text-xs text-green-600 dark:text-green-400 font-semibold", "Active now" }
                            }
                        }
                        span { class: "material-icons-round text-gray-300 dark:text-gray-600 group-hover:text-primary transition-colors", "chevron_right" }
                    }

                    // Navigation List
                    nav {
                        class: "space-y-6",

                        // Workspace Section
                        h3 { class: "text-xs font-bold text-gray-400 uppercase tracking-wider px-2", "Workspace" }
                        div {
                            class: "bg-surface-light dark:bg-surface-dark rounded-2xl shadow-sm border border-primary/5 dark:border-white/5 overflow-hidden",

                            // Interactions Item
                            Link {
                                to: Route::Interactions {},
                                class: "flex items-center gap-4 p-4 hover:bg-primary/5 transition-colors active:bg-primary/10 border-b border-gray-100 dark:border-white/5 last:border-0 group",
                                div {
                                    class: "w-10 h-10 rounded-xl bg-primary/10 dark:bg-primary/20 flex items-center justify-center text-accent dark:text-primary-300 group-hover:bg-primary group-hover:text-white transition-all",
                                    span { class: "material-icons-round text-xl", "chat_bubble_outline" }
                                }
                                div {
                                    class: "flex-1",
                                    h4 { class: "font-semibold text-accent dark:text-gray-100", "Interactions" }
                                    p { class: "text-xs text-gray-500 dark:text-gray-400", "Manage candidate chats" }
                                }
                                span { class: "material-icons-round text-gray-300 dark:text-gray-600", "chevron_right" }
                            }

                            // Opportunities Item
                            Link {
                                to: Route::Opportunities {},
                                class: "flex items-center gap-4 p-4 hover:bg-primary/5 transition-colors active:bg-primary/10 border-b border-gray-100 dark:border-white/5 last:border-0 group",
                                div {
                                    class: "w-10 h-10 rounded-xl bg-primary/10 dark:bg-primary/20 flex items-center justify-center text-accent dark:text-primary-300 group-hover:bg-primary group-hover:text-white transition-all",
                                    span { class: "material-icons-round text-xl", "work_outline" }
                                }
                                div {
                                    class: "flex-1",
                                    h4 { class: "font-semibold text-accent dark:text-gray-100", "Opportunities" }
                                    p { class: "text-xs text-gray-500 dark:text-gray-400", "Job matching dashboard" }
                                }
                                span { class: "bg-primary text-white text-[10px] font-bold px-2 py-0.5 rounded-full mr-2", "3 New" }
                                span { class: "material-icons-round text-gray-300 dark:text-gray-600", "chevron_right" }
                            }
                        }

                        // Account Section
                        h3 { class: "text-xs font-bold text-gray-400 uppercase tracking-wider px-2 pt-2", "Account" }
                        div {
                            class: "bg-surface-light dark:bg-surface-dark rounded-2xl shadow-sm border border-primary/5 dark:border-white/5 overflow-hidden",

                            // Profile & Settings
                            a {
                                class: "flex items-center gap-4 p-4 hover:bg-primary/5 transition-colors active:bg-primary/10 border-b border-gray-100 dark:border-white/5 last:border-0 group",
                                href: "#",
                                div {
                                    class: "w-10 h-10 rounded-xl bg-primary/10 dark:bg-primary/20 flex items-center justify-center text-accent dark:text-primary-300 group-hover:bg-primary group-hover:text-white transition-all",
                                    span { class: "material-icons-round text-xl", "settings" }
                                }
                                div {
                                    class: "flex-1",
                                    h4 { class: "font-semibold text-accent dark:text-gray-100", "Profile & Settings" }
                                    p { class: "text-xs text-gray-500 dark:text-gray-400", "Preferences & security" }
                                }
                                span { class: "material-icons-round text-gray-300 dark:text-gray-600", "chevron_right" }
                            }

                            // Help & Support
                            a {
                                class: "flex items-center gap-4 p-4 hover:bg-primary/5 transition-colors active:bg-primary/10 border-b border-gray-100 dark:border-white/5 last:border-0 group",
                                href: "#",
                                div {
                                    class: "w-10 h-10 rounded-xl bg-primary/10 dark:bg-primary/20 flex items-center justify-center text-accent dark:text-primary-300 group-hover:bg-primary group-hover:text-white transition-all",
                                    span { class: "material-icons-round text-xl", "help_outline" }
                                }
                                div {
                                    class: "flex-1",
                                    h4 { class: "font-semibold text-accent dark:text-gray-100", "Help & Support" }
                                    p { class: "text-xs text-gray-500 dark:text-gray-400", "FAQs and contact" }
                                }
                                span { class: "material-icons-round text-gray-300 dark:text-gray-600", "chevron_right" }
                            }
                        }
                    }

                    // Logout Section
                    div {
                        class: "pt-4 pb-8",
                        button {
                            class: "w-full bg-surface-light dark:bg-surface-dark border border-red-100 dark:border-red-900/30 text-red-500 font-semibold py-4 rounded-xl flex items-center justify-center gap-2 hover:bg-red-50 dark:hover:bg-red-900/10 active:scale-[0.98] transition-all shadow-sm",
                            onclick: move |_| { navigator.push(Route::Login {}); },
                            span { class: "material-icons-round text-xl", "logout" }
                            "Log Out"
                        }
                        p { class: "text-center text-xs text-gray-400 mt-6 font-medium", "Version 1.2.0 (Build 405)" }
                    }
                }

                // Bottom Tab Bar
                nav {
                    class: "absolute bottom-0 w-full bg-surface-light dark:bg-surface-dark border-t border-gray-100 dark:border-white/5 px-6 py-3 flex justify-between items-center z-20 pb-8 safe-area-bottom",
                    Link {
                        to: Route::Candidates {}, // Home/Dashboard placeholder
                        class: "flex flex-col items-center gap-1 text-gray-400 hover:text-primary transition-colors",
                        span { class: "material-icons-round text-2xl", "dashboard" }
                        span { class: "text-[10px] font-medium", "Home" }
                    }
                    Link {
                        to: Route::Candidates {},
                        class: "flex flex-col items-center gap-1 text-gray-400 hover:text-primary transition-colors",
                        span { class: "material-icons-round text-2xl", "people" }
                        span { class: "text-[10px] font-medium", "Candidates" }
                    }
                    Link {
                        to: Route::ResumeMatcher {},
                        class: "flex flex-col items-center gap-1 text-gray-400 hover:text-primary transition-colors",
                        div {
                            class: "relative",
                            span { class: "material-icons-round text-2xl", "assignment" }
                        }
                        span { class: "text-[10px] font-medium", "Resumes" }
                    }
                    Link {
                        to: Route::Menu {},
                        class: "flex flex-col items-center gap-1 text-primary",
                        span { class: "material-icons-round text-2xl", "menu" }
                        span { class: "text-[10px] font-bold", "More" }
                    }
                }
            }

            // Background Decoration Gradient
            div {
                class: "fixed inset-0 -z-10 pointer-events-none opacity-40 dark:opacity-20",
                div { class: "absolute top-[-10%] right-[-10%] w-[500px] h-[500px] bg-primary/20 rounded-full blur-[100px]" }
                div { class: "absolute bottom-[-10%] left-[-10%] w-[400px] h-[400px] bg-primary/10 rounded-full blur-[80px]" }
            }
        }
    }
}
