# Tonic Repo 1375

Reproduction of issue #1375 in tonic.

Here I have made 3 APIs that return a stream.

1. Example 1: futures::mpsc to return the stream (the solution in the issue)
2. Example 2: `async_stream` to return the stream my attempt at an alternative to tokio's mpsc that didn't work
3. Example 3: tokio's mpsc channel

You can either run the server and the client separately or run them both at once.
There are 3 binaries `server`, `client`, `both` which do these respectively.

Running as `both` lets you match up the timestamps more easily but if going into
the hyper logs the extra noise from both server and client in the same process
may be a pain to debug hence the 3 options.
