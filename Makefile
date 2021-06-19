SHELL=/bin/bash

.PHONY := major minor patch

.DEFAULT_GOAL := patch

PKG = ky
RE=[^0-9]*\([0-9]*\)[.]\([0-9]*\)[.]\([0-9]*\)\([0-9A-Za-z-]*\)
version = $(subst v,,$(shell git describe --tags --abbrev=0))
major_ver = $(shell (echo $(version) | sed -e 's#$(RE)#\1#'))
minor_ver = $(shell (echo $(version) | sed -e 's#$(RE)#\2#'))
patch_ver = $(shell (echo $(version) | sed -e 's#$(RE)#\3#'))

define release
	echo "-- Old Tag: $(version)"
	echo "-- New Tag: $(1)"
	echo ""
	echo ">> Updating Cargo.toml"
	sed -i -e 's/^version = .*/version = $(1)/g' Cargo.toml
	echo ">> Updating Cago.lock"
	cargo update -p $(PKG)
	echo ">> Release: $(1)"
	git commit -a -m "Release $(1)"
	git tag v$(1) -m "Release $(1)"
endef

push:
	@echo ">> Push Tags"
	@git push origin --tags
	@echo ">> All mighty push"
	@git push

major:
	@$(call release,"$(shell expr $(major_ver) + 1).0.0")

minor:
	@$(call release,"$(major_ver).$(shell expr $(minor_ver) + 1).0")

patch:
	@$(call release,"$(major_ver).$(minor_ver).$(shell expr $(patch_ver) + 1)")
