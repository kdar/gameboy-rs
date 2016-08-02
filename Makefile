test_disassembler:
	cargo run -- --disassemble "res\DMG_ROM.bin"

test_cpu:
	cargo run -- -b "res\DMG_ROM.bin" "res\cpu_instrs\cpu_instrs.gb"

test_boot:
	cargo run -- -b "res\DMG_ROM.bin" "res\opus5.gb"

test_debugger:
	cargo run -- -b "res\DMG_ROM.bin" "res\opus5.gb" --debug

test_game:
	cargo run -- -b "res\DMG_ROM.bin" "res\Tetris.gb" --debug
