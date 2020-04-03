## How to use

1. Copy `blank.rs`

    ```sh
    cp src/blank.rs src/<number of the problem>.rs
    ```

1. Add build target in `Cargo.toml` 

    ```toml
    # Cargo.toml
    [[bin]]
    name = "<number of the problem>"
    path = "src/<number of the problem>.rs"
    ```

1. **Solve the problem**

1. Run unit test

    ```sh
    cargo test --bin <number of the problem>
    ```

1. Run program

    ```sh
    cargo run --bin <number of the problem>
    ```
