debian-build:
	docker build \
		-f images/Dockerfile.debian \
		--tag "debian" \
		.

debian-run:
	docker run \
		-it --rm \
		debian

IMAGES = $(shell docker images -q)

docker-clean-all:
	docker system prune -f && docker rmi $(IMAGES)


