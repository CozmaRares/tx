use std::ffi::CString;
use std::process::Command;
use std::str;

use anyhow::{bail, Context, Result};
use libc;

pub fn run_command(args: &[&str]) -> Result<String> {
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

pub fn execvp(args: &[&str]) -> Result<()> {
    if args.is_empty() {
        anyhow::bail!("No command provided for exec");
    }

    let c_args: Vec<CString> = args
        .iter()
        .map(|s| CString::new(*s).context("CString::new failed"))
        .collect::<Result<_>>()?;

    let mut c_ptrs: Vec<*const i8> = c_args.iter().map(|c| c.as_ptr()).collect();
    c_ptrs.push(std::ptr::null());

    unsafe {
        libc::execvp(c_args[0].as_ptr(), c_ptrs.as_ptr());
        let err = std::io::Error::last_os_error();
        anyhow::bail!("execvp failed: {}", err);
    }
}
