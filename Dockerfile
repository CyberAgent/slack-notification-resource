FROM watawuwu/rust:stable AS rust-builder

ADD Makefile .
ADD *mk .
ADD Cargo.toml .
ADD Cargo.lock .

RUN mkdir -p src/bin
RUN echo 'fn main(){}' >  src/bin/in.rs
RUN echo 'fn main(){}' >  src/bin/out.rs
RUN echo 'fn main(){}' >  src/bin/check.rs

RUN make build BUILD_OPTIONS=--release

ADD benches benches
ADD tests tests
ADD src src

RUN make build BUILD_OPTIONS=--release

FROM watawuwu/openssl:1.0.2

RUN mkdir -p /opt/assets
COPY --from=rust-builder /root/src/target/x86_64-unknown-linux-musl/release/in /opt/assets
COPY --from=rust-builder /root/src/target/x86_64-unknown-linux-musl/release/out /opt/assets
COPY --from=rust-builder /root/src/target/x86_64-unknown-linux-musl/release/check /opt/assets




