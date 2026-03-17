export EZA_COLORS="gm=1;31" # 'git modified' color = bold;red
alias ls="eza"
alias ll="eza --long --all \
  --header --git --no-user \
  --git-ignore -I .git \
  --tree --level 2"
alias lll="eza --long --all \
  --header --git --hyperlink --icons \
  --git -I .git \
  --tree --level 4"
