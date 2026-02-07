### setting up tailwindcss intellisense in Zed

Zed's `settings.json`
```json
{
  "languages": {
    "Rust": {
      "language_servers": ["...", "tailwindcss-language-server"],
    }
  },
  "lsp": {
    "tailwindcss-language-server": {
      "settings": {
        "includeLanguages": {
          "Rust": "html",
          "rust": "html",
          "*.rs": "html",
        },
        "experimental": {
          // add your custom themes to hover previews
          "configFile": "input.css",
          "classRegex": [
            // class=("foo", move || ...)
            // class=(["foo", "bar"], move || ...)
            ["class=\\(([^)]*)", "(?:'|\"|`)([^\"'`]*)(?:'|\"|`)"],
            // class:foo=move || ...
            // class:"foo"=move || ..
            ["class:\"?([^=\"]*)\"?="],
          ],
        },
      },
    }
  }
```
<!--
### setting up tailwind?
```js
/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    files: ["*.html", "input.css", "./src/**/*.rs"],
  },
  theme: {
    extend: {},
  },
  plugins: [],
};
```-->
