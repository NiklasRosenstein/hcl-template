# hcl-template

A simple wrapper around the [`hcl-rs`][https://crates.io/crates/hcl-rs] crate to render HCL template strings.

## Usage

    $ cat example.tftpl
    Hello ${name}!
    $ cat values.yaml
    name: World
    $ hcl-template --template example.tftpl --values values.yaml

## Known limitations

* Does not currently support sequences and maps in the values.
