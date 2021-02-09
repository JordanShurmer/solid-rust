# Solid Rust

A [SoLiD] server implemented in rust.

![https://github.com/JordanShurmer/solid-rust/actions?query=workflow%3A%22nightly+rust+test%22](https://github.com/JordanShurmer/solid-rust/workflows/nightly%20rust%20test/badge.svg)
![https://github.com/JordanShurmer/solid-rust/actions?query=workflow%3A%22beta+rust+test%22](https://github.com/JordanShurmer/solid-rust/workflows/beta%20rust%20test/badge.svg)
![https://github.com/JordanShurmer/solid-rust/actions?query=workflow%3A%22stable+rust+test%22](https://github.com/JordanShurmer/solid-rust/workflows/stable%20rust%20test/badge.svg)




## Status

Work (barely) In Progress. Not usable at all.

See the [issues](https://github.com/JordanShurmer/solid-rust/issues) and [Milestones](https://github.com/JordanShurmer/solid-rust/milestones) to get an idea of the on going work.

- [ ] HTTP
- [ ] LDP
- [ ] WAC
- [ ] WebId
- [ ] ...

### HTTP Related Conformance

- [x] MUST [Be a Server](https://solidproject.org/TR/protocol#http)
- [ ] MUST [support GET, HEAD, OPTIONS on resources](https://solidproject.org/TR/protocol#reading-resources)
- [ ] MUST [indicate methods in Allow header for authorized requests](https://solidproject.org/TR/protocol#reading-resources)
- [ ] MUST [indicate supported media-types with Accept-Put, Accept-Post, Accept-Patch](https://solidproject.org/TR/protocol#reading-resources)
  - [ ] MAY indicate those in OPTIONS requests
- [ ] MUST support [Conditional Requests (Etag, 412, 304 etc)](https://solidproject.org/TR/protocol#http)
- [ ] SHOULD provide [Server Caching](https://solidproject.org/TR/protocol#http)
- [ ] MAY support [Range/If-Range (optional)](https://solidproject.org/TR/protocol#http)
- [ ] MUST support [http authentication standards](https://solidproject.org/TR/protocol#http) (i.e. 401 or 404 for invalid creds)
- [ ] MUST [reject `PUT`, `POST`, `PATCH` with 400 when no `Content-Type` header](https://solidproject.org/TR/protocol#http)
- [ ] MUST [405 for methods not supported on the target resource](https://solidproject.org/TR/protocol#reading-writing-resources)
- [ ] MUST [respond 404 when POSTing to a non-existing url](https://solidproject.org/TR/protocol#writing-resources)
- [ ] MAY [use the Slug header for resource naming handling POST](https://solidproject.org/TR/protocol#resource-type-heuristics)
- [ ] MUST [allow acl/description resources to be modified with PUT and PATCH](https://solidproject.org/TR/protocol#writing-resources)



### LDP Related Conformance

There doesn't seem to be any already existing LDP servers in Rust, so this is from scratch.


- [ ] MUST [support LDP containers](https://solidproject.org/TR/protocol#resource-containment)
- [ ] MUST [create intermediate containers when creating](https://solidproject.org/TR/protocol#writing-resources)
- [ ] MUST [allow POSTing to a container to create](https://solidproject.org/TR/protocol#writing-resources)
  - [ ] MUST [create a container resource when Link Header rel=type ldp:Container header is present in the POST](https://solidproject.org/TR/protocol#writing-resources)
  - [ ] MUST [create the resource/container directly under the container in the url hierarchy](https://solidproject.org/TR/protocol#writing-resources)
- [ ] MUST [403 when trying to POST with a slug indicating the auxiliary resource](https://solidproject.org/TR/protocol#writing-resources)
- [ ] MUST [409 when someone tries to PUT or PATCH container's containment directly](https://solidproject.org/TR/protocol#writing-resources)
- [ ] MUST [405 a request to delete the storage root or its acl](https://solidproject.org/TR/protocol#deleting-resources)
- [ ] MUST [remove the reference of a deleted resources from it's container too](https://solidproject.org/TR/protocol#deleting-resources)
- [ ] MUST [delete auxiliary resources when deleting a resource](https://solidproject.org/TR/protocol#deleting-resources)
- [ ] MUST [support DELETE of empty containers](https://solidproject.org/TR/protocol#deleting-resources)
- [ ] MUST [409 a DELETE request of a non-empty container](https://solidproject.org/TR/protocol#deleting-resources)
  
- [x] Read Resources (GET, HEAD, OPTIONS)
  - [ ] Content Negotiation (`.ttl->application/ld+json` et al.) (`Accept` header)
    - [x] text/turtle
    - [ ] **application/ld+json**
- [ ] Write Resources
  - [ ] PUT to create
  - [ ] PUT to update
  - [ ] POST to create through container
- [ ] Read Containers
- [ ] Write Containers?

### Miscellaneous Conformance

- [ ] MUST [advertise Storage roots with pim:Storage header](https://solidproject.org/TR/protocol#storage)
- [ ] MUST [provide an acl doc on the storage roo](https://solidproject.org/TR/protocol#storage)
- [ ] MUST [have an acl:Control privileged role on that acl?](https://solidproject.org/TR/protocol#storage)
- [ ] MUST [have the same origin for subject and its auxiliary resource](https://solidproject.org/TR/protocol#auxiliary-resources)
- [ ] MUST NOT [associate multiple description resources with a resource](https://solidproject.org/TR/protocol#auxiliary-resources)
- [ ] MUST [require acl:Write to change a description resource](https://solidproject.org/TR/protocol#auxiliary-resources)
- [ ] MUST [require acl:Read to view a description resource](https://solidproject.org/TR/protocol#auxiliary-resources)


### Web Access Control

- [ ] MUST honor the acl roles
- [ ] MUST NOT [associate multiple acl docs with a resource](https://solidproject.org/TR/protocol#auxiliary-resources)
- [ ] MUST [require `acl:Control` privilege to access acl doc](https://solidproject.org/TR/protocol#auxiliary-resources)
- [ ] SHOULD [validate acl doc changes (e.g. shape validation)](https://solidproject.org/TR/protocol#auxiliary-resources)


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
- Inrupt's (deprecated?) [pod-server]
- [tokio docs](https://docs.rs/tokio)
- [hyper docs](https://docs.rs/hyper)

### Sepcs

- [ldp]
- [solid spec]

[ldp]: https://www.w3.org/TR/ldp/
[solid spec]: https://solidproject.org/TR/protocol#storage
[SoLid]: https://solid.github.io/
[pod-server]: https://github.com/inrupt/pod-server
[nss]: https://github.com/solid/node-solid-server
[newman]: https://learning.getpostman.com/docs/postman/collection_runs/command_line_integration_with_newman/
