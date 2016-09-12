var ffi = require('ffi');
var path = require('path');
var ref = require('ref');
var ArrayType = require('ref-array')

var Gameboy = ref.types.void;
var GameboyPtr = ref.refType(Gameboy);

var lib = ffi.Library(path.join(__dirname, '../../target/debug/libgameboy.so'), {
  gb_new: [GameboyPtr, ["char*"]],
  gb_run_threaded: ["void", [GameboyPtr]],
  gb_set_button: ["void", [GameboyPtr, "uint8", "bool"]],
  gb_updated_frame: ["int", [GameboyPtr, ref.refType(ref.types.char)]],
  gb_drop: ["void", [GameboyPtr]],
});

function Capi(cart_path) {
  var buffer = new Buffer(cart_path.length);
  buffer.write(cart_path, 0, "utf-8");
  this.gb = lib.gb_new(buffer);

  this.vid_buffer = new Buffer(160*144*4);
}

Capi.prototype.run_threaded = function run_threaded() {
  lib.gb_run_threaded(this.gb);
};

Capi.prototype.set_button = function set_button(btn, pressed) {
  var btn_i = 0;
  switch (btn) {
    case "right": case 68: btn_i = 1; break;
    case "left": case 65: btn_i = 2; break;
    case "up": case 87: btn_i = 4; break;
    case "down": case 83: btn_i = 8; break;
    case "a": case 32: btn_i = 16; break;
    case "b": case 17: btn_i = 32; break;
    case "select": case 16: btn_i = 64; break;
    case "start": case 13: btn_i = 128; break;
  }

  lib.gb_set_button(this.gb, btn_i, pressed);
};

Capi.prototype.updated_frame = function updated_frame() {
  if (lib.gb_updated_frame(this.gb, this.vid_buffer) == 1) {
    return this.vid_buffer;
  }
  return null;
};

Capi.prototype.drop = function drop() {
  lib.gb_drop(this.gb);
};

module.exports = Capi;
