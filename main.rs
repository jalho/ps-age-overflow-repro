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

        let output = Command::new("ps")
            .args(["-o", "etimes=", "-p", &pid_foo.to_string()])
            .stdout(Stdio::piped())
            .output()
            .expect("cannot get output of process 'ps'");
        assert_eq!(
            String::from_utf8_lossy(&output.stdout),
            "      0\n",
            "at attempt #{}",
            attempt + 1
        );
    }

    println!(
        "Attempted {} times but couldn't reproduce the issue! PIDS used by executable './foo': {:?}",
        ATTEMPTS, pids
    );
}
