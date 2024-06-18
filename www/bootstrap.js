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

loadShader = (src) => {
    return new Promise((resolve,reject) => 
    {
        fetch(src).then((res) => 
        {
            resolve(res.text());
        }).catch((err) =>
        {
            reject(err);
        });
    });
};

let texture_sources = 
[
    loadImage("http://localhost:3030/static/possum_sprite_sheet.png"),
    loadImage("http://localhost:3030/static/background.png"),
];

let shader_sources = 
[
    loadShader("http://localhost:3030/static/sprite_vert.glsl"),
    loadShader("http://localhost:3030/static/sprite_frag.glsl"),
];

import("./index.js").then((mod) => 
{
    Promise.all(texture_sources).then((textures) => 
    {
        Promise.all(shader_sources).then((shaders) => 
        {
            //TODO: handle errors here
            mod.init(textures,shaders);

        }).catch((err)=> {
            console.log("Failed to load shaders:",err);
        });
    }).catch((err)=> {
        console.log("Failed to load textures:",err);
    });
});