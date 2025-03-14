#! /bin/sh

DATA_DIR="$TX_ROOT/data"
DIR_PATHS_FILE="$DATA_DIR/paths.txt"

LAYOUT_EXT=".layout.sh"
FRAGMENT_EXT=".fragment.sh"

FZF_ARGS="--color=dark,gutter:-1 --cycle --tmux center,75%,80%"

run_tmux() {
    tmux "$@" 2>/dev/null
}

attach_or_switch() {
    if [ -z "$TMUX" ]; then
        run_tmux attach-session -t "$1"
    else
        run_tmux switch-client -t "$1"
    fi
}

get_tmux_sessions() {
    run_tmux ls -F "#{session_name}"
}

get_layouts() {
    find "$DATA_DIR" -type f -name "*$LAYOUT_EXT" -exec basename {} "$LAYOUT_EXT" \;
}

get_fragments() {
    find "$DATA_DIR" -type f -name "*$FRAGMENT_EXT" -exec basename {} "$FRAGMENT_EXT" \;
}

get_dir_paths() {
    if [ -d "$DIR_PATHS_FILE" ]; then
        echo "Error: $DIR_PATHS_FILE is a directory"
        exit 1
    fi

    if [ ! -f "$DIR_PATHS_FILE" ]; then
        touch "$DIR_PATHS_FILE"
    fi

    local dir_paths=$(cat "$DIR_PATHS_FILE")
    local nested_dirs=$(
        echo "$dir_paths" | xargs -I[] find [] -mindepth 1 -maxdepth 1 -type d -exec bash -c '
            base_path=$(dirname "${}")
            name=$(basename "$base_path")/$(basename "${}")
            echo "$name"
        ' \;
    )

    {
        echo "$dir_paths" | xargs -I{} basename {};
        echo "$nested_dirs";
    } | sort | uniq
}

