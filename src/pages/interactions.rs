#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Interactions() -> Element {
    let navigator = use_navigator();

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
                            onclick: move |_| navigator.go_back(),
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

                    // Date Group: Today
                    div {
                        class: "relative z-10 mb-8",
                        h2 { class: "ml-14 text-xs font-bold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4", "Today" }

                        // Interaction Item 1 (Call)
                        div {
                            class: "flex mb-6 group",
                            div {
                                class: "flex-shrink-0 mr-4 relative",
                                div {
                                    class: "w-12 h-12 rounded-full bg-primary/10 flex items-center justify-center border-2 border-background-light dark:border-background-dark z-10 relative",
                                    span { class: "material-icons-round text-primary text-xl", "phone_in_talk" }
                                }
                            }
                            div {
                                class: "flex-1 bg-white dark:bg-[#232923] p-4 rounded-2xl shadow-card border border-gray-100 dark:border-gray-800 transition-all active:scale-[0.98]",
                                div {
                                    class: "flex justify-between items-start mb-1",
                                    h3 { class: "font-bold text-text-main dark:text-white text-sm", "Initial Screening" }
                                    span { class: "text-xs font-medium text-gray-400", "10:30 AM" }
                                }
                                p {
                                    class: "text-xs text-text-main/80 dark:text-gray-300 mb-2 leading-relaxed",
                                    "Candidate discussed salary expectations and current notice period. Seemed enthusiastic about the hybrid model."
                                }
                                div {
                                    class: "flex items-center gap-2 mt-3",
                                    span { class: "px-2 py-1 rounded bg-orange-100 dark:bg-orange-900/30 text-orange-600 dark:text-orange-300 text-[10px] font-bold uppercase tracking-wide", "High Priority" }
                                    div {
                                        class: "flex -space-x-1 ml-auto",
                                        img {
                                            alt: "Recruiter portrait",
                                            class: "w-5 h-5 rounded-full border border-white dark:border-gray-800",
                                            src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDE3m0fuGWkL8uN7CgKtA7fh9djc7rVJdzUHqu4oIL9yYld1goQjHIX6UZSOsftmXr4PZvc6tah8-uSioR2QOH8HZWZt8Uj1jb9W6WPy1ih90IDdxzIgf0uzESmULhaaQFNpEBvn-Pj0fL8WgX47h9CiiNzP4B68pwj3sF4pV2DJyDmG5Pug78qfqm_SJiEM8tLA7lulYhE-c1EFVW-i1wfrZAajuTZ0QIK-Wf8fCgOxrSuI3YhACUpeTnTYSIn-Zsg9FjRajBlk2-V"
                                        }
                                    }
                                }
                            }
                        }

                        // Interaction Item 2 (Note)
                        div {
                            class: "flex mb-6 group",
                            div {
                                class: "flex-shrink-0 mr-4 relative",
                                div {
                                    class: "w-12 h-12 rounded-full bg-gray-100 dark:bg-gray-800 flex items-center justify-center border-2 border-background-light dark:border-background-dark z-10 relative",
                                    span { class: "material-icons-round text-gray-500 text-xl", "edit_note" }
                                }
                            }
                            div {
                                class: "flex-1 bg-white dark:bg-[#232923] p-4 rounded-2xl shadow-card border border-gray-100 dark:border-gray-800 transition-all active:scale-[0.98]",
                                div {
                                    class: "flex justify-between items-start mb-1",
                                    h3 { class: "font-bold text-text-main dark:text-white text-sm", "Note Added" }
                                    span { class: "text-xs font-medium text-gray-400", "9:15 AM" }
                                }
                                p {
                                    class: "text-xs text-text-main/80 dark:text-gray-300 leading-relaxed italic",
                                    "\"Check references from previous role at TechCorp.\""
                                }
                            }
                        }
                    }

                    // Date Group: Yesterday
                    div {
                        class: "relative z-10 mb-8",
                        h2 { class: "ml-14 text-xs font-bold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4", "Yesterday" }

                        // Interaction Item 3 (Email)
                        div {
                            class: "flex mb-6 group",
                            div {
                                class: "flex-shrink-0 mr-4 relative",
                                div {
                                    class: "w-12 h-12 rounded-full bg-blue-50 dark:bg-blue-900/20 flex items-center justify-center border-2 border-background-light dark:border-background-dark z-10 relative",
                                    span { class: "material-icons-round text-blue-500 text-xl", "mail" }
                                }
                            }
                            div {
                                class: "flex-1 bg-white dark:bg-[#232923] p-4 rounded-2xl shadow-card border border-gray-100 dark:border-gray-800 transition-all active:scale-[0.98]",
                                div {
                                    class: "flex justify-between items-start mb-1",
                                    h3 { class: "font-bold text-text-main dark:text-white text-sm", "Resume Received" }
                                    span { class: "text-xs font-medium text-gray-400", "4:15 PM" }
                                }
                                p {
                                    class: "text-xs text-text-main/80 dark:text-gray-300 mb-3 leading-relaxed",
                                    "Attached is the updated portfolio and requested case study solutions."
                                }
                                div {
                                    class: "flex items-center gap-2 p-2 bg-background-light dark:bg-background-dark rounded-lg border border-gray-100 dark:border-gray-700",
                                    span { class: "material-icons-round text-primary text-sm", "attach_file" }
                                    span { class: "text-[10px] font-medium text-text-main dark:text-gray-300", "Portfolio_Final_v2.pdf" }
                                }
                            }
                        }
                    }

                    // Date Group: Oct 24
                    div {
                        class: "relative z-10 mb-8",
                        h2 { class: "ml-14 text-xs font-bold uppercase tracking-wider text-gray-400 dark:text-gray-500 mb-4", "Oct 24" }

                        // Interaction Item 4 (Meeting)
                        div {
                            class: "flex mb-6 group",
                            div {
                                class: "flex-shrink-0 mr-4 relative",
                                div {
                                    class: "w-12 h-12 rounded-full bg-purple-50 dark:bg-purple-900/20 flex items-center justify-center border-2 border-background-light dark:border-background-dark z-10 relative",
                                    span { class: "material-icons-round text-purple-500 text-xl", "videocam" }
                                }
                            }
                            div {
                                class: "flex-1 bg-white dark:bg-[#232923] p-4 rounded-2xl shadow-card border border-gray-100 dark:border-gray-800 transition-all active:scale-[0.98]",
                                div {
                                    class: "flex justify-between items-start mb-1",
                                    h3 { class: "font-bold text-text-main dark:text-white text-sm", "Technical Interview" }
                                    span { class: "text-xs font-medium text-gray-400", "2:00 PM" }
                                }
                                p {
                                    class: "text-xs text-text-main/80 dark:text-gray-300 mb-2 leading-relaxed",
                                    "Zoom link sent. Interviewer: Sarah Jenkins. Duration: 45 mins."
                                }
                                div {
                                    class: "flex -space-x-2 overflow-hidden py-1",
                                    img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-background-dark", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuC4JlENGoMrtvcJR5bjP0TJbmUR8UCvxshxAU6SDBmcQil1RTShk8pyy6RSsVtXCH1LiZ3Xhgh_-1Zbe7ckmpsO4NDnlptOTbV6fMdcCcXsuUnf9fRlGkobpXZA8KqdfZ4DrV2HWgPZOARXBrDNSlQYi5L5PhCyJJTe-9j1a-ajrXcj3lnH8PZA1pMrjPmZASyAU1Q78_DFe_6OyqyHIfdUTGBgttglwTnkxtrhuv1sXs2efdy7lkYMOhUulsgdpAl_HRsojxXvu_i3" }
                                    img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white dark:ring-background-dark", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuClWrRAFbUZcESuprph4ObovbL2k4i8CjKfv32I0ZChay0Fa1euDAYeGyNxRGYuIwZPjdKaCA7DLntdZ0IHOmLL9VrI-hQs6t2DHD4UFMQiHkAQniItkZprT-5l1j9Yczi0ZbIJcH1j7pE09ekNdEG5XdPSc8J9541JWDCbEmMVXs4UXfY3bShZ01Metevm_FF65DJJ0X0K-JqFIwPZMNGYEI8SO7RgeUzRLxHEvJE7qpLuYkAFVyoea7d0zNaUA3XEq6UElaICHPjX" }
                                }
                            }
                        }

                        // Interaction Item 5 (Outbound Call)
                        div {
                            class: "flex mb-6 group",
                            div {
                                class: "flex-shrink-0 mr-4 relative",
                                div {
                                    class: "w-12 h-12 rounded-full bg-red-50 dark:bg-red-900/20 flex items-center justify-center border-2 border-background-light dark:border-background-dark z-10 relative",
                                    span { class: "material-icons-round text-red-400 text-xl", "phone_missed" }
                                }
                            }
                            div {
                                class: "flex-1 bg-white dark:bg-[#232923] p-4 rounded-2xl shadow-card border border-gray-100 dark:border-gray-800 transition-all active:scale-[0.98] opacity-75",
                                div {
                                    class: "flex justify-between items-start mb-1",
                                    h3 { class: "font-bold text-text-main dark:text-white text-sm", "Unanswered Call" }
                                    span { class: "text-xs font-medium text-gray-400", "11:00 AM" }
                                }
                                p {
                                    class: "text-xs text-text-main/80 dark:text-gray-300 leading-relaxed",
                                    "Left voicemail regarding availability for next week."
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
