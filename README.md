# Solid Rust

A [SoLiD] server implemented in rust.

## Status

Work (barely) In Progress. Not useable at all.

See the [issues](https://github.com/JordanShurmer/solid-rust/issues) and [Milestones](https://github.com/JordanShurmer/solid-rust/milestones) to get an idea of the on going work.

- [ ] LDP support
- [ ] WAC
- [ ] WebId
- [ ] ...

### Tests Status

- [x] integration tests (you must have [`newman`] installed)

### HTTTP

- [ ] Conditional Requests
  - [x] If-Match
  - [ ] If-Unmodified-Since
  - [ ] If-None-Match
  - [ ] If-Modified-Since
  - [ ] Range/If-Range (optional)

### LDP Status

The first step I'm taking is to implement the LDP portion of a Solid server. There doesn't seem to be any already existing LDP servers in Rust, so this is from scratch.

- [ ] Read Resources (GET, HEAD, OPTIONS)
  - [x] Turtle Resources
  - [x] jsonld Resources
  - [x] Link header
  - [x] Allow header
  - [x] ETag Header
  - [ ] Conditional Requests
  - [ ] Content Negotiation (`.ttl->application/ld+json` et al.) (`Accept` header)
- [ ] Write Resources
- [ ] Containers

## Architecture

This is actually set up as a [Cargo Workspace](https://doc.rust-lang.org/nightly/book/ch14-03-cargo-workspaces.html). The workspace parent crate is here, and the [server](./server) crate is a member.

This parent crate is the CLI binary which is a very minimal wrapper around everything in ther server crate. The [server](./server) crate is the actual application.

More TBD.

Lifecycle of a request:

- Construct a Resource
  - path
    - file
  - etag
  - last modified
- 

- Check Conditional Headers
- Handle the request as an LDP Resource

## Running the server

```bash
# clone the repo
cd solid-rust
cargo run
```

## Tests

Tests are run by running Postman's [newman] from within a cargo integration test. The tests will start the server on port 7171, then execute various requests against 127.0.0.1:7171.

Unfortunately, this means you have to have [`newman`] installed. `npm install -g newman`. Alternatively, you can install the man Postman client and import the test suite and run it there.

More on using Postman for tests [here](https://www.getpostman.com/automated-testing).

### Running the tests

```bash
cd server
cargo test

# or, you can invoke newman directly if you want for some reason
cd server
newman run test-suite.postman_collection.json -e test-default-env.postman_environment.json
```

### Writing tests

The best way to write more integration tests is to use [Postman](https://www.getpostman.com/). You can import [the test suite](./server/tests/test-suite.postman_collection.json), add your own requests/tests scripts, then export back into the repo.

## Contributing

Contributions are welcome!

- Fork the project
- make your changes
- follow the guidelines
- submit a pull request

**It may be hard for now, since there's not really anything to contribute too.**

### Guidelines

- Write tests ([preferrably first](http://www.butunclebob.com/ArticleS.UncleBob.TheThreeRulesOfTdd))
- Write good documentation ([preferrably first](https://gist.github.com/zsup/9434452))

## Resources

- [Solid landing page][SoLiD]
- [The Node Solid Server][nss]
- Inrupt's new [pod-server]
- [tokio docs](https://docs.rs/tokio)
- [hyper docs](https://docs.rs/hyper)

[SoLid]: https://solid.github.io/
[pod-server]: https://github.com/inrupt/pod-server
[nss]: https://github.com/solid/node-solid-server
[newman]: https://learning.getpostman.com/docs/postman/collection_runs/command_line_integration_with_newman/
