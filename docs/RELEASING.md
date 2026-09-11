# Release setup

The workflows are installed locally; no repository remote, registry entry or
trusted publisher is created by adding these files. Package metadata currently
uses `https://github.com/bastibl/rtlsdr-nusb`; adjust it if hosting elsewhere.

Before using the release workflow:

1. Push the repository, including `Cargo.lock`, to its GitHub remote and enable
   Actions. Ordinary CI never runs the ignored hardware test.
2. Establish the crate on crates.io. A first publication may need a manual
   `cargo publish --locked` with the maintainer's credentials; follow the
   [current registry setup instructions](https://crates.io/docs/trusted-publishing).
3. Add a crates.io trusted publisher for the actual GitHub owner/repository and
   workflow filename `release.yml`. The workflow does not specify a GitHub
   environment. It uses the official
   [crates.io authentication action](https://github.com/rust-lang/crates-io-auth-action)
   with `id-token: write`; no permanent registry token is stored in the workflow.

For subsequent releases, update `Cargo.toml` and `Cargo.lock`, run CI and any
appropriate hardware checks, commit the release and push an annotated
`v<version>` tag. The release job checks the tag/version match, runs the reusable
CI workflow (including all native OS jobs), verifies the package, publishes it
and attaches the `.crate` archive to a GitHub release. Use a new version for each
crates.io publication; published versions cannot be overwritten.

If publication succeeds but GitHub release creation fails, create the missing
GitHub release from the existing tag/archive. Re-running publication of the same
version will fail because the registry already contains it.

Local package review without publishing:

```sh
cargo package --locked --allow-dirty
cargo package --locked --allow-dirty --list
```

`--allow-dirty` is for reviewing local work; CI/release requires a clean checkout.
