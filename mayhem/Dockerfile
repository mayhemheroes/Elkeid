# Build Stage
FROM rustlang/rust:nightly AS builder

RUN cargo install cargo-fuzz

## Add source code to the build stage.
ADD . /src
WORKDIR /src

RUN echo building instrumented harnesses && \
    bash -c "pushd plugins/journal_watcher/fuzz && cargo +nightly fuzz build && popd" && \
    mv plugins/journal_watcher/fuzz/target/x86_64-unknown-linux-gnu/release/parse_event /parse_event && \
    echo done

RUN echo building non-instrumented harnesses && \
    export RUSTFLAGS="--cfg fuzzing -Clink-dead-code -Cdebug-assertions -C codegen-units=1" && \
    bash -c "pushd plugins/journal_watcher/fuzz && cargo +nightly build --release && popd" && \
    mv plugins/journal_watcher/fuzz/target/release/parse_event /parse_event_no_inst && \
    echo done

# Package Stage
FROM rustlang/rust:nightly

COPY --from=builder /parse_event /parse_event_no_inst /
