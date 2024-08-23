use std::process::*;
use std::sync::*;
use std::thread::*;

fn main() {
    let mut attempts = 0;
    let pid_foo: Option<u32> = None;

    loop {
        let (tx, rx) = mpsc::channel::<u32>();

        if attempts >= 100 {
            break;
        }
        attempts += 1;

        match pid_foo {
            Some(pid) => {
                _ = Command::new("kill")
                    .arg(&pid.to_string())
                    .spawn()
                    .expect("cannot not spawn process 'kill'")
                    .wait()
                    .expect("cannot wait for termination of process 'kill'");
            }
            None => {}
        }

        spawn(move || {
            let process_foo = Command::new("./foo")
                .spawn()
                .expect("cannot not spawn process './foo'");
            // Not waiting for the process "./foo" to terminate!
            let pid = process_foo.id();
            tx.send(pid).expect("cannot send id of process './foo'");
        });
        let pid_foo = Some(rx.recv().expect("cannot receive id of process './foo'"));

        match pid_foo {
            Some(pid) => {
                let output = Command::new("ps")
                    .args(["-o", "etimes=", "-p", &pid.to_string()])
                    .stdout(Stdio::piped())
                    .output()
                    .expect("cannot get output of process 'ps'");
                assert_eq!(
                    String::from_utf8_lossy(&output.stdout),
                    "      0\n",
                    "at attempt #{}",
                    attempts
                );
            }
            None => {}
        }
    }

    println!(
        "Attempted {} times but couldn't reproduce the issue!",
        attempts
    );
}
