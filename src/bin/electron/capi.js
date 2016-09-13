var ffi = require('ffi');
var path = require('path');
var ref = require('ref');
var ArrayType = require('ref-array');
var Struct = require('ref-struct');

var Gameboy = ref.types.void;
var GameboyPtr = ref.refType(Gameboy);

var Error = Struct({
  'code': 'int8',
  'error': ArrayType('char'),
});
var ErrorPtr = ref.refType(Error);

Error.prototype.toString = function toString() {
  var string = "";
  for (var x = 0; x < 1024; x++) {
    if (this.error[x] == 0) {
      break;
    }
    string += String.fromCharCode(this.error[x]);
  }

  return string;
};

Error.prototype.hasError = function hasError() {
  return this.code != 0;
};

Error.prototype.ref = function ref() {
  this.code = 0;
  return this.__proto__.__proto__.ref.call(this);
};

var lib = ffi.Library(path.join(__dirname, '../../../target/debug/libgameboy.so'), {
  gb_new: [GameboyPtr, []],
  gb_load_cartridge: ['void', [GameboyPtr, 'char*', ErrorPtr]],
  gb_run_threaded: ['void', [GameboyPtr]],
  gb_set_button: ['void', [GameboyPtr, 'uint8', 'bool']],
  gb_updated_frame: ['int', [GameboyPtr, ref.refType(ref.types.char)]],
  gb_drop: ['void', [GameboyPtr]],
});

function Capi() {
  this.api_error = new Error();
  this.api_error.error = new Buffer(1024);
  this.vid_buffer = new Buffer(160*144*4);

  this.gb = lib.gb_new();
}

Capi.prototype.load_cartridge = function load_cartridge(cart_path) {
  var cart_buffer = new Buffer(cart_path.length);
  cart_buffer.write(cart_path, 0, "utf-8");

  lib.gb_load_cartridge(this.gb, cart_buffer, this.api_error.ref());
  if (this.api_error.hasError()) {
    return this.api_error.toString();
  }
};

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
