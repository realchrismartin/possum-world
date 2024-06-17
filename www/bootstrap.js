import("./index.js").then((mod) => {

    let img = document.createElement('img');
    img.crossOrigin = "*"

    img.onload = () => {
        mod.init(img);
    };

    img.src = 'https://images.pexels.com/photos/7495924/pexels-photo-7495924.jpeg?auto=compress&cs=tinysrgb&w=600';
});