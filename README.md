# ps age overflow

```
$ ps --version
ps from procps-ng 4.0.2
```

```
$ rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)

$ rustc main.rs

$ git log HEAD | head -n 1
commit 87e20f674d51d6a3cad13a3bcb730a38ba113c15

$ ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s for pid 126028 at attempt #4 -- all used pids at this point: 126010, 126015, 126021, 126028
  left: false
 right: true
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s for pid 126056 at attempt #1 -- all used pids at this point: 126056
  left: false
 right: true
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

$ ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s for pid 126066 at attempt #1 -- all used pids at this point: 126066
  left: false
 right: true
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
```

The demo repeatedly starts a process from executable [`./foo`](./foo), reads its
age (using external command `ps -o etimes= -p $PID`) and kills it (using
external command `kill`). Each subsequent attempt to reproduce the issue starts
with the kill command so the process' age is expected to always be close to 0.
The issue is that sometimes the age is reported to be 4123168608, which is a
little over four billion, hence the hint of an overflow in the title.
