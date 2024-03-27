# rust-practice

#### Installations

- clippy (like a linter)

        rustup component add clippy

        // to add clippy
        #[!deny(clippy::all)]

- cargo watch (to run files)

        cargo install cargo-watch


#### Run

- using cargo-watch

    cargo-watch -qc -x run -x clippy
