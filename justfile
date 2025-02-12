build:
    RUSTFLAGS="-Awarnings -C link-arg=-nostartfiles" \
        cargo build \
        --target=x86_64-unknown-linux-gnu \
        --features start

    # RUSTFLAGS="-Awarnings -C relocation-model=pic" \
    #     cargo build \
    #     --target=x86_64-unknown-linux-gnu \
    #     -Zbuild-std


lint:
    cargo fmt --all -- --check

    cargo clippy --target=x86_64-unknown-linux-gnu \
        --no-default-features \
        --features panic_handler -- \
        -D warnings \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_extern_crates \
        -D unused_import_braces \
        -D unused_qualifications \
        -D clippy::all \
        -D clippy::correctness \
        -D clippy::suspicious \
        -D clippy::complexity \
        -D clippy::perf \
        -D clippy::style \
        -A clippy::missing_safety_doc \
        -A non_upper_case_globals \
        -A non_camel_case_types


lint_fix:
    cargo clippy --fix --allow-dirty --target=x86_64-unknown-linux-gnu \
        --no-default-features \
        --features panic_handler -- \
        -D warnings \
        -D trivial_casts \
        -D trivial_numeric_casts \
        -D unused_extern_crates \
        -D unused_import_braces \
        -D unused_qualifications \
        -D clippy::all \
        -D clippy::correctness \
        -D clippy::suspicious \
        -D clippy::complexity \
        -D clippy::perf \
        -D clippy::style \
        -A clippy::missing_safety_doc \
        -A non_upper_case_globals \
        -A non_camel_case_types


test:
    RUSTFLAGS="-Awarnings" cargo test --target=x86_64-unknown-linux-gnu \
        --no-default-features
