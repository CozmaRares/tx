use anyhow::{Context, bail};
use libc;
use std::{
    ffi::CString,
    io::Write,
    process::{Command, Stdio},
    str,
};

pub(super) fn run_command(args: &[&str]) -> anyhow::Result<String> {
    if args.is_empty() {
        bail!("No command provided");
    }

    let output = Command::new(args[0])
        .args(&args[1..])
        .output()
        .with_context(|| format!("Failed to execute command: {:?}", args))?;

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout)
            .with_context(|| "Failed to parse command output as UTF-8")?;
        Ok(stdout.trim().to_string())
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
        bail!("Command failed: {}\n{}", args.join(" "), stderr)
    }
}

pub(super) fn run_command_with_stdin(args: &[&str], input: &str) -> anyhow::Result<String> {
    if args.is_empty() {
        bail!("No command provided");
    }

    let mut child = Command::new(args[0])
        .args(&args[1..])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to spawn command: {:?}", args))?;

    if let Some(mut stdin) = child.stdin.take() {
        stdin
            .write_all(input.as_bytes())
            .with_context(|| "Failed to write to child stdin")?;
    }

    let output = child
        .wait_with_output()
        .with_context(|| format!("Failed to wait on child process: {:?}", args))?;

    if output.status.success() {
        let stdout = str::from_utf8(&output.stdout)
            .with_context(|| "Failed to parse command output as UTF-8")?;
        Ok(stdout.trim().to_string())
    } else {
        let stderr = str::from_utf8(&output.stderr).unwrap_or("Unknown error");
        bail!("Command failed: {}\n{}", args.join(" "), stderr)
    }
}

pub fn execvp(args: &[&str]) -> anyhow::Result<()> {
    if args.is_empty() {
        bail!("No command provided for exec");
    }

    let c_args: Vec<CString> = args
        .iter()
        .map(|s| CString::new(*s).context("CString::new failed"))
        .collect::<anyhow::Result<_>>()?;

    let mut c_ptrs: Vec<*const i8> = c_args.iter().map(|c| c.as_ptr()).collect();
    c_ptrs.push(std::ptr::null());

    unsafe {
        libc::execvp(c_args[0].as_ptr(), c_ptrs.as_ptr());
        let err = std::io::Error::last_os_error();
        bail!("execvp failed: {}", err);
    }
}
