install_system: create_user_dir
	cp build/nsod_system /usr/bin
	ln -sf /usr/bin/nsod_system /usr/bin/nsod

	mkdir -p /usr/lib/nsod
	cp build/libnsod_open_hook.so /usr/lib/nsod

	echo "system install finished"

install_user: create_user_dir
	mkdir -p ~/.nsod/bin
	cp build/nsod_user ~/.nsod/bin
	ln -sf ~/.nsod/bin/nsod_user ~/.nsod/bin/nsod

	mkdir -p ~/.nsod/lib
	cp build/libnsod_open_hook.so ~/.nsod/lib

	echo "user install finished";


build: build/nsod_system build/nsod_user build/libnsod_open_hook.so src/cfg_lib.rs src/cfg_ui.rs src/cmd.rs src/lib.rs src/main.rs src/nsod_cfg.rs src/route.rs src/route.rs src/run.rs src/ui.rs Cargo.toml
	echo "build finished"

build_system: build/nsod_system build/libnsod_open_hook.so src/cfg_lib.rs src/cfg_ui.rs src/cmd.rs src/lib.rs src/main.rs src/nsod_cfg.rs src/route.rs src/route.rs src/run.rs src/ui.rs Cargo.toml
	echo "build for system finished"

build_user: build/nsod_user build/libnsod_open_hook.so src/cfg_lib.rs src/cfg_ui.rs src/cmd.rs src/lib.rs src/main.rs src/nsod_cfg.rs src/route.rs src/route.rs src/run.rs src/ui.rs Cargo.toml
	echo "build for user finished"


package: build
	cp -r ./build ./nsod_pkg/build
	cp -r ./ui ./nsod_pkg/ui
	tar -czf nsod_pkg.tar.gz ./nsod_pkg

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
	cp cfg_install/user.rs src/cfg_install.rs

install_cfg_system: cfg_install/system.rs
	cp cfg_install/system.rs src/cfg_install.rs

fake:
	echo "triggered fake target"

create_user_dir:
	mkdir -p ~/.nsod/cfg
	cp -r ui ~/.nsod/ui