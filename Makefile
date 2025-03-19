debian-build:
	docker build \
		-f images/Dockerfile.debian \
		--tag "debian_crab" \
		.

debian-run:
	docker run \
		-it --rm \
		debian_crab

IMAGES = $(shell docker images -q)

docker-clean-all:
	docker system prune -f && docker rmi $(IMAGES)


