#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::Route;

#[component]
pub fn Login() -> Element {
    let navigator = use_navigator();

    rsx! {
        div {
            class: "bg-background-light dark:bg-background-dark font-display text-slate-800 dark:text-slate-100 min-h-screen flex items-center justify-center p-4 relative overflow-hidden",

            // Decorative Background Elements
            div {
                class: "absolute top-0 left-0 w-full h-full overflow-hidden -z-10 pointer-events-none",
                div { class: "absolute -top-[10%] -left-[10%] w-[50%] h-[30%] bg-primary/10 rounded-full blur-3xl" }
                div { class: "absolute top-[20%] -right-[5%] w-[40%] h-[25%] bg-primary/10 rounded-full blur-3xl" }
                div { class: "absolute bottom-[5%] left-[20%] w-[60%] h-[20%] bg-primary/5 rounded-full blur-3xl" }
            }

            // Main Container
            main {
                class: "w-full max-w-sm mx-auto relative z-10",

                // Header
                div {
                    class: "mb-8 text-center",
                    div {
                        class: "inline-flex items-center justify-center w-16 h-16 rounded-2xl bg-gradient-to-br from-primary to-primary-dark shadow-lg shadow-primary/30 mb-6",
                        span { class: "material-icons text-white text-3xl", "work_outline" }
                    }
                    h1 { class: "text-3xl font-bold text-slate-900 dark:text-white mb-2", "Welcome Back" }
                    p { class: "text-slate-500 dark:text-slate-400 text-sm", "Log in to manage your candidates." }
                }

                // Login Card
                div {
                    class: "bg-surface-light dark:bg-surface-dark rounded-2xl shadow-soft p-6 sm:p-8 border border-primary/10 dark:border-primary/20 backdrop-blur-sm",
                    form {
                        class: "space-y-5",
                        onsubmit: move |evt| {
                            evt.prevent_default();
                            navigator.push(Route::Candidates {});
                        },

                        // Email Field
                        div {
                            class: "space-y-1.5",
                            label { class: "block text-sm font-medium text-slate-700 dark:text-slate-300", r#for: "email", "Email Address" }
                            div {
                                class: "relative group",
                                div {
                                    class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                                    span { class: "material-icons text-slate-400 group-focus-within:text-primary text-xl transition-colors duration-200", "mail_outline" }
                                }
                                input {
                                    class: "block w-full pl-10 pr-3 py-3 border-slate-200 dark:border-slate-700 rounded-lg text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all duration-200 bg-slate-50 dark:bg-slate-800/50 sm:text-sm",
                                    id: "email",
                                    name: "email",
                                    placeholder: "recruiter@company.com",
                                    required: true,
                                    r#type: "email"
                                }
                            }
                        }

                        // Password Field
                        div {
                            class: "space-y-1.5",
                            label { class: "block text-sm font-medium text-slate-700 dark:text-slate-300", r#for: "password", "Password" }
                            div {
                                class: "relative group",
                                div {
                                    class: "absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none",
                                    span { class: "material-icons text-slate-400 group-focus-within:text-primary text-xl transition-colors duration-200", "lock_outline" }
                                }
                                input {
                                    class: "block w-full pl-10 pr-10 py-3 border-slate-200 dark:border-slate-700 rounded-lg text-slate-900 dark:text-white placeholder-slate-400 focus:outline-none focus:ring-2 focus:ring-primary/20 focus:border-primary transition-all duration-200 bg-slate-50 dark:bg-slate-800/50 sm:text-sm",
                                    id: "password",
                                    name: "password",
                                    placeholder: "Enter your password",
                                    required: true,
                                    r#type: "password"
                                }
                                div {
                                    class: "absolute inset-y-0 right-0 pr-3 flex items-center cursor-pointer",
                                    span { class: "material-icons text-slate-400 hover:text-slate-600 dark:hover:text-slate-200 text-xl transition-colors duration-200", "visibility_off" }
                                }
                            }
                        }

                        // Forgot Password Link
                        div {
                            class: "flex items-center justify-end",
                            a { class: "text-sm font-medium text-primary hover:text-primary-dark transition-colors duration-200", href: "#", "Forgot Password?" }
                        }

                        // Submit Button
                        button {
                            class: "w-full flex justify-center py-3.5 px-4 border border-transparent rounded-lg shadow-md shadow-primary/20 text-sm font-semibold text-white bg-primary hover:bg-primary-dark focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary transition-all duration-200 transform active:scale-[0.98]",
                            r#type: "submit",
                            "Log In"
                        }
                    }
                }

                // Footer
                div {
                    class: "mt-8 text-center",
                    p {
                        class: "text-sm text-slate-500 dark:text-slate-400",
                        "New here? "
                        a {
                            class: "font-semibold text-primary hover:text-primary-dark transition-colors duration-200 ml-1 inline-flex items-center",
                            href: "#",
                            "Create an Account"
                            span { class: "material-icons text-base ml-0.5", "arrow_forward" }
                        }
                    }
                }
            }
        }
    }
}
