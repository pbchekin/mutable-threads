# Mutable Threads Example (Rust)

This example runs several parallel threads, each prints to stdout a local counter.
Also it runs a local TCP server, that listens, by default, on 127.0.0.1:7343 and accepts several simple commands:

* `shutdown` - shut down the application
* `mute N`   - mute threads (disable output) for N seconds
* `unmute`   - unmute threads (enable output)

Example:

In the terminal 1:

```
$ cargo run
```

In the terminal 2:

```
$ echo mute | nc 127.0.0.1 7343
ok

```
