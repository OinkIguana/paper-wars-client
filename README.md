[Paper Wars]: https://github.com/foxfriends/paper-wars
[server]: https://github.com/foxfriends/paper-wars-server

# Paper Wars API Client

The [Paper Wars][] client provides a friendly and type-safe interface to the API exposed by
the [server][]. Designed to be embedded in any front-end, this client aims to be compiled to
WASM or a native C library.

## Setup

1.  Set up the [server][].
2.  Compile the server (with `cargo build`). One of the output binaries is required to build 
    the client.
