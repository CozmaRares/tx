use crate::commands::runner::run_command_with_stdin;

pub(super) const PROGRAM: &str = "fzf";

pub fn pick_session(input: &str) -> anyhow::Result<String> {
    let preview_cmd = r#"echo {} | awk '{
        match($0, /\(([^)]+)\)/, arr);
        type = substr(arr[1], 1, 1);
        gsub(/\(.*\)/, "", $0);
        name = $0
        gsub(/[[:space:]]+$/, "", name)
        printf "tx preview -%s \"%s\"", type, name
    }' | bash"#;

    picker(input, preview_cmd)
}

pub fn pick_dir(input: &str) -> anyhow::Result<String> {
    let preview_cmd = r#"basename {} | xargs -I{} tx preview -d {}"#;
    picker(input, preview_cmd)
}

fn picker(input: &str, preview_cmd: &str) -> anyhow::Result<String> {
    let cmd = vec![
        "fzf",
        "--color=dark,gutter:-1",
        "--cycle",
        "--tmux",
        "center,75%,80%",
        "--reverse",
        "--bind",
        "tab:down",
        "--bind",
        "btab:up",
        "--preview",
        preview_cmd,
        "--preview-window",
        "up,75%,border-bottom",
    ];

    run_command_with_stdin(&cmd, input)
}
