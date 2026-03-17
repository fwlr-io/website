brew install rg
alias grep="rg"

export RIPGREP_CONFIG_PATH="$HOME/.ripgreprc"
echo "
--engine
auto
--hyperlink-format
zed://file/{path}:{line}:{column}
" > $RIPGREP_CONFIG_PATH
