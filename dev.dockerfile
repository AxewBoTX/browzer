FROM docker.io/rust:1.77.0

# set the working directory
WORKDIR /usr/src/app

# copy all the project files onto the working directory
COPY . .

# install needed dependencies
RUN cargo check && \
	cargo build

# run the tail command to keep the container running
CMD ["tail", "-f", "/dev/null"]
