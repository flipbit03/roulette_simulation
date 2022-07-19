windows:
	rustup target add x86_64-pc-windows-gnu
	cargo install cross --git https://github.com/cross-rs/cross
	cross build --release --target x86_64-pc-windows-gnu
	cp target/x86_64-pc-windows-gnu/release/roulette_simulation.exe win32_build/
	strip win32_build/roulette_simulation.exe