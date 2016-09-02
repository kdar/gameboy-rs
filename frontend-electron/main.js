var ffi = require('ffi');
var path = require('path');

var lib = ffi.Library(path.join(__dirname, '../target/debug/libgameboy.so'), {
  node_test: ["void", []]
});

lib.node_test();
