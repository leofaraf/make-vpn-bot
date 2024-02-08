FROM rust:1.75

COPY ./ ./

CMD [ "cargo", "build", "--release" ]

RUN ./target/release/make-vpn-bot