NAME = matrix


all: $(NAME)

$(NAME):
	cargo build

clippy:
	cargo clippy -- -D warnings

test:
	cargo test
