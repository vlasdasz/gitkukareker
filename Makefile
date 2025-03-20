
include build/common.mk

all:
	order
	make ios

lint:
	cargo clippy \
      -- \
      \
      -W clippy::all \
      -W clippy::pedantic \
      \
      -A clippy::explicit_deref_methods \
      -A clippy::needless_pass_by_value \
      -A clippy::missing_panics_doc \
      -A clippy::module_inception \
      -A clippy::struct_field_names \
      \
      -D warnings

.PHONY: mobile
