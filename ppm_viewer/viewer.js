const PPM_LOCATION = "http://f2koi-ubuntu:8000/test.ppm";


function showError(msg) {
    let canvas = document.getElementById("imageCanvas");
    let ctx = canvas.getContext("2d");

    var errorDiv = document.getElementById("errorDiv");
    errorDiv.innerHTML = '<div class="error">Error: ' + msg + '</div>';
    ctx.clearRect(0, 0, canvas.width, canvas.height);
}

function processPPM(fileContents) {
    let canvas = document.getElementById("imageCanvas");
    let ctx = canvas.getContext("2d");

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    fileContents = fileContents.replace(/^\s+/, '').replace(/\s+$/, '');
    var data = fileContents.split(/\s+/);

    if (fileContents.substr(0, 2) != 'P3' || data[0] != 'P3') {
        showError('File is not a PPM');
        return;
    }

    var width = data[1];
    var height = data[2];
    var maxColors = data[3];

    if (maxColors != 255) {
        showError('MaxColors is not 255');
        return;
    }

    if (data.length != 3 * width * height + 4) {
        showError(
            'Not enough pixel data.<br>'
            + 'Found: ' + (data.length - 4) + '<br>'
            + 'Expecting: ' + (3 * width * height) + '<br>'
            + 'Based on width = ' + width
            + ' and height = ' + height);
        return;
    }

    errorDiv.innerHTML = '';

    canvas.width = width;
    canvas.height = height;

    var img = ctx.getImageData(0, 0, width, height);
    var pixels = img.data;

    var imageIndex = 0;
    for (var i = 4; i < data.length; i += 3) {
        pixels[imageIndex++] = data[i]; // r
        pixels[imageIndex++] = data[i + 1]; // g
        pixels[imageIndex++] = data[i + 2]; // b
        pixels[imageIndex++] = 255; // a
    }
    ctx.putImageData(img, 0, 0);
}

async function processFiles() {
    let text = await fetch(PPM_LOCATION).then(resp => resp.text());
    processPPM(text);
}

processFiles();
