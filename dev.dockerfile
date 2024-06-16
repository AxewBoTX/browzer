FROM docker.io/rust:1.77.0

# set the working directory
WORKDIR /usr/src/app

# copy all the project files onto the working directory
COPY . .

# basic container setup (according to my liking)
RUN apt-get update && \
	apt-get install -y curl && \
	apt-get install -y tmux psmisc && \
	bash -c "echo 'PATH="/usr/local/cargo/bin:$PATH"' >> ~/.bashrc" && \
	bash -c "source ~/.bashrc"

# install needed dependencies
RUN cargo install cargo-watch && \
	cargo check && \
	cargo build

# run the tail command to keep the container running
CMD ["tail", "-f", "/dev/null"]
