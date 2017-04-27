gameboy-rs
==========

A classic Gameboy emulator written in Rust.

Passes all the Blargg's CPU behavior tests.

![Screenshot1](https://github.com/kdar/gameboy-rs/raw/master/doc/screenshot1.png)

![Screenshot2](https://github.com/kdar/gameboy-rs/raw/master/doc/screenshot2.png)

![Screenshot3](https://github.com/kdar/gameboy-rs/raw/master/doc/screenshot3.png)

![Screenshot4](https://github.com/kdar/gameboy-rs/raw/master/doc/screenshot4.png)

### Some unique features

#### Tests

I provide a whole slew of CPU bahavior tests in `testdata/` that will allow you to test your own emulator. All you have to do is read in `testdata/cpu.json` or `testdata/cpu.yaml` and convert it to your internal structures in your own emulator.

You then can set the `pre` state and then compare to the `post` state. Check `test_runner_json()` in `lib/cpu.rs` for an example.

#### An electron GUI

I have an expiermental electron GUI, but still not sure if it's fast enough for the job. It works though.

### Rust version

This is the last known rust version it works with: `rustc 1.12.0-nightly (545a3a94f 2016-08-04)`

### TODO

1) Instruction timings
2) Memory timings
3) Sound
4) Color gameboy?
5) Optimize
