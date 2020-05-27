FROM rustlang/rust:nightly

RUN apt update -y && apt upgrade -y

RUN curl -sL https://deb.nodesource.com/setup_12.x | bash -

RUN apt -y install nodejs

WORKDIR /app

COPY . .

# TODO: run node as well to generate static assets.

RUN cargo build

CMD ./target/debug/rocket_practice
