bun patch tailwindcss
# edit preflight.css in the given dir
# (probably `node_modules/tailwindcss/preflight.css`)
# search for `until-found` and delete that selector
bun patch --commit "node_modules/tailwindcss"
