const {ipcRenderer} = require('electron');
const Capi = require('./capi');
const capi = new Capi("../../res/opus5.gb");

capi.run_threaded();

const back_canvas = document.createElement("canvas");
back_canvas.width = 160;
back_canvas.height = 144;
// document.body.appendChild(back_canvas);   
const back_context = back_canvas.getContext("2d");
const back_screen = back_context.createImageData(160, 144);
back_context.putImageData(back_screen, 0, 0);

const canvas = document.querySelector("#canvas");
canvas.width = 160*2;
canvas.height = 144*2;
const context = canvas.getContext('2d');
context.imageSmoothingEnabled = false;

document.addEventListener('keydown', function (e) {
   capi.set_button(e.which, true);
});
document.addEventListener('keyup', function (e) {
   capi.set_button(e.which, false);
});

setInterval(function() {
  const data = capi.updated_frame();
  if (data != null) {
    for (var x = 0; x < data.length; x++) {
      back_screen.data[x] = data[x];
    }
  }
  back_context.putImageData(back_screen, 0, 0);

  context.clearRect(0, 0, canvas.width, canvas.height);
  context.drawImage(back_canvas, 0, 0, 160, 144, 0, 0, 160*2, 144*2);
}, 1000/60);
