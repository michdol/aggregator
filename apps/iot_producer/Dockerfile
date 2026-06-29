ARG RUST_VERSION=1.96.0
ARG APP_NAME=iot_producer
FROM rust:${RUST_VERSION}-slim-bullseye AS build
ARG APP_NAME
WORKDIR /app

COPY . /app
RUN cargo build --locked --release && cp ./target/release/$APP_NAME /bin/server

FROM debian:bullseye-slim AS final

# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/develop/develop-images/dockerfile_best-practices/   #user
ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

# Copy the executable from the "build" stage.
COPY --from=build /bin/server /bin/

# Expose the port that the application listens on.
EXPOSE 8000

# What the container should run when it is started.
CMD ["/bin/server"]
