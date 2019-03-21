# Rust-yang

[![Build Status](https://travis-ci.org/ivajloip/rust-yang.svg?branch=master)](https://travis-ci.org/ivajloip/rust-yang)
[![Latest Version]][crates.io]
[![Docs]][doc.rs]
[![Gitter chat](https://badges.gitter.im/Join%20Chat.svg)](https://gitter.im/rust-yang/rust-yang)

Package rust-yang provides structures and tools to parse yang files and build
their tree like in memory representation.

*NOTE*: This is a toy-project that I undertake in order to learn more about
Rust. Currrently it is far from stable or finished. Use at your own risk :)

## Already present

The following structures are implemented (+ fields):
* Module parsing

## Next steps

I plan to implement soon:

* [ ] Leaf
* [ ] Container
* [ ] Typedef
* [ ] Grouping
* [ ] Identity
* [ ] Extension
* [ ] Choice
* [ ] Using
* [ ] Import
* [ ] Leaf-list
* [ ] List
* [ ] Anyxml
* [ ] Augments
* [ ] Include
* [ ] Notification
* [ ] RPC
* [ ] Possibly others

## Used code and inspiration

I would like to thank the projects [nom][1] :)

[1]: https://github.com/Geal/nom
[Latest Version]: https://img.shields.io/crates/v/rust-yang.svg
[crates.io]: https://crates.io/crates/rust-yang
[Docs]: https://docs.rs/rust-yang/badge.svg
[doc.rs]: https://docs.rs/rust-yang
