use crate::shell::Shell;

pub fn generate_hook(shell: Shell) -> String {
    match shell {
        Shell::Bash => generate_bash_hook(),
        Shell::Zsh => generate_zsh_hook(),
        Shell::Fish => generate_fish_hook(),
    }
}

fn generate_bash_hook() -> String {
    r#"# putain hook for bash
# Add to ~/.bashrc: eval "$(putain --hook bash)"

__putain_last_command=""

__putain_preexec() {
    __putain_last_command="$1"
}

trap '__putain_preexec "$BASH_COMMAND"' DEBUG

__putain_precmd() {
    local exit_code=$?
    if [ $exit_code -ne 0 ] && [ -n "$__putain_last_command" ]; then
        export PUTAIN_CMD="$__putain_last_command"
        export PUTAIN_EXIT_CODE="$exit_code"
        export PUTAIN_OUTPUT="$(eval "$__putain_last_command" 2>&1)"
        putain --from-hook
        unset PUTAIN_CMD PUTAIN_OUTPUT PUTAIN_EXIT_CODE
    fi
    __putain_last_command=""
}

PROMPT_COMMAND="__putain_precmd;${PROMPT_COMMAND}"
"#.to_string()
}

fn generate_zsh_hook() -> String {
    r#"# putain hook for zsh
# Add to ~/.zshrc: eval "$(putain --hook zsh)"

__putain_last_command=""

__putain_preexec() {
    __putain_last_command="$1"
}

__putain_precmd() {
    local exit_code=$?
    if [[ $exit_code -ne 0 ]] && [[ -n "$__putain_last_command" ]]; then
        export PUTAIN_CMD="$__putain_last_command"
        export PUTAIN_EXIT_CODE="$exit_code"
        export PUTAIN_OUTPUT="$(eval "$__putain_last_command" 2>&1)"
        putain --from-hook
        unset PUTAIN_CMD PUTAIN_OUTPUT PUTAIN_EXIT_CODE
    fi
    __putain_last_command=""
}

autoload -Uz add-zsh-hook
add-zsh-hook preexec __putain_preexec
add-zsh-hook precmd __putain_precmd
"#.to_string()
}

fn generate_fish_hook() -> String {
    r#"# putain hook for fish
# Add to ~/.config/fish/config.fish: putain --hook fish | source

function __putain_on_postexec --on-event fish_postexec
    set -l exit_code $status
    set -l cmd $argv[1]
    if test $exit_code -ne 0
        set -x PUTAIN_CMD "$cmd"
        set -x PUTAIN_EXIT_CODE "$exit_code"
        set -x PUTAIN_OUTPUT (eval $cmd 2>&1)
        putain --from-hook
        set -e PUTAIN_CMD
        set -e PUTAIN_OUTPUT
        set -e PUTAIN_EXIT_CODE
    end
end
"#.to_string()
}

pub fn install_instructions(shell: &Shell) -> String {
    match shell {
        Shell::Bash => "Ajoute à ~/.bashrc :\n  eval \"$(putain --hook bash)\"".to_string(),
        Shell::Zsh => "Ajoute à ~/.zshrc :\n  eval \"$(putain --hook zsh)\"".to_string(),
        Shell::Fish => "Ajoute à ~/.config/fish/config.fish :\n  putain --hook fish | source".to_string(),
    }
}
