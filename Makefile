debian-build:
	docker build \
		-f images/Dockerfile.debian\
		--tag "debian_crab" \
		.

debian-build-amd64:
	docker build \
		-f images/Dockerfile.debian\
		--platform linux/amd64 \
		--tag "debian_crab_amd64" \
		.

debian-run:
	docker run \
		-it --rm \
		-v ./chapter2/naive_hashmap:/home/crab/chapter2/naive_hashmap \
		debian_crab

debian-run-amd64:
	docker run \
		-it --rm \
		--platform linux/amd64 \
		--security-opt seccomp=images/seccomp.json \
		-v ./chapter2/naive_hashmap:/home/crab/chapter2/naive_hashmap \
		debian_crab_amd64

IMAGES = $(shell docker images -q)

docker-clean-all:
	docker system prune -f && docker rmi $(IMAGES)

fuzz_hashmap:
	cargo afl fuzz -i resources/in/ -o resources/out/ target/release/specialized_interpreter


