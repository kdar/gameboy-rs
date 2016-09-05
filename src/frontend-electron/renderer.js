const {ipcRenderer} = require('electron');
const capi = require('./capi');

const canvas = document.querySelector("#canvas");
console.log(canvas);
const context = canvas.getContext('2d');
const screen = context.createImageData(160, 144);

for(i=0; i<screen.data.length; i++) {
  screen.data[i]=145;
}

context.putImageData(screen, 0, 0);

setInterval(function() {
  const data = capi.video_data();
  screen.data = data;
  context.putImageData(screen, 0, 0);
}, 1000);

var hrstart = process.hrtime();
var hz = 0;
setInterval(function() {   
  capi.step();
  // hz++;
  // const hrend = process.hrtime(hrstart);
  // if (hrend[0] >= 1) {
  //   console.log(hz);
  //   hz = 0;
  //   hrstart = process.hrtime();
  // }
  //console.info("Execution time (hr): %ds %dms", hrend[0], hrend[1]/1000000);
}, 0);