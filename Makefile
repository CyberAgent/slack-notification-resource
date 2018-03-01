# Setting
#===============================================================
SHELL := /bin/bash
OS := $(shell uname)

# Const
#===============================================================
name                    := snotif
override_docker_conifig := docker-compose.override.yml
override_make_file      := override.mk
lib_name                := $(name)

# Option
#===============================================================
CARGO_OPTIONS   := +stable
BUILD_OPTIONS   :=
ARGS            :=
LOG_LEVEL       := debug
LOG             := $(name)=$(LOG_LEVEL)
INPUT           :=
RUN_TYPE        := in # (in|out|Chuck)
IMAGE_REPO      := watawuwu/slack-notification-resource

# Task
#===============================================================
deps:
	rustup update
	rustup component add rustfmt-preview
	cargo +nightly install --force clippy

test: .target
	RUST_LOG=$(LOG) \
	RUST_BACKTRACE=1 \
	cargo $(CARGO_OPTIONS) test \
	  $(BUILD_OPTIONS) \
	  --target $(target) \
	  -- --nocapture $(ARGS)
run: .target
	RUST_LOG=$(LOG),in=$(LOG_LEVEL) \
	cargo $(CARGO_OPTIONS) run \
	  --bin $(RUN_TYPE) \
	  $(BUILD_OPTIONS) \
	  --target $(target) \
	  -- ./tmp <<<$$(cat $(INPUT))
build: .target
	cargo $(CARGO_OPTIONS) build \
	  $(BUILD_OPTIONS) \
	  --target $(target)
fmt:
	cargo +stable fmt
	cargo +nightly clippy
check:
	cargo $(CARGO_OPTIONS) check
clean:
	cargo $(CARGO_OPTIONS) clean
update:
	cargo $(CARGO_OPTIONS) update
	cargo $(CARGO_OPTIONS) build


# Docker for local development
#-------------------------------
## Ship the Docker image
docker-integration: docker-build docker-push docker-clean
## Building docker image.
docker-build:
	$(eval tag := $(shell date +'%Y-%m-%dT%H%M%S'))
	docker build  -t $(IMAGE_REPO):$(tag) .
	docker tag $(IMAGE_REPO):$(tag) $(IMAGE_REPO):latest
## Push docker image.
docker-push: .image-tag
	docker push $(IMAGE_REPO):latest
	docker push $(IMAGE_REPO):$(tag)
## Remove the image for two generations, delete the remaining images.
docker-clean:
	docker images --format "{{.Repository}}\t{{.CreatedAt}}\t{{.Tag}}" $(IMAGE_REPO) | \
	  sort -r -k2,3 | \
	  awk -F"\t" '$$3 != "latest" && NR > 3 {print $$1":"$$3}' | \
	  xargs -n 1 docker rmi -f


# Internal Task
#===============================================================
.check-local-docker-compose:
ifeq ("$(wildcard $(local_docker_conifig))","")
	$(error Please create $(local_docker_conifig) )
endif

.target:
ifeq ($(OS),Darwin)
	$(eval target = x86_64-apple-darwin)
endif
ifeq ($(OS),Linux)
	$(eval target = x86_64-unknown-linux-musl)
endif
	$(eval target_dir = target/$(target)/release)

.image-tag:
	$(eval tag := $(shell docker images --format "{{.ID}}\t{{.CreatedAt}}\t{{.Tag}}" $(IMAGE_REPO) | \
	  sort -r -k2,3 | \
	  awk -F"\t" '$$3 != "latest" {print $$3}' | \
	  head -n 1 ))

# Override task
#===============================================================
ifneq ("$(wildcard $(override_make_file))","")
include $(override_make_file)
endif


.PHONY        : deps test run build release format clean update
.DEFAULT_GOAL := build
