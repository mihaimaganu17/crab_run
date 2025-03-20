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
		debian_crab

debian-run-amd64:
	docker run \
		-it --rm \
		--platform linux/amd64 \
		-v ./chapter1:/home/crab/chapter1 \
		debian_crab_amd64

IMAGES = $(shell docker images -q)

docker-clean-all:
	docker system prune -f && docker rmi $(IMAGES)


