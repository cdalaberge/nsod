system: out/libnsod_rust.so out/libnsod_open_hook.so
	cp target/debug/nsod /usr/bin
	
	mkdir -p /usr/lib/nsod
	cp out/libnsod_rust.so /usr/lib/nsod
	cp out/libnsod_open_hook.so /usr/lib/nsod

local: out/libnsod_rust.so out/libnsod_open_hook.so


out/libnsod_open_hook.so: out/libnsod_rust.a c/nsod_open_hook.c c/nsod_rust.h
	gcc -shared -fPIC -I c c/nsod_open_hook.c -o out/libnsod_open_hook.so -L./out -l:libnsod_rust.a

out/libnsod_rust.a: src/cfg_lib.rs src/cfg_ui.rs src/cmd.rs src/lib.rs src/main.rs src/nsod_cfg.rs src/route.rs src/route.rs src/run.rs src/ui.rs Cargo.toml
	cargo build
	cp target/debug/libnsod_rust.a out/libnsod_rust.a

out/libnsod_rust.so: out/libnsod_rust.a
	cp target/debug/libnsod_rust.so out/libnsod_rust.so