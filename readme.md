for updating and uninstalling
rustup

to uninstall project
rustup self uninstall

rustc --version

to build file or compile
rustc main.rs

to format file
for file := rustfmt main.rs

*********cargo cli tool to manage rust projects

for project formatting
cargo fmt 

for project build 
cargo build or cargo b

project build for release
cargo build --release

to run the project
cargo run --quiet
cargo r --quiet

to check runtime error or compilation error
cargo check

rustc --explain E0384

there is variable shadowing in rust

there is scoping in rust


<!-- function -->
return type of a empty function is unit or empty tuple

Rust does not have ternary operator


<!-- ***********************************************  task related ******************************************* -->
<!-- ectix-web: web framework for building web applications in Rust 
        High performance (built on the Actix actor system)
        Asynchronous support (non-blocking I/O)
        Type safety and memory safety
        Great for building APIs and microservices
        In short: Actix-web = Rust + Speed + Safety for Web Apps. -->



<!-- *********************************************** Serde -->

<!-- Serde is a Rust framework for serializing and deserializing data.

In short:

Serialize = Convert Rust data → JSON, TOML, etc.
Deserialize = Convert JSON, TOML, etc. → Rust data -->


<!-- *********************************************** quick-xml -->

<!-- 
quick-xml is a fast and lightweight XML parser for Rust.

In short:

Parses and writes XML efficiently
Designed for speed and low memory usage
Great for handling large or streaming XML data
Think of it as: Rust + Fast XML Processing.
 -->


 <!-- *********************************************** Scylla -->

 <!-- Scylla is a high-performance NoSQL database, compatible with Apache Cassandra.

In short:

Built in C++ for speed and low latency
Handles massive workloads with auto-sharding
Ideal for real-time big data and distributed systems
Think of it as: Cassandra on steroids -->

<!--  ***********************************************  uuid-->

<!-- UUID stands for Universally Unique Identifier.

In short:

A 128-bit unique ID used to identify things (like users, sessions, resources)
Looks like: 550e8400-e29b-41d4-a716-446655440000
Common in databases, APIs, and distributed systems
Think of it as: A globally unique name tag  -->
