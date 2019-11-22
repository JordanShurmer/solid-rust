# Solid Rust

A [SoLiD] server implemented in rust.

![https://github.com/JordanShurmer/solid-rust/actions?query=workflow%3A%22nightly+rust+test%22](https://github.com/JordanShurmer/solid-rust/workflows/nightly%20rust%20test/badge.svg)
![https://github.com/JordanShurmer/solid-rust/actions?query=workflow%3A%22beta+rust+test%22](https://github.com/JordanShurmer/solid-rust/workflows/beta%20rust%20test/badge.svg)
![https://github.com/JordanShurmer/solid-rust/actions?query=workflow%3A%22stable+rust+test%22](https://github.com/JordanShurmer/solid-rust/workflows/stable%20rust%20test/badge.svg)




## Status

Work (barely) In Progress. Not useable at all.

See the [issues](https://github.com/JordanShurmer/solid-rust/issues) and [Milestones](https://github.com/JordanShurmer/solid-rust/milestones) to get an idea of the on going work.

- [ ] LDP support
- [ ] WAC
- [ ] WebId
- [ ] ...

### HTTTP Status

- [x] Read http resources (GET, HEAD, OPTIONS)
- [ ] Write http resources (POST, PUT?)
- [x] Conditional Requests (Etag, 412, 304 etc)
  - [ ] Range/If-Range (optional)

### LDP Status

There doesn't seem to be any already existing LDP servers in Rust, so this is from scratch.

- [ ] Read Resources (GET, HEAD, OPTIONS)
  - [ ] Content Negotiation (`.ttl->application/ld+json` et al.) (`Accept` header)
    - [x] text/turtle
    - [ ] **application/ld+json**
- [ ] Write Resources
  - [ ] POST directly
  - [ ] POST through container
- [ ] Read Containers
- [ ] Write Containers

## Architecture

This is set up as a [Cargo Workspace](https://doc.rust-lang.org/nightly/book/ch14-03-cargo-workspaces.html). The workspace parent crate is here, and the [server](./server) crate is a member.

This parent crate is the CLI binary which is a very minimal wrapper around everything in ther server crate. The [server](./server) crate is the actual application.

The gist of the code structure is as follows: there is a module for each of the major specs that are involved in a Solid server

- HTTP: The [http](./server/http) module
  handles various things about the HTTP related specs. Handles 404 detection, Conditional Requests, Accept Header, etc

- LDP: The [ldp](./server/ldp) module
  handles the things which are particular to the LDP specs. Handles LDP-Resource and LDP-Container related logic

More TBD.

## Running the server

Currently there are no pre-build binaries. To run the server you must run it from source using `cargo`.

```bash
# clone the repo
cd solid-rust
cargo run # [-- -p port-number]
```

## Tests

Tests are run by running Postman's [newman] from within a cargo integration test. The tests will start the server on port 7171, then execute various requests against 127.0.0.1:7171.

Unfortunately, this means you have to have [`newman`] installed. `npm install -g newman`. Alternatively, you can install the man Postman client and import the test suite and run it there.

More on using Postman for tests [here](https://www.getpostman.com/automated-testing).

### Running the tests

```bash
cd solid-rust/server
cargo test
```

### Writing tests

When it makes sense, simply write unit tests directly in the rust source code like normal.

Intergation tests are written in nodejs, using postmam. The best way to write more integration tests is to use [Postman](https://www.getpostman.com/). You can import [the test suite](./server/tests/test-suite.postman_collection.json), add your own requests/tests scripts, then export back into the repo.

## Contributing

Contributions are welcome!

- Fork the project
- make your changes
- follow the guidelines
- submit a pull request

**One major contribution we need is json-ld support. I haven't found any existing rust crate for interacting with rdf through json-ld.**

### Guidelines

- Write tests ([preferrably first](http://www.butunclebob.com/ArticleS.UncleBob.TheThreeRulesOfTdd))
- Write good documentation ([preferrably first](https://gist.github.com/zsup/9434452))

## Resources

- [Solid landing page][SoLiD]
- [The Node Solid Server][nss]
- Inrupt's new [pod-server]
- [tokio docs](https://docs.rs/tokio)
- [hyper docs](https://docs.rs/hyper)

### Sepcs

- [ldp]
- [solid spec]

[ldp]: https://www.w3.org/TR/ldp/
[solid spec]: https://github.com/solid/specification
[SoLid]: https://solid.github.io/
[pod-server]: https://github.com/inrupt/pod-server
[nss]: https://github.com/solid/node-solid-server
[newman]: https://learning.getpostman.com/docs/postman/collection_runs/command_line_integration_with_newman/
