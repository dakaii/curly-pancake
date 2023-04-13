.PHONY: build up down

release-build:
	cargo build --release --target x86_64-unknown-linux-musl

build:
	cargo build --target=x86_64-unknown-linux-musl
	docker-compose build

up:
	cargo build --target=x86_64-unknown-linux-musl
	docker-compose up backend && docker-compose rm -fsv

down:
	docker-compose down --volumes

test:
	docker compose -f docker-compose.test.yml run test && docker-compose rm -fsv

binary:
	cargo build --target=x86_64-unknown-linux-musl

clean:
	docker rm --force `docker ps -qa`