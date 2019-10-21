FROM rustlang/rust:nightly-stretch as build

WORKDIR /app

COPY . /app

RUN cargo build --release
RUN ln -s /app/target/release/upload_images /usr/bin/

CMD [ "upload_images" ]
