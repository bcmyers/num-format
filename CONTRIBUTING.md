## Contributing to num-format

**num-format** welcomes contribution from everyone in the form of suggestions, bug
reports, pull requests, and feedback. This document gives some guidance if you
are thinking of helping us.

Please reach out here in a GitHub issue if we can do anything to help you contribute.

When reporting a bug or asking for help, please include enough details so that
the people helping you can reproduce the behavior you are seeing.

When making a feature request, please make it clear what problem you intend to
solve with the feature, any ideas for how num-format could support solving that
problem, any possible alternatives, and any disadvantages.

### Obtaining the source code

**num-format** uses a git submodule. To download everything you will need for
development, including the submodule:

```bash
$ git clone git@github.com:bcmyers/num-format.git
$ cd num-format
$ git submodule init
$ git submodule update
```

### Running the test suite

We encourage you to check that the test suite passes locally before submitting a
pull request with your changes. If anything does not pass, typically it will be
easier to iterate and fix it locally than waiting for the CI servers to run
tests for you.

To run the full test suite:

* Make sure you're in the root directory (the directory with the workspace Cargo.toml), and then...

```bash
$ ./scripts/test.sh
```

### num-format-dev

`num-format/src/locale.rs` is a programatically generated rust module. To modify
it, you will need to work with **num-format-dev**, a *separate* rust crate
included at the root of the repository but **not** published to crates.io.

num-format-dev produces a binary whose purpose is to parse the json files
located in the git submodule `num-format-dev/cldr-numbers-full` and
output a new `num-format/src/locale.rs`.

To run num-format-dev (thereby re-creating `num-format/src/locale.rs`):

* Make sure you're in the root directory (the directory with the workspace Cargo.toml), and then...

```bash
$ ./scripts/locale.sh
```

You only need to run num-format-dev if you wish to modify `num-format/src/locale.rs`.
If you do **not** wish to modify `num-format/src/locale.rs`, the above is not
necessary.

## Code of Conduct

In all num-format-related forums, we follow the [Rust Code of Conduct]. For
escalation or moderation issues please contact Brian (brian.carl.myers@gmail.com)
instead of the Rust moderation team.

[Rust Code of Conduct]: https://www.rust-lang.org/conduct.html
