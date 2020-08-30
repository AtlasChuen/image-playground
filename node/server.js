const express = require('express');
const path = require('path');
const { grayscalify, jpeg_compress } = require('../pkg/lib.js');
const fileUpload = require('express-fileupload');
const {createImageData, Canvas} = require('canvas');
const fs = require("fs");
const output = require('image-output')

const app = express();
const port = 8080;
app.use(express.static(__dirname + '/../pkg'));
app.use(express.static(__dirname + '/public'));

app.use(express.urlencoded({ extended: false }));
app.use(fileUpload());

app.set('views', path.join(__dirname, 'views'))
app.set('view engine', 'ejs');

app.get('/', (req, res) => { res.render("index", {
  imgSrc: ''
}) })

app.post('/grayscalify', function(req, res) {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }
  if(req.files.image.mimetype !== "image/jpeg") {
    return res.status(400).send('Only jpg supported.');
  }
  grayscalify(req.files.image.data);
  res.render("index", {
    imgSrc: 'uploaded/uploaded.jpg'
  });
})

app.post('/compress', async function(req, res) {
  if (!req.files || Object.keys(req.files).length === 0) {
    return res.status(400).send('No files were uploaded.');
  }
  console.log(req.files.image); // the uploaded file object

  var compressed_gray = jpeg_compress(req.files.image.data);
  // var c = new Canvas(320, 240);
  var ctx = c.getContext("2d");
  output({
    data: compressed_gray,
    width: 320,
    height: 240
  }, "compressed_gray.jpg")

  // const newImg = createImageData(Uint8ClampedArray.from(gray), 320);
  // ctx.putImageData(newImg, 320, 240); 

  res.render("index", {
    // imgSrc: 'data:image/png;base64,' + base64Data
    // imgSrc: 'uploaded/test.jpg'
    imgSrc: 'compressed_gray.jpg',
  });
});

app.listen(port, () => console.log(`Listening at http://localhost:${port}`))
