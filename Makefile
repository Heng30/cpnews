#!/bin/bash

build-evn=
run-evn=RUST_LOG=error,warn,info,debug,sqlx=off,reqwest=off,wgpu_core=off,naga=off,wgpu_hal=off,winit=off

all: build-android-lib build-android-app

build-android-lib:
	$(build-evn) cargo ndk -t arm64-v8a -o cpnews/app/src/main/jniLibs/  build --release

build-android-app: build-android-lib
	cd ./cpnews && ./gradlew build

install: build-android-app
	cd ./cpnews && ./gradlew installDebug

install-release: build-android-app
	cd ./cpnews && ./gradlew installRelease

run:
	$(build-evn) $(run-evn) cargo run --bin cpnews --features=desktop

test:
	$(build-evn) $(run-evn) cargo test -- --nocapture

clippy:
	cargo clippy

clean-incremental:
	rm -rf ./target/debug/incremental/*

clean:
	cargo clean
