const {ipcRenderer} = require('electron');
const Capi = require('./capi');
const capi = new Capi();

function run() {
  var err = capi.load_cartridge("../../../res/opus5.gb");
  if (err) {
    alert(err);
    return;
  }

  capi.run_threaded();

  const canvas = document.querySelector("#canvas");
  canvas.style.display = "block";
  canvas.style.width = (160*6) + "px";
  canvas.style.height = (144*6) + "px";

  const context = canvas.getContext('2d');
  var imageData = context.createImageData(160, 144);
  context.imageSmoothingEnabled = false;

  document.addEventListener('keydown', function (e) {
     capi.set_button(e.which, true);
  });
  document.addEventListener('keyup', function (e) {
     capi.set_button(e.which, false);
  });

  var prev_data = new Buffer(160*144*4);
  var lastRender = Date.now();
  var fps = 0;
  function draw() {
    requestAnimationFrame(draw);

    // var delta = Date.now() - lastRender;
    // if (delta > 1000) {
    //   console.log(fps);
    //   lastRender = Date.now();
    //   fps = 0;
    // }

    const data = capi.updated_frame();
    if (data == null) {
      return;
    }

    for (var x = 0; x < data.length; x++) {
      //if (prev_data[x] != data[x]) {
        imageData.data[x] = data[x];
      //  prev_data[x] = data[x];
      //}
    }

    context.putImageData(imageData, 0, 0);
    fps += 1;
  }

  draw();
}

run();
