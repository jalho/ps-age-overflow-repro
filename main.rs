use std::process::*;
use std::sync::*;
use std::thread::*;

fn main() {
    const ATTEMPTS: usize = 100;
    let mut pids: [Option<u32>; ATTEMPTS] = [None; ATTEMPTS];

    for attempt in 0..=ATTEMPTS - 1 {
        let (tx, rx) = mpsc::channel::<u32>();

        if attempt != 0 {
            _ = Command::new("kill")
                .arg(&pids[attempt - 1].unwrap().to_string())
                .spawn()
                .expect("cannot not spawn process 'kill'")
                .wait()
                .expect("cannot wait for termination of process 'kill'");
        }

        spawn(move || {
            let process_foo = Command::new("./foo")
                .spawn()
                .expect("cannot not spawn process './foo'");
            // Not waiting for the process "./foo" to terminate!
            let pid = process_foo.id();
            tx.send(pid).expect("cannot send id of process './foo'");
        });
        let pid_foo = rx.recv().expect("cannot receive id of process './foo'");
        pids[attempt] = Some(pid_foo);

        for &pid in pids.iter().flatten().rev() {
            let output = Command::new("ps")
                .args(["-o", "etimes=", "-p", &pid.to_string()])
                .stdout(Stdio::piped())
                .output()
                .expect("cannot get output of process 'ps'");
            let stdout_utf8 = String::from_utf8_lossy(&output.stdout);
            let stdout_trimmed = stdout_utf8.trim();
            let stdout_u64 = stdout_trimmed
                .parse::<u64>()
                .expect(&format!("cannot parse stdout as u64: \"{}\"", stdout_utf8));

            /*
              Looking for a process whose age is reported to be way too long
              (4123168608 seconds to be exact, but let's say anything at around
              a minute or more shall trigger the assertion failure).
            */
            assert_eq!(
                stdout_u64 < 60, // process' reported age in seconds
                true,
                "got age {}s for pid {} at attempt #{} -- all used pids at this point: {}",
                stdout_u64,
                pid,
                attempt + 1,
                pids.iter().flatten().map(|n| { n.to_string() }).collect::<Vec<String>>().join(", "),
            );
        }
    }

    println!(
        "Attempted {} times but couldn't reproduce the issue! PIDS used by executable './foo': {}",
        ATTEMPTS,
        pids.map(|n| { n.unwrap().to_string() }).join(", ")
    );
}
