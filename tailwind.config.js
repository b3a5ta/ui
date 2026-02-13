/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: "class",
  content: [
    "./src/**/*.{rs,html,css}",
    "./index.html"
  ],
  theme: {
    extend: {
      colors: {
        "primary": "#609966",
        "primary-dark": "#40513B",
        "background-light": "#F7F9F0",
        "background-dark": "#171b17",
        "surface-light": "#FFFFFF",
        "surface-dark": "#232923",
        "sage-border": "rgba(96, 154, 102, 0.2)",
        "sage-surface": "rgba(96, 154, 102, 0.05)",
        "sage-50": "#eff5f0",
        "sage-100": "#dcead9",
        "sage-200": "#bcd8b9",
        "sage-300": "#94bf91",
        "sage-800": "#345237",
        "sage-900": "#2b432e",
      },
      fontFamily: {
        "display": ["Manrope", "sans-serif"]
      },
      borderRadius: {
        "DEFAULT": "0.25rem",
        "lg": "0.5rem",
        "xl": "0.75rem",
        "2xl": "1rem",
        "full": "9999px"
      },
      boxShadow: {
        'soft': '0 4px 20px -2px rgba(96, 154, 102, 0.1)',
        'nav': '0 -4px 20px -2px rgba(0, 0, 0, 0.05)',
      }
    },
  },
  plugins: [],
}
