#![allow(non_snake_case)]

use dioxus::prelude::*;

mod components;
mod pages;

use pages::login::Login;
use pages::menu::Menu;
use pages::candidates::Candidates;
use pages::opportunities::Opportunities;
use pages::contacts::Contacts;
use pages::interactions::Interactions;
use pages::candidate_details::CandidateDetails;
use pages::resume_matcher::ResumeMatcher;
use pages::match_results::MatchResults;

fn main() {
    dioxus::launch(App);
}

fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Login {},
    #[route("/menu")]
    Menu {},
    #[route("/candidates")]
    Candidates {},
    #[route("/opportunities")]
    Opportunities {},
    #[route("/contacts")]
    Contacts {},
    #[route("/interactions")]
    Interactions {},
    #[route("/candidate/:id")]
    CandidateDetails { id: String },
    #[route("/resume-matcher")]
    ResumeMatcher {},
    #[route("/match-results")]
    MatchResults {},
}
