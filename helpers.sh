#! /bin/sh

DATA_DIR="$TX_ROOT/data"
DIR_PATHS_FILE="$DATA_DIR/paths.txt"

LAYOUT_EXT=".layout.sh"
FRAGMENT_EXT=".fragment.sh"

run_tmux() {
    tmux "$@" 2>/dev/null
}

tmux_attach_or_switch() {
    if [ -z "$TMUX" ]; then
        run_tmux attach-session -t "$1"
    else
        run_tmux switch-client -t "$1"
    fi
}

get_tmux_sessions() {
    run_tmux ls -F "#{session_name}"
}

is_session() {
    get_tmux_sessions | grep -qx "$1"
}

get_layouts() {
    find "$DATA_DIR" -type f -name "*$LAYOUT_EXT" -exec basename {} "$LAYOUT_EXT" \;
}

is_layout() {
    find "$DATA_DIR" -type f -name "$1$LAYOUT_EXT" > /dev/null
}

get_fragments() {
    find "$DATA_DIR" -type f -name "*$FRAGMENT_EXT" -exec basename {} "$FRAGMENT_EXT" \;
}

is_fragment() {
    find "$DATA_DIR" -type f -name "$1$FRAGMENT_EXT" > /dev/null
}

read_dir_paths_file() {
    if [ -d "$DIR_PATHS_FILE" ]; then
        # echo "Error: $DIR_PATHS_FILE is a directory"
        exit 1
    fi

    if [ ! -f "$DIR_PATHS_FILE" ]; then
        touch "$DIR_PATHS_FILE"
    fi

    local dir_paths=$(cat "$DIR_PATHS_FILE")

    if [ -z "$dir_paths" ]; then
        # echo "Error: no paths in $DIR_PATHS_FILE\nEdit with 'tx edit dirs'"
        exit 1
    fi

    echo "$dir_paths"
}

get_dir_paths() {
    local dir_paths=$(read_dir_paths_file)
    local nested_dirs=$(
        echo "$dir_paths" | xargs -P 8 -I"[]" find "[]" -mindepth 1 -maxdepth 1 -type d -exec bash -c '
            base_path=$(dirname "${}")
            name=$(basename "$base_path")/$(basename "${}")
            echo "$name"
        ' \;
    )

    {
        echo "$dir_paths" | xargs -P 8 -I{} basename {};
        echo "$nested_dirs";
    } | sort | uniq
}

get_full_dir_path() {
    read_dir_paths_file |\
    xargs -P 8 -I"[]" find "[]" -maxdepth 1 -type d |\
    grep --color=never -E "/$1$" |\
    head -1
}

picker() {
    local input="$1"
    local preview_cmd="$2"

    local selected=$(
        echo "$input" |\
        fzf \
            --color=dark,gutter:-1 \
            --cycle \
            --tmux center,75%,80% \
            --reverse \
            --bind "tab:down" \
            --bind "btab:up" \
            --preview "$preview_cmd" \
            --preview-window 'up,75%,border-bottom' \
    )

    if [ -z "$selected" ]; then
        # echo "Error: Nothing selected"
        exit 1
    fi

    echo "$selected"
}
