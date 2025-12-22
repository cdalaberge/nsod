# This is the most general Makefile for building and installing NSOD on the local system.

system_install: 
	cp build/nsod_system /usr/bin
	ln -sf /usr/bin/nsod_system /usr/bin/nsod

	mkdir -p /usr/lib/nsod

	cp -r ui /usr/lib/nsod
	cp build/libnsod_open_hook.so /usr/lib/nsod

	mkdir -p /etc/nsod

	echo "system install finished"

user_install: 
	mkdir -p "$(HOME)/.nsod/cfg"
	mkdir -p "$(HOME)/.nsod/bin"
	mkdir -p "$(HOME)/.nsod/lib"

	cp build/nsod_user "$(HOME)/.nsod/bin"
	ln -sf "$(HOME)/.nsod/bin/nsod_user" "$(HOME)/.nsod/bin/nsod"

	cp build/libnsod_open_hook.so "$(HOME)/.nsod/lib"

	cp -r ui "$(HOME)/.nsod/lib"
	
	echo "user install finished";


system_uninstall:
	rm -f /usr/bin/nsod
	rm -f /usr/bin/nsod_system

	rm -rf /usr/lib/nsod
	rm -rf /etc/nsod

user_uninstall:
	rm -rf "$(HOME)/.nsod"

build: build/nsod_system build/nsod_user build/libnsod_open_hook.so
	echo "build finished"

system_build: build/nsod_system build/libnsod_open_hook.so
	echo "build for system finished"

user_build: build/nsod_user build/libnsod_open_hook.so
	echo "build for user finished"


# linking dl isn't needed on most modern systems but helps with compatibility.
build/libnsod_open_hook.so: build/libnsod_rust.a c/nsod_open_hook.c c/nsod_rust.h
	gcc -shared -fPIC -I c c/nsod_open_hook.c -o build/libnsod_open_hook.so -L./build -l:libnsod_rust.a -ldl


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