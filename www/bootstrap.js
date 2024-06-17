
loadImage = (src) => {
    return new Promise((resolve,reject) => 
    {
        let img = document.createElement('img');
        img.crossOrigin = "*";

        img.onload = () =>
        {
            resolve(img);
        };

        img.onerror = (err) =>
        {
            reject(err);
        }

        img.src = src;
    });
};

import("./index.js").then((mod) => 
{
    let texture_sources = 
    [
        loadImage("https://b38tn1k.com/sprites/possum.png"),
        loadImage("https://as2.ftcdn.net/v2/jpg/01/62/70/99/1000_F_162709948_qKGXdatZdGFkhUp84GPWqezGGTfnj1RP.jpg"),
    ];

    Promise.all(texture_sources).then((images) => 
    {
        mod.init(images)
    }).catch((err)=> {
        console.log("Failed to load:",err);
    });
});