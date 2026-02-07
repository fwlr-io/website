### note on setting up tailwindcss intellisense in Zed

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
