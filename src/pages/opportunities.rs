#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Opportunities() -> Element {
    let navigator = use_navigator();

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
                            onclick: move |_| navigator.go_back(),
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

                    // Filter Pills
                    div {
                        class: "flex gap-2 pb-2 overflow-x-auto scrollbar-hide -mx-6 px-6",
                        button { class: "px-4 py-1.5 rounded-full bg-sage-800 text-white text-sm font-semibold shadow-sm whitespace-nowrap", "All Jobs" }
                        button { class: "px-4 py-1.5 rounded-full bg-white border border-gray-200 text-slate-600 text-sm font-medium shadow-sm whitespace-nowrap", "Active" }
                        button { class: "px-4 py-1.5 rounded-full bg-white border border-gray-200 text-slate-600 text-sm font-medium shadow-sm whitespace-nowrap", "Drafts" }
                        button { class: "px-4 py-1.5 rounded-full bg-white border border-gray-200 text-slate-600 text-sm font-medium shadow-sm whitespace-nowrap", "Archived" }
                    }

                    // Stats Overview
                    div {
                        class: "flex justify-between items-center py-2 text-sm text-slate-500 font-medium",
                        span { "12 Active Opportunities" }
                        span {
                            class: "text-primary flex items-center gap-1",
                            "Sort by: Date "
                            span { class: "material-icons text-base", "expand_more" }
                        }
                    }

                    // Opportunity Card 1 (Active)
                    div {
                        class: "bg-white rounded-xl p-5 shadow-sm border border-gray-100 relative group active:scale-[0.99] transition-transform duration-100",
                        div {
                            class: "flex justify-between items-start mb-3",
                            div {
                                class: "flex items-center gap-3",
                                div {
                                    class: "w-10 h-10 rounded-lg bg-gray-50 flex items-center justify-center border border-gray-100 overflow-hidden",
                                    img { class: "w-full h-full object-cover", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuB9tYtn82Hn0LrV6xcbasvLXXIteW80rpAnq3sMl-R1TzJzgvzxkJXbTxPblgUBkZMLdbke3GmlXB0XEzulZvOcnlrCmoxXrHCI-avk9QmShs-eO3bNiIFMC3sb15xgHfxb6vprknIxj6vKuxkok6aXAJpPV40GFPn1EEeIWp4slSdhjhzR3AjD-ObOU-yPY3PZGzoy981THj1sbMOdh8sB3z0tPKd7fby4-QGZDD7Fl9ERXyIsRz5htuifrjIg_CnliyI_Yvp5Gz1q" }
                                }
                                div {
                                    h3 { class: "text-lg font-bold text-slate-900 leading-tight", "Senior UX Designer" }
                                    p { class: "text-xs text-slate-500 font-medium mt-0.5", "TechCorp Inc." }
                                }
                            }
                            span {
                                class: "inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-bold bg-green-50 text-green-700 border border-green-100",
                                "Active"
                            }
                        }
                        div {
                            class: "flex items-center gap-4 mt-4 border-t border-gray-50 pt-3",
                            div {
                                class: "flex -space-x-2 overflow-hidden",
                                img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDsXignmaPv6k-3Z9plL8l9p_xr6B2sCsBXK3mEiPwuuHEv0zem_5i0KbBMw87ntpgwRrjqE2ibPwDQsg_xuLYiHjLzuqQ7LKxrifoNbl7my9G3L4Zaou9Keg-EFPv1Y8fdHLcgM01OJGCAff7bqmSGAXzz3WDGumD1zBfmTU-HmkxtLekoNKpp9G3UmHMFST0xbE8wykWOJnqIZZXxG4eFyLk-NoZZRQIvpIiQm-Iy4CyznAis_cWky5Wf9GLD9QgWyyJZ_Q7-UGo3" }
                                img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAVAIvqeL1359BBZDvPk5MHmxXCnfZhY0dxwhH60UZLZ5Ceta2wFzaIRDqX6aWJbEOvlepmwSRw2tenQfc2HD4Jfvm3vItWrA6qffpzx8QdM3xTGo-vuec-PczqDONjpMwCJdDoTCoLJ9-TVmE2sWIRg92zrcbbG_RE2WLcNyLvzTjBmC8TDKZXz8h0zV78pB95N_E9R7S4zE-ar0ptPDwj3u_tB2VL_Hu9egA-HzpiZeYd4yMWut6kW-pSQIiabNeavLf9539-2tWy" }
                                img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDo0U3cCMgE_ICCaFGWwaOSQvir9M7zHsP0ioNZTKuEhBndZQiPGTO74pSknX4YBvjnJaU_yF3Kwg4x3W3Sb6suhhHS4CQ4szYFleCvjXRx5Dlx-geN9Yrk9Dt8mpgsQq9gXBH_abYM3FBnq966yFK1jY2AAnpNSo-mdI9NL9FTCpw9gon_iGFbgO_GkRdwjb4B5UVlqMJkMmO2IBfIiDtaShVlEhGH7wPY_fM0xyD_RVWcF2aCrSjEDMsHS9F10QOjUxqSM5jlz9g1" }
                            }
                            span { class: "text-xs font-semibold text-slate-600", "+9 candidates" }
                            span { class: "mx-auto" }
                            span { class: "text-xs text-slate-400", "Posted 2d ago" }
                        }
                    }

                    // Opportunity Card 2 (Interviewing)
                    div {
                        class: "bg-white rounded-xl p-5 shadow-sm border border-gray-100 relative group active:scale-[0.99] transition-transform duration-100",
                        div {
                            class: "flex justify-between items-start mb-3",
                            div {
                                class: "flex items-center gap-3",
                                div {
                                    class: "w-10 h-10 rounded-lg bg-indigo-50 flex items-center justify-center border border-indigo-100 overflow-hidden",
                                    img { class: "w-full h-full object-cover", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuBAOY_qKjSEpRpHnprteT86wnKX5IMYu5-rwEaubRpYCKTGCwIBLPrEfQ3qxP9MY5kwVR6MGBEdFU9V0XH4bT2yHCF4Ri5eLEUYumhVpxEwfqCWl_fx9ur0lwDw-zE1lqxQwTEOy0Fjj5s4dwdvAAjjpTTExfQzv0l4ADZWj57-2kJ66sWwrOIp29NoryhDPmgeI5qedf9_dmcy5uF0nPASVloZZZ3xL48e8QmGFFPOvL7oMZeyDNYWDFXh6mmxeulqH0NRpXIyVMrO" }
                                }
                                div {
                                    h3 { class: "text-lg font-bold text-slate-900 leading-tight", "Product Manager" }
                                    p { class: "text-xs text-slate-500 font-medium mt-0.5", "Innovate LLC" }
                                }
                            }
                            span {
                                class: "inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-bold bg-primary/10 text-primary border border-primary/20",
                                "Interviewing"
                            }
                        }
                        div {
                            class: "flex items-center gap-4 mt-4 border-t border-gray-50 pt-3",
                            div {
                                class: "flex -space-x-2 overflow-hidden",
                                img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAcMshSJMnoqN9RRNDK8wiqQpwXCbd8OC8-4bZIk7zpki-NUgffd9p6h_bDFlxorJoV3a-Mw8XT8Cdh9ly6iXqNNO-mTwGHovRTzPZnY9rpmZl22O6RX8fcShoVhYGNujJ_BzY_O59ISRJajsc4HJZqez8ZWtOCbDV_DyejlX7mwHlOvynj5hO38pehfrVGwaE5JHCk21QJ8YaOYCsiZxQtBzNJYSwgeNh-VZ2I_O0ig932YxGfJ2bGPvQuZUbyClKdCgTv75deha1g" }
                            }
                            span { class: "text-xs font-semibold text-slate-600", "4 candidates" }
                            span { class: "mx-auto" }
                            span { class: "text-xs text-slate-400", "Updated 5h ago" }
                        }
                    }

                    // Opportunity Card 3 (Draft)
                    div {
                        class: "bg-white rounded-xl p-5 shadow-sm border border-gray-100 relative group active:scale-[0.99] transition-transform duration-100 opacity-80",
                        div {
                            class: "flex justify-between items-start mb-3",
                            div {
                                class: "flex items-center gap-3",
                                div {
                                    class: "w-10 h-10 rounded-lg bg-orange-50 flex items-center justify-center border border-orange-100 overflow-hidden",
                                    img { class: "w-full h-full object-cover", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAmz--Jy5reFvCz2Ew3MMXYelj4JgeFWSKfZ36MDSeg-7-VXNl40llrdlAh1ipH_ujCgqIK8rSIeNziTl9p4oLCvMM0MLYk-MJLIX0PRICMT_vhcrtTqnf4RjaVtyEAtQknux9Ux1m9v_b1FChKcTao11Gc0iywUtSrxv1mZSsM8jU0lLo86DQR4FwTCnm2sqBtvSuRKZ7GIMCICqPjgpYDbjUQB-O58igDj1R-tInv03EGnIO23avSgGsmF6VCEfe_qAb6VGwB4koj" }
                                }
                                div {
                                    h3 { class: "text-lg font-bold text-slate-900 leading-tight", "Backend Engineer" }
                                    p { class: "text-xs text-slate-500 font-medium mt-0.5", "Server Solutions" }
                                }
                            }
                            span {
                                class: "inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-bold bg-gray-100 text-slate-600 border border-gray-200",
                                "Draft"
                            }
                        }
                        div {
                            class: "flex items-center gap-4 mt-4 border-t border-gray-50 pt-3",
                            span { class: "text-xs font-medium text-slate-400 italic", "No candidates yet" }
                            span { class: "mx-auto" }
                            span { class: "text-xs text-slate-400", "Created 1w ago" }
                        }
                    }

                    // Opportunity Card 4 (On Hold)
                    div {
                        class: "bg-white rounded-xl p-5 shadow-sm border border-gray-100 relative group active:scale-[0.99] transition-transform duration-100",
                        div {
                            class: "flex justify-between items-start mb-3",
                            div {
                                class: "flex items-center gap-3",
                                div {
                                    class: "w-10 h-10 rounded-lg bg-blue-50 flex items-center justify-center border border-blue-100 overflow-hidden",
                                    img { class: "w-full h-full object-cover", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuCWC2x3L_ALnlNgcsN7Ce0nZfP9Of2LYcXiD56iIWZZYNNT-VHTPbYvLcZO2jgXX7iSLd680zySPtE2wobGDxwDxTi6ikzhRwHWAbdMRdHKpEPL6OtuTpIbEifeu6xTj8yR4u9qPFx0Ak5D_cR6YAM8Pea0gwBfJ6itBbqVJBjxISfg0ApYybWGEfoWAL8UgyPj4hJIDqnljK5CYTtpfseuoCx51mYlblTs6RGl2WCmmr1GVG48i4ZQfGD0rK8C2E2dI5-zerOaRjZx" }
                                }
                                div {
                                    h3 { class: "text-lg font-bold text-slate-900 leading-tight", "Lead Brand Designer" }
                                    p { class: "text-xs text-slate-500 font-medium mt-0.5", "Global Design" }
                                }
                            }
                            span {
                                class: "inline-flex items-center px-2.5 py-1 rounded-lg text-xs font-bold bg-amber-50 text-amber-700 border border-amber-100",
                                "On Hold"
                            }
                        }
                        div {
                            class: "flex items-center gap-4 mt-4 border-t border-gray-50 pt-3",
                            div {
                                class: "flex -space-x-2 overflow-hidden",
                                img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuDwzun0d5pD8zhg20RmiI8ysmM0mhwzbjAGjsqcKjXzJLvZ49YdHKMYzmbNfsDrubZ0Csmwp8L4IBwLldepNOz4XkIruiE9mS_X9hDDwHgepe7DLnnVO6IH7QzzQxoudurudTvmW3ZVc9W30pMdVLdsHnt6FWqiOBncOyEBDQU1Z_hdISxGl7t5UdwXy-RKvoPHSyzhY8xcwXoMaIe0XVLjy9dU3nj1wmrBnNJ9_3jvId6HR9FUltApIR3XQjnDVzyd0tg8wkr8lyR4" }
                                img { class: "inline-block h-6 w-6 rounded-full ring-2 ring-white", src: "https://lh3.googleusercontent.com/aida-public/AB6AXuAJLQPuey2D4j22QBuX-o6pQvIkIU_PSxUHyYThd91nZAxyK3i-9R8rLO6u_K9opGRcamySAHJxayUwtctoWeUkIm59UUT4uDQ57MFG-igdObpYMIZglFxczOtW-9npYCISKDizXU0QjqKCxTEhgydINpp_FTQG7TLO3azYq0f-4T-XbkkNVvbCYuEg2Nun72Xa29BN-1Yox1RSSRkgqfQOT89onSynRYSUwNwgNZRHyLuVWgQYeT7L4E83g852vcV-qoq5PCmGMbc-" }
                            }
                            span { class: "text-xs font-semibold text-slate-600", "2 candidates" }
                            span { class: "mx-auto" }
                            span { class: "text-xs text-slate-400", "Paused 3d ago" }
                        }
                    }

                    // Spacer for FAB
                    div { class: "h-20" }
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
