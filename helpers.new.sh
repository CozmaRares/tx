#! /bin/sh

DATA_DIR="$TX_ROOT/data"
LAYOUTS_EXT=".layout.sh"
FRAGMENTS_EXT=".fragment.sh"
SESSIONIZER_PATHS="$DATA_DIR/sessionizer.txt"

run_tmux() {
    tmux "$@" 2>/dev/null
}

is_session() {
    run_tmux ls -F "#{session_name}" | grep -qx "$1"
}

find_file() {
    local file="$1"
    local ext="$2"
    local matched_file=$(find "$DATA_DIR" -maxdepth 1 -type f -name "$file$ext")

    if [ -n "$matched_file" ]; then
        echo "$matched_file"
    fi
}

find_layout() {
    find_file "$1" "$LAYOUTS_EXT"
}

find_fragment() {
    find_file "$1" "$FRAGMENTS_EXT"
}

get_sessionizer_paths() {
    if [[ -f "$SESSIONIZER_PATHS" ]]; then
        cat "$SESSIONIZER_PATHS"
    else
        echo ""
    fi
}

find_dirs() {
    local path="$1"
    find "$path" -mindepth 1 -maxdepth 1 -type d | sort
}

find_sessionizer_path() {
    local path="$1"
    local regex="/$path$"

    get_sessionizer_paths |\
    while read -r path; do
        if grep -qE "$regex" <<< "$path"; then
            echo "$path"
            break
        fi

        path=$(
            find_dirs "$path" |\
            while read -r path; do
                if grep -qE "$regex" <<< "$path"; then
                    echo "$path"
                    break
                fi
            done
        )

        if [ -n "$path" ]; then
            echo "$path"
            break
        fi
    done
}
