### The Builder Stage Compiles the code
FROM rust:buster as builder

RUN rustup default nightly-2021-11-07 && \
        rustup target add wasm32-unknown-unknown --toolchain nightly-2021-11-07

RUN apt-get update && \
        apt-get dist-upgrade -y -o Dpkg::Options::="--force-confold" && \
        apt-get install -y cmake pkg-config libssl-dev git clang libclang-dev

# Clone the template code and checkout the right commit
RUN git clone --recursive https://github.com/0xmostima/subtensor-parachain.git
WORKDIR /subtensor-parachain
RUN echo $(ls /)
#RUN git checkout 9506b93

# Build the Parachain Collator node
RUN CARGO_PROFILE_RELEASE_LTO=true RUSTFLAGS="-C codegen-units=1" cargo build --features with-nakamoto-runtime --release

### The final stage just copies binary and chainspecs

# Choose the base image. Same on used in main Polkadot repo
FROM debian:stretch-slim

# Copy the node into the image
COPY --from=builder /subtensor-parachain/target/release/parachain-collator .

RUN ./parachain-collator build-spec --disable-default-bootnode > /nakamoto
## Copy chainspecs into the image
#COPY shared/chainspecs/rococo-custom-2-raw.json .
#COPY shared/chainspecs/rococo-custom-3-raw.json .
#COPY shared/chainspecs/rococo-custom-4-raw.json .

# Open default ports. User is responsible for re-mapping these
# or using host or overlay networking.
EXPOSE 30333 9933 9944

ENTRYPOINT ["./parachain-collator"]