# Hidasta

A simple cross-process signalling mechanism for shell scripts.

**This is not an officially supported Google product**

## Why?

Sometimes you need multiple shell script processes to wait for an event.

## How?

To wait:

`hidasta -w $socket_path`, called from any number of processes, will block until signalled.

To signal:

`hidasta -s $socket_path` just once. All instances currently waiting on the socket path will return, and the callers can continue.

## Where?

`$socket_path` is the path at which a new Unix Domain Socket will be created (and removed once signalled). Only works on Unix, but can use filesystem permissions to restrict who can signal or wait.
