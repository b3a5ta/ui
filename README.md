# NuViO369 CRM Mobile App

Mobile application for NuViO369 CRM, built with Dioxus (Rust) and Tauri.

## Prerequisites

- Rust (cargo)
- Node.js (npm)
- Tauri CLI (optional, for mobile/desktop packaging)
- Dioxus CLI (`dx`): Install with `cargo install dioxus-cli`

## Setup

1.  **Install Dependencies:**
    ```bash
    npm install
    cargo install dioxus-cli
    ```

2.  **Run Application:**

    *   **Development (Tauri + Dioxus):**
        This command will start the Dioxus development server (dx serve) and the Tailwind CSS watcher concurrently, then launch the Tauri application.
        ```bash
        cargo tauri dev
        ```
        (Make sure `npm run dev` works first, which runs `dx serve`)

    *   **Web Only:**
        ```bash
        dx serve
        ```

## Project Structure

-   `src/`: Frontend Rust code (Dioxus components and pages).
-   `src-tauri/`: Tauri backend configuration.
-   `public/`: Static assets (compiled CSS).
-   `input.css`: Tailwind input file.
-   `tailwind.config.js`: Sage Green theme configuration.

## Features implemented

-   **Authentication:** Login Screen.
-   **Dashboard/Lists:** Candidates, Opportunities, Contacts.
-   **Details:** Candidate Details, Interactions History.
-   **Tools:** Resume Matcher (Upload, Match Results).
-   **Navigation:** Bottom Tab Bar, Side Menu (More).

## Theme

Uses the "Sage Green" theme:
-   Primary: `#609966`
-   Background: `#F7F9F0`
