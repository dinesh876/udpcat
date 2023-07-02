prog :=udpcat

debug ?=

$(info debug is $(debug))

ifdef debug
  release :=
  target :=debug
  extension :=-debug
else
  release :=--release
  target :=release
  extension :=
endif

run-server:
	cargo run -- -m server

run-client:
	caro run -- -m client -r 127.0.0.1:50001 -l 8192

build:
	cargo build $(release)

install:
	cp target/$(target)/$(prog) ~/.local/bin/$(prog)$(extension)

clean:
	rm -rf target utils/target
	rm -rf ~/.local/bin/$(prog)$(extension)

all: clean build install
 
help:
	@echo "usage: make $(prog) [debug=1]"
