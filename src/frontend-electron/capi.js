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

function Capi() {
  this.gb = lib.gameboy_new();
  this.vid_buffer = new ImageBuffer(160*144*4);
  for (var i = 0; i < 160*144*4; i++) {
    this.vid_buffer[i] = 0xff;
  }
}

Capi.prototype.drop = function drop() {
  lib.gameboy_drop(this.gb);
};

Capi.prototype.step = function step() {
  lib.gameboy_step(this.gb);
};

Capi.prototype.video_data = function video_data() {
  lib.gameboy_video_data(this.gb, this.vid_buffer);
  return this.vid_buffer;
};

module.exports = new Capi();