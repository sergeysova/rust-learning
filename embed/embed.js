
var ffi = require('ffi');

var lib = ffi.Library('target/release/libembed', {
  process: ['void', []],
  demo: ['void', ['String']]
});

// lib.process();
lib.demo('Lestad');

console.log('done')
