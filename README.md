# Mutable Threads Example (Rust)

This example runs several parallel threads, each prints to stdout a local counter.
Also it runs a local TCP server, that listens, by default, on 127.0.0.1:7343 and accepts several simple commands:

* `version`  - print the application version
* `shutdown` - shut down the application
* `mute`     - mute threads (disable output )
* `unmute`   - unmute threads (enable output)

Example:

```
$ echo "mute" | nc 127.0.0.1 7343
muted

```
