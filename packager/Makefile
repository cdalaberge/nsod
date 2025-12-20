# This Makefile is used to cross-compile and package NSOD for distribution to various platforms.
# The 'nsod' subdirectory holds everything that is included for the final package

final_pkg:
	cp -r ../ui ./nsod
	cp -r ../c ./nsod
	cp ./Make_installer ./nsod/Makefile
	cp ../LICENSE ./nsod/LICENSE
	tar -czf "./$(pkg_name).tar.gz" "./nsod"



arm32: cargo_target = armv7-unknown-linux-gnueabihf
arm32: target_glibc_version = .2.31
arm32: zcc_target = arm-linux-gnueabihf
arm32: pkg_name = nsod_arm32
arm32: build final_pkg
	echo "created package for arm32"


build: nsod/nsod_system nsod/nsod_user nsod/libnsod_open_hook.so
	echo "build finished"


nsod/libnsod_open_hook.so: tmp/libnsod_rust.a ../c/nsod_open_hook.c ../c/nsod_rust.h
	zig cc -target $(zcc_target)$(target_glibc_version) -shared -fPIC -I ../c ../c/nsod_open_hook.c -o ./nsod/libnsod_open_hook.so -L./tmp -lnsod_rust


nsod/nsod_system: install_cfg_system

	echo "cargo building for system"
	
	cd ..; cargo zigbuild --release --target $(cargo_target)$(target_glibc_version)

	cp ../target/$(cargo_target)/release/libnsod_rust.a tmp/libnsod_rust.a
	cp ../target/$(cargo_target)/release/nsod nsod/nsod_system


nsod/nsod_user: install_cfg_user 

	echo "cargo building for user"

	cd ..; cargo zigbuild --release --target $(cargo_target)$(target_glibc_version)

	cp ../target/$(cargo_target)/release/libnsod_rust.a nsod/libnsod_rust.a
	cp ../target/$(cargo_target)/release/nsod nsod/nsod_user


install_cfg_user: ../cfg_install/user.rs
	cp ../cfg_install/user.rs ../src/cfg_install_path.rs

install_cfg_system: ../cfg_install/system.rs
	cp ../cfg_install/system.rs ../src/cfg_install_path.rs