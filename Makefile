.PHONY: build up down

install:
	cargo install --path .

release-build:
	cargo build --release --target x86_64-unknown-linux-musl
	docker build -t rusty_auth_release .

release-run:
	docker run -it -p 3000:3000 rusty_auth_release

build:
	cargo build --target=x86_64-unknown-linux-musl
	docker-compose build

up:
	docker-compose up backend && docker-compose rm -fsv

down:
	docker-compose down --volumes

test:
	docker compose  -f docker-compose.test.yml run test && docker-compose rm -fsv

binary:
	cargo build --target=x86_64-unknown-linux-musl

clean:
	docker rm --force `docker ps -qa`