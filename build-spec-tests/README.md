This is a helper tool to generate tests from the [Commonmark
spec](https://github.com/commonmark/commonmark-spec/blob/master/spec.txt)

    spec-test-builder spec.txt > tests/spec.rs
    cargo run -- -s 9 -e ../special-outputs.txt ../commonmark-spec.txt > ../tests/spec.rs
