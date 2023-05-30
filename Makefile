prepare:
	cargo sqlx prepare -- --lib

init_db:
	./scripts/init_db.sh

.PHONY: migrations
migrations:
	SKIP_DOCKER=true ./scripts/init_db.sh

build:
	docker build --tag zero2prod --file Dockerfile .
	
run:
	docker run -p 8000:8000 --network=host zero2prod

tidy:
	cargo +nightly udeps