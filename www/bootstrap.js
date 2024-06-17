import("./index.js").then((mod) => {

    let img = document.createElement('img');
    img.crossOrigin = "*"

    img.onload = () => {
        mod.init(img);
    };

    img.src = 'https://b38tn1k.com/sprites/possum.png';
});