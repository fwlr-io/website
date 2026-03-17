export FZF_CTRL_R_COMMAND=''

export FZF_DEFAULT_COMMAND='fd
  --type f --follow --hidden
  --strip-cwd-prefix --exclude .git
'
export FZF_ALT_C_OPTS="
  --walker-skip .git,node_modules,target,dist
  --preview 'eza -TL 2 {} --color=always'
"
export FZF_CTRL_T_OPTS="
  --walker-skip .git,node_modules,target,dist
  --preview '[ -d {} ] &&
    eza -TL 2 {} --color=always ||
    bat {} --color=always'
"
export FZF_DEFAULT_OPTS="
  --no-info
  --list-border rounded
  --scrollbar='▐'
  --prompt='\$ '
  --preview-window 'noinfo'
"
source <(fzf --zsh)

_fzf_git_add() {
  git status -su |
    fzf --ansi --no-input \
      --bind "start:unbind(tab)+unbind(shift-tab)" \
      --bind 'a:execute-silent[
          git add $(echo {} | cut -c4- | cut -wf3)
        ]+reload[git status -su]' \
      --bind 'd:execute-silent[
          git restore -S $(echo {} | cut -c4- | cut -wf3)
        ]+reload[git status -su]' \
      --list-border 'none' \
      --preview-window 'right,75%,border-line' \
      --preview '[[ $(echo {} | cut -wf1) =~ "\?\?" ]] &&
        bat -f $(echo {} | cut -wf2) \
          --terminal-width $FZF_PREVIEW_COLUMNS ||
        git diff -- $(echo {} | cut -wf3) |
        delta -w $FZF_PREVIEW_COLUMNS'
  git status
}
