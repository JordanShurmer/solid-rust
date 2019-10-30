# Solid Rust

A [SoLiD] server implemented in rust.

## Status

Work (barely) In Progress. Not useable at all.

See the [issues](https://github.com/JordanShurmer/solid-rust/issues) and [Milestones](https://github.com/JordanShurmer/solid-rust/milestones) to get an idea of the on going work.

### Tests

Need to get some integration tests set up. These don't have to be in rust.. they would need to 1) start the rust server, 2) make a series of requests, 3) validate the responses.

Perhaps Postman's [newman](https://learning.getpostman.com/docs/postman/collection_runs/command_line_integration_with_newman/) CLI would be a good fit here.

### LDP

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

- [main.rs](./src/main.rs) - entry point. Contains the main http request handler which dispatches the request to one of the other modules depending on which one is relevant
- [our.rs](./src/our.rs) - contains shared type definitions. e.g. `our::ServerResult`
- [ldp module](./src/ldp) - An LDP server. Handles most requests once someone is logged in.

More TBD. Need to think about test-ability more

## Running the server

```bash
# clone the repo
cd solid-rust
cargo run
```

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
- [tide](https://github.com/rustasync/tide) - async/await http in rust

[SoLid]: https://solid.github.io/
[pod-server]: https://github.com/inrupt/pod-server
[nss]: https://github.com/solid/node-solid-server
