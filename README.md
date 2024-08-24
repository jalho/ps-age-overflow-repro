resolved in https://gitlab.com/procps-ng/procps/-/issues/346

---

# ps age overflow

Sometimes `ps` reports a right before launched process' age to be 4123168608
seconds when another process launched from the same executable was killed right
before the new launch. The age of the process that was just launched should be
less than a second, perhaps 0, but not 4123168608.

```
$ ps --version
ps from procps-ng 4.0.2
```

```
$ uname -rmo
6.1.0-23-amd64 x86_64 GNU/Linux

$ lsb_release -a
Distributor ID: Debian
Description:    Debian GNU/Linux 12 (bookworm)
Release:        12
Codename:       bookworm
```

## demo: [`./main.rs`](./main.rs)

```
$ rustc --version
rustc 1.79.0 (129f3b996 2024-06-10)

$ rustc main.rs

$ git log HEAD | head -n 1
commit 87e20f674d51d6a3cad13a3bcb730a38ba113c15

$ ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s for pid 126028 at attempt #4 -- all used pids at this point: 126010, 126015, 126021, 126028

$ ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s for pid 126056 at attempt #1 -- all used pids at this point: 126056

$ ./main
thread 'main' panicked at main.rs:49:13:
assertion `left == right` failed: got age 4123168608s for pid 126066 at attempt #1 -- all used pids at this point: 126066
```

The demo repeatedly starts a process from executable [`./foo`](./foo), reads its
age (using external command `ps -o etimes= -p $PID`) and kills it (using
external command `kill`). Each subsequent attempt to reproduce the issue starts
with the kill command so the process' age is expected to always be close to 0.
The issue is that sometimes the age is reported to be 4123168608, which is a
little over four billion, hence the hint of an overflow in the title.

**N.B.:** This demo does not reproduce the issue consistently. On my system the
issue sometimes occurs on every run of the demo and then no longer occurs for
whatever reason when I try to run the demo a bit later. When the demo fails to
reproduce the issue, it prints something like the following:

```
 ./main
Attempted 100 times but couldn't reproduce the issue! PIDS used by executable './foo': 139415, 139420, 139426, 139433, 139441, 139450, 139460, 139471, 139483, 139496, 139510, 139525, 139541, 139558, 139576, 139595, 139615, 139636, 139658, 139681, 139705, 139739, 139788, 139815, 139844, 139873, 139903, 139934, 139966, 139999, 140081, 140116, 140152, 140189, 140227, 140266, 140306, 140347, 140389, 140432, 140476, 140521, 140567, 140614, 140662, 140711, 140761, 140812, 140864, 140917, 141032, 141087, 141143, 141200, 141268, 141423, 141483, 141544, 141606, 141669, 141733, 141798, 141864, 141931, 141999, 142068, 142138, 142209, 142281, 142354, 142428, 142503, 142579, 142656, 142734, 142817, 142897, 142978, 143060, 143143, 143227, 143312, 143398, 143485, 143573, 143662, 143752, 143843, 143935, 144028, 144122, 144217, 144313, 144410, 144508, 144607, 144707, 144808, 144910, 145013
```

**N.B.:** On my system when the issue occurs the curious process age seems to
always be exactly 4123168608.
