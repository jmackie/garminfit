SHELL = /usr/bin/env bash
THIS_FILE := $(abspath $(lastword $(MAKEFILE_LIST)))
FIT_SDK ?= sdk/FitSDKRelease_20.66.00
FIT_SDK_PROFILE ?= $(FIT_SDK)/Profile.xlsx
# Attempt to get the FIT_SDK_VERSION by splitting on the final underscore
FIT_SDK_VERSION ?= $(shell echo $(FIT_SDK) | awk -F_ '{print $$NF}')
CARGO = cargo --color always


.PHONY: all
all: ## Run the build target (default)
all: build


.PHONY: build
build: ## Build the crate
build:
	@$(CARGO) build


.PHONY: check
check: ## Build and test the crate
check: build test


.PHONY: profile-modules
sdk-modules: ## Generate rust modules from the FIT SDK Profile.xlsx (as specified by $FIT_SDK)
sdk-modules:
	@$(CARGO) run --package garminfit-profile-gen -- \
		--sdk-version $(FIT_SDK_VERSION) \
		--messages-module messages.rs \
		--types-module types.rs \
		$(FIT_SDK_PROFILE) \
		--output-dir $$(dirname $(THIS_FILE))/src/profile

	@# Format the generated code
	@$(MAKE) -f $(THIS_FILE) --no-print-directory format


.PHONY: test
test: ## Run the test suite (TODO). You should probably `make testdata` first
test: build
	@$(CARGO) test


.PHONY: testdata
testdata: ## Populate the testdata directory from testdata/sources.txt
testdata:
	@# lol gross
	@while read -r line; do \
    	if [[ $$line == \#* ]] || [[ -z "$${line// /}" ]]; then continue; fi; \
		echo $$line; \
    	curl -s -L -o testdata/$${line%=*} $${line#*=}; \
	done <testdata/sources.txt


.PHONY: bench
bench: ## Run the benchmarks
bench:
	@$(CARGO) bench


.PHONY: format
format: ## Format all the code
format:
	@$(CARGO) fmt --all


.PHONY: doc
doc: ## Generate local docs
doc:
	@$(CARGO) doc --package garminfit # --open


.PHONY: clean
clean: ## Cleanup build stuff
clean:
	@$(CARGO) clean

	@# Delete all the test files as well
	@find testdata -type f -not -name sources.txt -exec rm {} \;


# Self-documenting Makefile
# https://gist.github.com/prwhite/8168133
.PHONY: help
help:
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) \
		| awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36;1m%-20s\033[0m %s\n", $$1, $$2}'
