# ps age overflow

```
$ ps --version
ps from procps-ng 4.0.2
```

```
$ git show HEAD | head -n 1
commit 52f5e84713a935e22e9b6eee0391d4d40877f32c

$ rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)

$ rustc main.rs

$ time ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s at attempt #1
  left: false
 right: true
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real    0m0.005s
user    0m0.005s
sys     0m0.000s

$ time ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s at attempt #8
  left: false
 right: true
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real    0m0.131s
user    0m0.049s
sys     0m0.081s

$ time ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s at attempt #6
  left: false
 right: true
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

real    0m0.074s
user    0m0.035s
sys     0m0.039s
```

The demo repeatedly starts a process from executable [`./foo`](./foo), reads its
age (using external command `ps -o etimes= -p $PID`) and kills it (using
external command `kill`). Each subsequent attempt to reproduce the issue starts
with the kill command so the process' age is expected to always be close to 0.
The issue is that sometimes the age is reported to be 4123168608, which is a
little over four billion, hence the hint of an overflow in the title.
