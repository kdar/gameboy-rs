var ffi = require('ffi');
var path = require('path');
var ref = require('ref');
var ArrayType = require('ref-array')

var Gameboy = ref.types.void;
var GameboyPtr = ref.refType(Gameboy);
var ImageBuffer = ArrayType('uint8');

var lib = ffi.Library(path.join(__dirname, '../../target/debug/libgameboy.so'), {
  gameboy_new: [GameboyPtr, []],
  gameboy_step: ["void", [GameboyPtr]],
  gameboy_video_data: ["void", [GameboyPtr, ImageBuffer]],
  gameboy_drop: ["void", [GameboyPtr]],
});

var gameboy = lib.gameboy_new();

var buffer = new ImageBuffer(160*144*4);
for (var i = 0; i < 160*144*4; i++) {
  buffer[i] = 0xff;
}

// for (var i = 0; i < 0xffffff; i++) {
//   lib.gameboy_step(gameboy);

//   if (i % 1000 == 0) {
//     console.time("vid");
//     lib.gameboy_video_data(gameboy, buffer);
//     console.timeEnd("vid");
//     console.log(buffer.slice(0, 10));
//   }
// }

lib.gameboy_drop(gameboy);
