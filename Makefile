.PHONY: all test clean images
.DEFAULT_GOAL := all

# Boiler plate for bulding Docker containers.
# All this must go at top of file I'm afraid.
IMAGE_PREFIX := quay.io/weaveworks
IMAGE_TAG := $(shell ./tools/image-tag)
UPTODATE := .uptodate
BUILD_IMAGE := deployer-build

EXE := target/x86_64-unknown-linux-musl/release/deployer
RUST_FILES := $(shell find . -name '*.rs')

# Get a list of directories containing Dockerfiles
DOCKERFILES := $(shell find . -type f -name Dockerfile ! -path "./tools/*" ! -path "./vendor/*")
UPTODATE_FILES := $(patsubst %/Dockerfile,%/$(UPTODATE),$(DOCKERFILES))

SUDO := $(shell docker info >/dev/null 2>&1 || echo "sudo -E")
BUILD_IN_CONTAINER := true
RM := --rm

all: $(UPTODATE_FILES)

# Building Docker images is now automated. The convention is every directory
# with a Dockerfile in it builds an image calls quay.io/weaveworks/<dirname>.
# Dependencies (i.e. things that go in the image) still need to be explicitly
# declared.
%/$(UPTODATE): %/Dockerfile
	$(SUDO) docker build -t $(IMAGE_PREFIX)/$(shell basename $(@D)) $(@D)/
	$(SUDO) docker tag $(IMAGE_PREFIX)/$(shell basename $(@D)) $(IMAGE_PREFIX)/$(shell basename $(@D)):$(IMAGE_TAG)
	touch $@

deployer/$(EXE): $(EXE)
	cp $(EXE) deployer/

deployer/$(UPTODATE): deployer/$(EXE)

DOCKER_IMAGE_DIRS := $(patsubst %/Dockerfile,%,$(DOCKERFILES))
IMAGE_NAMES := $(foreach dir,$(DOCKER_IMAGE_DIRS),$(patsubst %,$(IMAGE_PREFIX)/%,$(shell basename $(dir))))
images:
	$(info $(IMAGE_NAMES))
	@echo > /dev/null


# TODO: We can probably fetch the dependencies and build them as a separate
# image layer, updating that when Cargo.lock is changed.
# https://github.com/rust-lang/cargo/issues/1891#issuecomment-279781302
# describes one way to do this.

ifeq ($(BUILD_IN_CONTAINER),true)

$(EXE) test: $(BUILD_IMAGE)/$(UPTODATE)
	$(SUDO) docker run $(RM) -ti \
		-v $(shell pwd):/src \
		$(IMAGE_PREFIX)/$(BUILD_IMAGE) $@

else

$(EXE): $(BUILD_IMAGE)/$(UPTODATE) $(RUST_FILES) Cargo.lock
	cargo build --release --target=x86_64-unknown-linux-musl

test: $(BUILD_IMAGE)/$(UPTODATE)
	cargo test

endif

clean:
	$(SUDO) docker rmi $(IMAGE_NAMES) >/dev/null 2>&1 || true
	rm -rf $(UPTODATE_FILES) $(EXE)
	cargo clean
