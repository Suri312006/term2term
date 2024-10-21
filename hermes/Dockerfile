FROM rust:1.80.1-bullseye AS build
RUN apt-get update && apt-get install -y protobuf-compiler  && apt-get clean

# RUN wget https://github.com/protocolbuffers/protobuf/releases/download/v3.21.12/protoc-3.21.12-linux-x86_64.zip
# RUN unzip protoc-3.21.12-linux-x86_64.zip -d $HOME/.local
# RUN export PATH=$PATH:$HOME/.local/bin
# create a new empty shell project
RUN USER=root cargo new --bin t2t-server
WORKDIR /t2t-server

# copy over your manifests
COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release
RUN rm src/*.rs

# this build step will cache your dependencies
COPY ./src ./src
COPY ./Cargo.toml ./Cargo.toml
COPY ./build.rs ./build.rs
COPY ./proto ../proto
# copy your source tree
# build for release
RUN cargo build --release

# our final base
FROM debian:bullseye-slim

# copy the build artifact from the build stage
COPY --from=build /t2t-server/target/release/t2t-server .

EXPOSE 50051
# set the startup command to run your binary
CMD ["./t2t-server"]
# FROM rust:1.77-buster AS build
#
# # Install protobuf-compiler
# RUN apt-get update && apt-get install -y protobuf-compiler && apt-get clean
#
# # Set the working directory
# WORKDIR /t2t-server
#
# # Copy the manifest files
# COPY ./t2t-server/Cargo.toml ./t2t-server/Cargo.lock ./
# COPY ./t2t-server/build.rs ./build.rs
#
# # Create an empty project to cache dependencies
# RUN mkdir src
# RUN echo "fn main() {}" > src/main.rs
# RUN cargo build --release
# RUN rm -rf src
#
# # Copy the source code and proto files
# COPY ./t2t-server/src ./src
# COPY ./proto ../proto
#
# # Build the project
# RUN cargo build --release
#
# # Create the final image
# FROM debian:buster-slim
#
# # Copy the build artifact from the build stage
# COPY --from=build /t2t-server/target/release/t2t-server .
#
# # Expose the required port
# EXPOSE 50051
#
# # Set the startup command to run your binary
# CMD ["./t2t-server"]
