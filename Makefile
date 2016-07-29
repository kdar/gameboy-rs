test_disassembler:
	cargo run -- --disassemble "res\DMG_ROM.bin"

test_cpu:
	cargo run -- -b "res\DMG_ROM.bin" "res\Tetris Blast (USA, Europe).gb"

test_debugger:
	cargo run -- -b "res\DMG_ROM.bin" "res\Tetris Blast (USA, Europe).gb" --debug
