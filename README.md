# Smart Pointer

A rust module specifying traits for smart pointer functionality, i.e. shared ownership of values. The main intention is to enable data structures that are parameterized by the pointer type to be used internally, allowing the user to enable features such as thread-safety or the ability to handle self-containment as needed. The traits can also serve as a feature checklist for implementers of smart pointers.

See the module documentation for more details.

Note that the usage of this crate remains fairly restricted as long as rust does not support higher-kinded types. The [archery](https://crates.io/crates/archery) crate emulates higher-kinded smart pointers, but has a coarser interface than this crate.
