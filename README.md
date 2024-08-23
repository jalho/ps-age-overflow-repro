# ps age overflow

```
$ ps --version
ps from procps-ng 4.0.2
```

```
$ rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)

$ rustc main.rs

$ ./main
thread 'main' panicked at main.rs:46:13:
assertion `left == right` failed: at attempt #7
  left: "4123168608\n"
 right: "      0\n"
```

The demo repeatedly starts a process from executable [`./foo`](./foo), reads its
age (using external command `ps -o etimes= -p $PID`) and kills it (using
external command `kill`). Each subsequent attempt to reproduce the issue starts
with the kill command so the process' age is expected to always be close to 0.
The issue is that sometimes the age is reported to be 4123168608, which is a
little over four billion, hence the hint of an overflow in the title.
