#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn MatchResults() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "bg-background-light dark:bg-background-dark font-display text-text-primary-light dark:text-gray-100 min-h-screen relative pb-24",

            // Header Section
            header {
                class: "sticky top-0 z-30 bg-surface-light/90 dark:bg-surface-dark/90 backdrop-blur-md border-b border-primary/10 px-4 pt-12 pb-4 shadow-sm",
                div {
                    class: "flex items-center justify-between mb-4",
                    button {
                        class: "p-2 -ml-2 rounded-full hover:bg-primary/10 active:bg-primary/20 transition-colors text-primary",
                        onclick: move |_| navigator.go_back(),
                        span { class: "material-icons-round text-3xl", "chevron_left" }
                    }
                    h1 { class: "text-xl font-bold text-center flex-1 pr-8", "Match Results" }
                    button {
                        class: "p-2 rounded-full hover:bg-primary/10 text-primary",
                        span { class: "material-icons-round", "more_horiz" }
                    }
                }

                // JD Context Card
                div {
                    class: "bg-primary/5 dark:bg-primary/10 rounded-xl p-4 border border-primary/10 flex items-start gap-3",
                    div {
                        class: "bg-white dark:bg-surface-dark p-2 rounded-lg shadow-sm text-primary shrink-0",
                        span { class: "material-icons-round", "work_outline" }
                    }
                    div {
                        p { class: "text-xs font-semibold text-primary uppercase tracking-wider mb-0.5", "Matching Against" }
                        h2 { class: "text-sm font-bold text-gray-900 dark:text-white leading-tight", "Product Manager (Senior)" }
                        p { class: "text-xs text-gray-500 dark:text-gray-400 mt-1", "JD Ref: #PM-2023-04 • San Francisco, CA" }
                    }
                }
            }

            // Filters
            div {
                class: "px-4 py-4 overflow-x-auto hide-scrollbar flex gap-3 sticky top-[158px] z-20 bg-background-light dark:bg-background-dark/95 backdrop-blur-sm",
                button {
                    class: "flex items-center gap-1.5 px-4 py-2 bg-primary text-white rounded-full text-sm font-semibold shadow-soft whitespace-nowrap",
                    span { class: "material-icons-round text-sm", "filter_list" }
                    "All Matches"
                }
                button { class: "px-4 py-2 bg-white dark:bg-surface-dark border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 rounded-full text-sm font-medium whitespace-nowrap hover:border-primary/50 hover:text-primary transition-colors", "Score > 90%" }
                button { class: "px-4 py-2 bg-white dark:bg-surface-dark border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 rounded-full text-sm font-medium whitespace-nowrap hover:border-primary/50 hover:text-primary transition-colors", "Exp: 5+ Years" }
                button { class: "px-4 py-2 bg-white dark:bg-surface-dark border border-gray-200 dark:border-gray-700 text-gray-600 dark:text-gray-300 rounded-full text-sm font-medium whitespace-nowrap hover:border-primary/50 hover:text-primary transition-colors", "Remote Only" }
            }

            // Stats Summary
            div {
                class: "px-4 mb-2 flex justify-between items-end",
                p { class: "text-sm font-semibold text-gray-500 dark:text-gray-400", "14 Candidates Found" }
                button {
                    class: "text-xs font-bold text-primary flex items-center gap-1",
                    "Sort by: Relevance"
                    span { class: "material-icons-round text-sm", "keyboard_arrow_down" }
                }
            }

            // Candidate List
            main {
                class: "px-4 space-y-4",

                // Card 1: Top Match
                div {
                    class: "bg-surface-light dark:bg-surface-dark rounded-xl p-4 shadow-card border-l-4 border-primary relative overflow-hidden group active:scale-[0.98] transition-transform duration-200",
                    onclick: move |_| navigator.push(Route::CandidateDetails { id: "1".to_string() }),

                    // Selection Checkbox
                    div {
                        class: "absolute top-4 right-4 z-10",
                        div {
                            class: "h-6 w-6 rounded border-2 border-primary bg-primary flex items-center justify-center text-white",
                            span { class: "material-icons-round text-sm", "check" }
                        }
                    }
                    div {
                        class: "flex gap-4 mb-3",
                        div {
                            class: "relative shrink-0",
                            img {
                                alt: "Smiling woman with red hair",
                                class: "w-14 h-14 rounded-full object-cover ring-2 ring-primary/20",
                                src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDSftfTfjX0RFGFzbr2ExWzFbn0u9az7eLEJuTSFNPlkRNhR2mV6WBZGQZFnCwFInXykYej8RgADDgC29r3E1n1hzOoGyoHXh-ZqVjV2wnWdg9_VKA_dCII4EAXyVQr-d68Ps9Efv7SgGC7UfxejUgP55n_VfIW5LB800x3RX75dJCGRFd8UkI2iJIhjY1gKqxYZhG1IKtMN1iFNGqFz6ZUkmh-P-LZRTV6cGmiRtDY_C8tY45-H7vY_xRma-bzMMB9ZB_OEV_UIijN"
                            }
                            div {
                                class: "absolute -bottom-1 -right-1 bg-white dark:bg-surface-dark rounded-full p-0.5",
                                span { class: "material-icons-round text-primary text-base bg-primary/10 rounded-full p-0.5", "star" }
                            }
                        }
                        div {
                            class: "flex-1 min-w-0 pr-8",
                            div {
                                class: "flex items-center gap-2 mb-0.5",
                                h3 { class: "text-lg font-bold text-gray-900 dark:text-white truncate", "Sarah Jenning" }
                            }
                            p { class: "text-sm text-gray-500 dark:text-gray-400 truncate", "Senior Product Manager at TechFlow" }
                            p {
                                class: "text-xs text-gray-400 mt-1 flex items-center gap-1",
                                span { class: "material-icons-round text-[14px]", "location_on" }
                                " San Francisco, CA"
                            }
                        }
                    }
                    // Match Score Section
                    div {
                        class: "bg-background-light dark:bg-background-dark rounded-lg p-3 mb-3",
                        div {
                            class: "flex justify-between items-end mb-2",
                            span { class: "text-xs font-semibold uppercase text-gray-500 dark:text-gray-400 tracking-wider", "Match Score" }
                            span { class: "text-xl font-bold text-primary", "98%" }
                        }
                        div {
                            class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5 overflow-hidden",
                            div { class: "bg-primary h-2.5 rounded-full", style: "width: 98%" }
                        }
                        p {
                            class: "text-xs text-primary/80 mt-2 font-medium flex items-center gap-1",
                            span { class: "material-icons-round text-sm", "check_circle" }
                            " Matches 14/15 key requirements"
                        }
                    }
                    // Tags
                    div {
                        class: "flex flex-wrap gap-2",
                        span { class: "px-2.5 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-md", "Product Strategy" }
                        span { class: "px-2.5 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-md", "Agile" }
                        span { class: "px-2.5 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-md", "JIRA" }
                        span { class: "px-2.5 py-1 bg-gray-100 dark:bg-gray-700 text-gray-500 dark:text-gray-400 text-xs font-semibold rounded-md", "+4 more" }
                    }
                }

                // Card 2
                div {
                    class: "bg-surface-light dark:bg-surface-dark rounded-xl p-4 shadow-card border border-transparent hover:border-primary/20 relative active:scale-[0.98] transition-transform duration-200",
                    div {
                        class: "absolute top-4 right-4 z-10",
                        div { class: "h-6 w-6 rounded border-2 border-gray-300 dark:border-gray-600" }
                    }
                    div {
                        class: "flex gap-4 mb-3",
                        div {
                            class: "relative shrink-0",
                            img {
                                alt: "Man with beard looking sideways",
                                class: "w-14 h-14 rounded-full object-cover ring-2 ring-gray-100 dark:ring-gray-700",
                                src: "https://lh3.googleusercontent.com/aida-public/AB6AXuCjr04MR_ct_1iCmSkILwB58ssdp28T7BEfijrQQKkoLO6a5ZyJzMKFRdOVhTIIm3unEAKbjbailDP3m0YRD3PL5qswdwdXl4xzeWRi0TrJqSPoMcD2VEPkKkwDsOVAoC5A2Jeh20W5mLpS8KXx6uVecDvB1beNWzoiJ3JyQoKVW4MADa7ukU9lNkGS7FUzHLm_YlfMSQxgPVrTVp12aHC9uHjN2k7SYdTmS9rboaWf8EkikvKa-iBSlyP8Run_w4mZ5W0SqgGq0-YL"
                            }
                        }
                        div {
                            class: "flex-1 min-w-0 pr-8",
                            h3 { class: "text-lg font-bold text-gray-900 dark:text-white truncate", "Michael Chen" }
                            p { class: "text-sm text-gray-500 dark:text-gray-400 truncate", "Product Lead at StartUp Inc." }
                            p {
                                class: "text-xs text-gray-400 mt-1 flex items-center gap-1",
                                span { class: "material-icons-round text-[14px]", "location_on" }
                                " New York, NY"
                            }
                        }
                    }
                    div {
                        class: "bg-background-light dark:bg-background-dark rounded-lg p-3 mb-3",
                        div {
                            class: "flex justify-between items-end mb-2",
                            span { class: "text-xs font-semibold uppercase text-gray-500 dark:text-gray-400 tracking-wider", "Match Score" }
                            span { class: "text-lg font-bold text-primary", "92%" }
                        }
                        div {
                            class: "w-full bg-gray-200 dark:bg-gray-700 rounded-full h-2.5 overflow-hidden",
                            div { class: "bg-primary/90 h-2.5 rounded-full", style: "width: 92%" }
                        }
                    }
                    div {
                        class: "flex flex-wrap gap-2",
                        span { class: "px-2.5 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-md", "Roadmapping" }
                        span { class: "px-2.5 py-1 bg-primary/10 text-primary text-xs font-semibold rounded-md", "UX Research" }
                    }
                }
            }

            // Floating Action Area
            div {
                class: "fixed bottom-0 left-0 right-0 p-4 bg-gradient-to-t from-background-light via-background-light to-transparent dark:from-background-dark dark:via-background-dark pt-12 z-40",
                div {
                    class: "max-w-md mx-auto",
                    button {
                        class: "w-full bg-primary hover:bg-primary-dark text-white font-bold py-4 px-6 rounded-xl shadow-lg shadow-primary/30 flex items-center justify-center gap-3 transition-all active:scale-[0.98]",
                        span { class: "material-icons-round", "phone_in_talk" }
                        span { "Start Bulk Call (1 Selected)" }
                    }
                }
            }
        }
    }
}
