# Kinesis DB

This is a custom database being developped that will be a crucial component in an upcoming, even
bigger project.

### Getting Started

After cloning the project, you should be getting no errors when running a `cargo run --force --bin kinesis-db` from the
directory of the cloned project. Furthermore, all tests should be OK when running a `cargo test`.

### Running

To run the project as a normal bin project, just do a

```bash
cargo run --bin kinesis-db
```

To run this project with Kinesis API, ensure the API project is cloned in the root directory and accessible in the
`api/` directory. Then just execute the bash script found in the root directory of this project itself to package
Kinesis DB as a JS package glued with WASM.

```
chmod +x build-wasm.sh
./build-wasm.sh
```

### Contributing

- Some improvements can be made here and there to enforce borrowing wherever it can be made to
  save up on the memory footprint of the app and to get some (not sure if noticeable) performance boost.
- More tests can be created in the `src/tests.rs` file in my opinion, there are still a couple of
  cases that aren't being covered. Splitting up the file into smaller files could be a good idea as well.
- More modularity: Some scripts are just straight up a couple of hundreds of lines long. Splitting files
  into their own subfolder(s) would make the project easier to work on.
- More reusability: There are many functions that do practically the same thing. For example, the
  `save_all_XYZ` functions in most of the scripts do the same thing except for which data to format and save,
  and the separator being used ('|' or ';'). Finding a solution to reduce those similar functions while still
  not compromising on readability would surely help in maintainability.

Note: It'd be really appreciated it if a `cargo fmt` was ran before submitting the PR, thanks!
