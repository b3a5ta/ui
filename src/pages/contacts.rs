#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Contacts() -> Element {
    let navigator = use_navigator();

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

                    // Quick Actions / Top Contacts
                    section {
                        class: "mb-6",
                        h3 { class: "text-xs font-semibold uppercase tracking-wider text-text-muted mb-3 ml-1", "Recent" }
                        div {
                            class: "flex space-x-4 overflow-x-auto pb-4 hide-scrollbar -mx-6 px-6",
                            // Recent Item 1
                            div {
                                class: "flex flex-col items-center space-y-2 min-w-[72px]",
                                div {
                                    class: "w-16 h-16 rounded-full bg-gradient-to-br from-primary to-primary-dark p-0.5 shadow-soft",
                                    div {
                                        class: "w-full h-full rounded-full border-2 border-white dark:border-surface-dark overflow-hidden relative",
                                        img {
                                            alt: "Sarah Connor",
                                            class: "w-full h-full object-cover",
                                            src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAKhPEJ8e0KG0NtG1y5YwNV2Kmh0gLAIsEa7Mi3YjtDecfOfEBp1ug4F-RoF25uJiPl_T14WVFxi4d0umelp3g6XWdHwgFmUbSGC5t8VilWQ4tuF4iKGa8mMrUFu2qei01e_O7xim_YeRHElvr4ihRx4HZ4J-1n37CiES88STdbonq7-26E-U2tNbdrLAMHJgkMmJDIAsZ-60K2h8BpMe4izfAKiGSG04HsFjxxV-Pn-Dn5C-nFcR1mrhjrEAOvXZ0wMw9CF-OXQiZV"
                                        }
                                    }
                                }
                                span { class: "text-xs font-medium text-center truncate w-full", "Sarah" }
                            }
                            // Recent Item 2
                            div {
                                class: "flex flex-col items-center space-y-2 min-w-[72px]",
                                div {
                                    class: "w-16 h-16 rounded-full bg-gradient-to-br from-primary to-primary-dark p-0.5 shadow-soft",
                                    div {
                                        class: "w-full h-full rounded-full border-2 border-white dark:border-surface-dark overflow-hidden flex items-center justify-center bg-primary/10",
                                        span { class: "text-primary font-bold text-lg", "MK" }
                                    }
                                }
                                span { class: "text-xs font-medium text-center truncate w-full", "Mike" }
                            }
                            // Recent Item 3
                            div {
                                class: "flex flex-col items-center space-y-2 min-w-[72px]",
                                div {
                                    class: "w-16 h-16 rounded-full bg-gradient-to-br from-primary to-primary-dark p-0.5 shadow-soft",
                                    div {
                                        class: "w-full h-full rounded-full border-2 border-white dark:border-surface-dark overflow-hidden relative",
                                        img {
                                            alt: "Julian Reed",
                                            class: "w-full h-full object-cover",
                                            src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDnxiKJolMUzdsNjBi2rIlXtZqSqeiC6Ie8kiBHOffzJCuFeqd-l7-VuS6vE_Ll5-woBxlEn9cCJmBfys6bhPH4WEL3Wstdz-n1eu964gtOoIIxIWcrAMEBWRDwGidEOv3ZWKd0bouBdiUVl49URrhtS1i9DgarXp5ObRun5NfzSikMMuAOlnq3G6Dj4mDYL3Fe63riE9JLum6hhY4xPnmF6-QjNzgmat2d6wkBzUo_8hzKc7-WP_VK9BIwmX74Z8QRWL_bzDuQjw87"
                                        }
                                    }
                                }
                                span { class: "text-xs font-medium text-center truncate w-full", "Julian" }
                            }
                        }
                    }

                    // Contacts List
                    div {
                        class: "space-y-6",

                        // Section A
                        div {
                            class: "relative",
                            div {
                                class: "sticky top-0 bg-background-light/95 dark:bg-background-dark/95 backdrop-blur-sm py-2 z-10 border-b border-gray-100 dark:border-gray-800 mb-2",
                                span { class: "text-primary font-bold text-sm", "A" }
                            }
                            div {
                                class: "bg-white dark:bg-surface-dark rounded-xl shadow-sm border border-gray-100 dark:border-gray-800 divide-y divide-gray-50 dark:divide-gray-800",
                                // Contact Item
                                div {
                                    class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                    div {
                                        class: "h-12 w-12 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold text-lg shrink-0 mr-4 group-hover:bg-primary group-hover:text-white transition-colors duration-300",
                                        "AB"
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "Alice Brown" }
                                        p { class: "text-xs font-medium text-text-muted truncate", "Senior Recruiter" }
                                        p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "TechCorp" }
                                    }
                                    button {
                                        class: "text-gray-300 hover:text-primary dark:text-gray-600 dark:hover:text-primary transition-colors",
                                        span { class: "material-icons text-xl", "chevron_right" }
                                    }
                                }
                                // Contact Item
                                div {
                                    class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                    div {
                                        class: "h-12 w-12 rounded-full bg-blue-50 dark:bg-blue-900/20 flex items-center justify-center text-blue-600 dark:text-blue-400 font-bold text-lg shrink-0 mr-4",
                                        "AD"
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "Adam Davis" }
                                        p { class: "text-xs font-medium text-text-muted truncate", "Hiring Manager" }
                                        p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "Innovate Inc" }
                                    }
                                    button {
                                        class: "text-gray-300 hover:text-primary dark:text-gray-600 dark:hover:text-primary transition-colors",
                                        span { class: "material-icons text-xl", "chevron_right" }
                                    }
                                }
                            }
                        }

                        // Section B
                        div {
                            class: "relative",
                            div {
                                class: "sticky top-0 bg-background-light/95 dark:bg-background-dark/95 backdrop-blur-sm py-2 z-10 border-b border-gray-100 dark:border-gray-800 mb-2",
                                span { class: "text-primary font-bold text-sm", "B" }
                            }
                            div {
                                class: "bg-white dark:bg-surface-dark rounded-xl shadow-sm border border-gray-100 dark:border-gray-800 divide-y divide-gray-50 dark:divide-gray-800",
                                // Contact Item
                                div {
                                    class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                    div {
                                        class: "h-12 w-12 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold text-lg shrink-0 mr-4 group-hover:bg-primary group-hover:text-white transition-colors duration-300",
                                        "BL"
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "Brian Lee" }
                                        p { class: "text-xs font-medium text-text-muted truncate", "VP of Engineering" }
                                        p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "SoftSystems" }
                                    }
                                    div {
                                        class: "flex items-center space-x-2",
                                        span { class: "material-icons text-lg text-primary/40", "work_history" }
                                    }
                                }
                                // Contact Item
                                div {
                                    class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                    div {
                                        class: "relative h-12 w-12 shrink-0 mr-4",
                                        img {
                                            alt: "Bella Swan",
                                            class: "h-full w-full rounded-full object-cover",
                                            src: "https://lh3.googleusercontent.com/aida-public/AB6AXuBh21a959rAmHkWGAUZeNi79Lnw1U1dS9L-RkhnbrtkuO5wdC9F9lIATA1ar8zj9KnC2YBOFwc1EvLSbyi9hI2A8ZKnR91otVBRwkgnGOXOiJZDeHmakqg3UpmMTPjT5qDZTU2C-0GxSYZSRUatH1mbhz6evZ7YMj1YOYw02SwaZT3BD4ACsz4Onypd6DvZKA8ryssnWQMe3k4WSToMviSBqKTFOlsY7JYCerz9zARfnusHiRMm1NDyUHqKaYwncPZvtDwwOw1M5Qmb"
                                        }
                                        span { class: "absolute bottom-0 right-0 block h-3 w-3 rounded-full bg-green-500 ring-2 ring-white dark:ring-surface-dark" }
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "Bella Swan" }
                                        p { class: "text-xs font-medium text-text-muted truncate", "Talent Acquisition" }
                                        p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "Global Tech" }
                                    }
                                    button {
                                        class: "text-gray-300 hover:text-primary dark:text-gray-600 dark:hover:text-primary transition-colors",
                                        span { class: "material-icons text-xl", "chevron_right" }
                                    }
                                }
                            }
                        }

                        // Section C
                        div {
                            class: "relative",
                            div {
                                class: "sticky top-0 bg-background-light/95 dark:bg-background-dark/95 backdrop-blur-sm py-2 z-10 border-b border-gray-100 dark:border-gray-800 mb-2",
                                span { class: "text-primary font-bold text-sm", "C" }
                            }
                            div {
                                class: "bg-white dark:bg-surface-dark rounded-xl shadow-sm border border-gray-100 dark:border-gray-800 divide-y divide-gray-50 dark:divide-gray-800",
                                // Contact Item
                                div {
                                    class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                    div {
                                        class: "h-12 w-12 rounded-full bg-purple-50 dark:bg-purple-900/20 flex items-center justify-center text-purple-600 dark:text-purple-400 font-bold text-lg shrink-0 mr-4",
                                        "CC"
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "Charlie Carter" }
                                        p { class: "text-xs font-medium text-text-muted truncate", "Head of Product" }
                                        p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "Creative Labs" }
                                    }
                                    button {
                                        class: "text-gray-300 hover:text-primary dark:text-gray-600 dark:hover:text-primary transition-colors",
                                        span { class: "material-icons text-xl", "chevron_right" }
                                    }
                                }
                            }
                        }

                        // Section D
                        div {
                            class: "relative",
                            div {
                                class: "sticky top-0 bg-background-light/95 dark:bg-background-dark/95 backdrop-blur-sm py-2 z-10 border-b border-gray-100 dark:border-gray-800 mb-2",
                                span { class: "text-primary font-bold text-sm", "D" }
                            }
                            div {
                                class: "bg-white dark:bg-surface-dark rounded-xl shadow-sm border border-gray-100 dark:border-gray-800 divide-y divide-gray-50 dark:divide-gray-800",
                                // Contact Item
                                div {
                                    class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                    div {
                                        class: "h-12 w-12 rounded-full bg-primary/10 flex items-center justify-center text-primary font-bold text-lg shrink-0 mr-4 group-hover:bg-primary group-hover:text-white transition-colors duration-300",
                                        "DA"
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "David Anderson" }
                                        p { class: "text-xs font-medium text-text-muted truncate", "Technical Lead" }
                                        p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "Streamline" }
                                    }
                                    button {
                                        class: "text-gray-300 hover:text-primary dark:text-gray-600 dark:hover:text-primary transition-colors",
                                        span { class: "material-icons text-xl", "chevron_right" }
                                    }
                                }
                                // Contact Item
                                div {
                                    class: "flex items-center p-4 active:bg-gray-50 dark:active:bg-gray-800 transition-colors cursor-pointer group",
                                    div {
                                        class: "h-12 w-12 rounded-full bg-orange-50 dark:bg-orange-900/20 flex items-center justify-center text-orange-600 dark:text-orange-400 font-bold text-lg shrink-0 mr-4",
                                        "DP"
                                    }
                                    div {
                                        class: "flex-1 min-w-0",
                                        h4 { class: "text-sm font-bold text-text-main dark:text-white truncate", "Diana Prince" }
                                        p { class: "text-xs font-medium text-text-muted truncate", "Operations Manager" }
                                        p { class: "text-[10px] text-primary/80 uppercase tracking-wide mt-0.5 truncate", "Logistics Co." }
                                    }
                                    button {
                                        class: "text-gray-300 hover:text-primary dark:text-gray-600 dark:hover:text-primary transition-colors",
                                        span { class: "material-icons text-xl", "chevron_right" }
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
