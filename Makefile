# This is the most general Makefile for building and installing NSOD on the local system.

install_system: 
	cp build/nsod_system /usr/bin
	ln -sf /usr/bin/nsod_system /usr/bin/nsod

	mkdir -p /usr/lib/nsod

	cp -r ui /usr/lib/nsod
	cp build/libnsod_open_hook.so /usr/lib/nsod

	mkdir -p /etc/nsod

	echo "system install finished"

install_user: 
	mkdir -p ~/.nsod/cfg
	mkdir -p ~/.nsod/bin
	mkdir -p ~/.nsod/lib

	cp build/nsod_user ~/.nsod/bin
	ln -sf ~/.nsod/bin/nsod_user ~/.nsod/bin/nsod

	cp build/libnsod_open_hook.so ~/.nsod/lib

	cp -r ui ~/.nsod/lib
	
	echo "user install finished";


uninstall_system:
	rm /usr/bin/nsod
	rm /usr/bin/nsod_system

	rm -r /usr/lib/nsod
	rm -r /etc/nsod

uninstall_user:
	rm -r ~/.nsod

build: build/nsod_system build/nsod_user build/libnsod_open_hook.so
	echo "build finished"

build_system: build/nsod_system build/libnsod_open_hook.so
	echo "build for system finished"

build_user: build/nsod_user build/libnsod_open_hook.so
	echo "build for user finished"


build/libnsod_open_hook.so: build/libnsod_rust.a c/nsod_open_hook.c c/nsod_rust.h
	gcc -shared -fPIC -I c c/nsod_open_hook.c -o build/libnsod_open_hook.so -L./build -l:libnsod_rust.a


build/nsod_system: install_cfg_system

	echo "cargo building for system"
	
	cargo build --release

	cp target/release/libnsod_rust.a build/libnsod_rust.a
	cp target/release/nsod build/nsod_system


build/nsod_user: install_cfg_user 

	echo "cargo building for user"

	cargo build --release

	cp target/release/libnsod_rust.a build/libnsod_rust.a
	cp target/release/nsod build/nsod_user


install_cfg_user: cfg_install/user.rs
	cp cfg_install/user.rs src/cfg_install_path.rs

install_cfg_system: cfg_install/system.rs
	cp cfg_install/system.rs src/cfg_install_path.rs