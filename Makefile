test_disassembler:build
	frontend-piston/target/debug/frontend-piston --disassemble "res/DMG_ROM.bin"

test_cpu:build
	frontend-piston/target/debug/frontend-piston "res/cpu_instrs/cpu_instrs.gb"

test_boot:build
	frontend-piston/target/debug/frontend-piston -b "res/DMG_ROM.bin" "res/Tetris.gb"

test_game:build
	frontend-piston/target/debug/frontend-piston "res/Tetris.gb"

test_opus5:build
	frontend-piston/target/debug/frontend-piston "res/opus5.gb"

test_debugger:build
	frontend-piston/target/debug/frontend-piston "res/cpu_instrs/cpu_instrs.gb" --debug

build:
	@#cargo rustc --lib -- -Z incremental=/tmp/gameboy-rs
	cargo build
	cargo build --manifest-path ./frontend-piston/Cargo.toml

run:build
	frontend-piston/target/debug/frontend-piston $(filter-out $@,$(MAKECMDGOALS))
