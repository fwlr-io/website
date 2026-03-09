# find your binary
which tailwindcss
# if it's a shim, extract the paths
# (regex matches `tailwindcss/` prefixed by any number of `directory/`s)
grep -Eo "(\w*/)+tailwindcss/" $(which tailwindcss)
# search for the template
find <paths> -name preflight.css -exec <editor> {} +

find \
  -f $(which tailwindcss) \
  -f $(grep -Eo "(\w*/)+tailwindcss/" $(which tailwindcss)) \
  -name preflight.css \
  -exec ${EDITOR:-open} {} +
