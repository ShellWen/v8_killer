use once_cell::sync::Lazy;
use std::process;
use tracing::{info_span, Span};

static PID_SPAN: Lazy<Span> = Lazy::new(|| {
    let pid = process::id();
    #[cfg(not(target_os = "linux"))]
    let pid_span: Span = info_span!("process", pid);
    #[cfg(target_os = "linux")]
    let pid_span: Span = {
        fn read_host_pid() -> Option<u32> {
            let status = std::fs::read_to_string("/proc/self/status").ok()?;
            // format:
            // NSpid:  1381510 1
            let mut nspid_line = status
                .lines()
                .find(|line| line.starts_with("NSpid"))?
                .split_whitespace();
            let nspid = nspid_line.nth(1)?.parse::<u32>().ok()?;
            // if ns pid is None, it will return None
            nspid_line.next()?;
            Some(nspid)
        }
        let host_pid = read_host_pid();
        match host_pid {
            Some(host_pid) => info_span!("process", pid, host_pid, in_sandbox = true),
            None => info_span!("process", pid),
        }
    };
    pid_span
});

pub(crate) fn pid_span() -> Span {
    PID_SPAN.clone()
}
