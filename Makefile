test_disassembler:build
	target/debug/gameboy-emu --disassemble "res/DMG_ROM.bin"

test_cpu:build
	target/debug/gameboy-emu "res/cpu_instrs/cpu_instrs.gb"

test_boot:build
	target/debug/gameboy-emu -b "res/DMG_ROM.bin" "res/Tetris.gb"

test_game:build
	target/debug/gameboy-emu "res/Tetris.gb"

test_opus5:build
	target/debug/gameboy-emu "res/opus5.gb"

test_debugger:build
	target/debug/gameboy-emu "res/cpu_instrs/cpu_instrs.gb" --debug

build:
	cargo rustc --lib -- -Z incremental=/tmp/gameboy-rs

run:build
	target/debug/gameboy-emu $(filter-out $@,$(MAKECMDGOALS))
