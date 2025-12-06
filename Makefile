system: install_cfg_system create_user_dir out/libnsod_open_hook.so
	cp target/release/nsod /usr/bin

	mkdir -p /usr/lib/nsod
	cp out/libnsod_open_hook.so /usr/lib/nsod

user: install_cfg_user create_user_dir out/libnsod_open_hook.so
	mkdir -p ~/.nsod/bin
	cp target/release/nsod ~/.nsod/bin

	mkdir -p ~/.nsod/lib
	cp out/libnsod_rust.so ~/.nsod/lib
	cp out/libnsod_open_hook.so ~/.nsod/lib

out/libnsod_open_hook.so: out/libnsod_rust.a c/nsod_open_hook.c c/nsod_rust.h
	gcc -shared -fPIC -I c c/nsod_open_hook.c -o out/libnsod_open_hook.so -L./out -l:libnsod_rust.a

out/libnsod_rust.a: src/cfg_lib.rs src/cfg_ui.rs src/cmd.rs src/lib.rs src/main.rs src/nsod_cfg.rs src/route.rs src/route.rs src/run.rs src/ui.rs src/cfg_install.rs Cargo.toml

	@if [ "$(SUDO_USER)" = "" ]; then\
		echo "cargo building for user";\
		cargo build --release;\
	else\
		echo "cargo building for system";\
		HOME="/home/$(SUDO_USER)" sudo -u "$(SUDO_USER)" "/home/$(SUDO_USER)/.cargo/bin/cargo" build --release;\
	fi

	cp target/release/libnsod_rust.a out/libnsod_rust.a

install_cfg_user: cfg_install/user.rs
	cp cfg_install/user.rs src/cfg_install.rs

install_cfg_system: cfg_install/system.rs
	cp cfg_install/system.rs src/cfg_install.rs

create_user_dir:
	mkdir -p ~/.nsod/cfg
	cp -r ui ~/.nsod/ui