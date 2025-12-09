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

create_user_dir:
	mkdir -p ~/.nsod/cfg
	cp -r ui ~/.nsod/ui