# Author: FL03 <jo3mccain@icloud.com>
ARG RUST_VERSION=latest

FROM rust:${RUST_VERSION} AS builder-base
# update and upgrade the system
RUN apt-get update -y && \
    apt-get upgrade -y
# STAGE 2: build
FROM builder-base AS builder
# declare some environment variables
ENV RUST_BACKTRACE=1 \
    CARGO_NET_GIT_FETCH_WITH_CLI=true \
    CARGO_HOME=/usr/local/cargo
# setup the working directory
WORKDIR /app
# copy the source code
ADD . .
# build the project
RUN --mount=type=cache,target=/workspace/target/ \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --locked --release --features full --workspace
# STAGE 3: create the runtime image
FROM debian:bookworm-slim AS runner-base
# update and upgrade the system packages
RUN apt-get update -y && \
    apt-get upgrade -y
# create a group and add a non-root user
RUN groupadd -g 10001 agroup && \
    useradd -m -u 10001 -g agroup auser
# switch to the new user
USER auser
# copy the executable(s) to the system
COPY --from=builder --chown=auser:agroup /app/target/release/scsys /usr/local/bin/scsys
# copy the configuration directory
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/.config /opt/scsys/.config
# copy any additional configuration files
COPY --from=builder --chown=auser:agroup --chmod=755 --link /app/*.config.toml* /opt/scsys/.config/*.config.toml*
# set the permissions
RUN chmod +x /usr/local/bin/scsys && \
    chmod +x /opt/scsys/.config && \
    chown auser /usr/local/bin/scsys && \
    chown -R auser /opt/scsys
# STAGE 4: use the configured runtime base to create the final image
FROM runner-base AS runner
# Set the environment variables
ENV APP_MODE=release \
    RUST_LOG="scsys=debug,info" \
    SCSYS_HOST="0.0.0.0" \
    SCSYS_PORT=8080 \
    SCSYS_CONFIG="/opt/scsys/.config" \
    SCSYS_WORKDIR="/opt/scsys"
# set the working directory
WORKDIR /opt/scsys
# expose the port
EXPOSE ${SCSYS_PORT}
# set the entrypoint
ENTRYPOINT ["scsys"]
