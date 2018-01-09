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
PUSH_TAG        := latest

# Task
#===============================================================
deps:
	rustup update
	cargo +nightly install --force rustfmt-nightly
	cargo +nightly install --force clippy

test: .target
	RUST_LOG=$(LOG) \
	cargo $(CARGO_OPTIONS) test \
	  --lib \
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
format:
	cargo +nightly fmt
	cargo +nightly clippy
clean:
	cargo $(CARGO_OPTIONS) clean
update:
	cargo $(CARGO_OPTIONS) update
	cargo $(CARGO_OPTIONS) build


# Docker for local development
#-------------------------------
## Building docker image.
docker-build:
	$(eval tag := $(shell date +'%Y-%m-%dT%H%M%S'))
	docker build  -t $(IMAGE_REPO):$(tag) .
	docker tag $(IMAGE_REPO):$(tag) $(IMAGE_REPO):latest
## Push docker image.
docker-push: .image-tag
	docker push $(IMAGE_REPO):$(PUSH_TAG)
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


# Override task
#===============================================================
ifneq ("$(wildcard $(override_make_file))","")
include $(override_make_file)
endif


.PHONY        : deps test run build release format clean update
.DEFAULT_GOAL := build
