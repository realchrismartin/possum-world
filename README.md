<div align="center">

  <h1>Possum World</h1>
  
  <p>2D Rust & WebGL Browser Game!</p>
</div>

# Table of Contents

- [About the Project](#about-the-project)
- [Tech Stack](#tech-stack)
- [Features](#features)
- [Usage](#usage)

## About the Project

Possum World is a side-scrolling browser game implemented using Rust, WASM, and WebGL.

This repository contains both the game engine and the game content. 

### Tech Stack

[![Rust](https://img.shields.io/badge/Rust-%23000000.svg?e&logo=rust&logoColor=white)](#)
[![WebAssembly](https://img.shields.io/badge/WebAssembly-654FF0?logo=webassembly&logoColor=fff)](#)
[![JavaScript](https://img.shields.io/badge/JavaScript-F7DF1E?logo=javascript&logoColor=000)](#)

### Features

- Player-controlled Marsupial Madness!: click or tap (on mobile) to pilot your Possum around the world.
- Both Large and Small Possums!
- Extensible `Renderable` concept! : each `Renderable` can have its own unique vertex layout!
- Transform Buffering! : Transform data is uploaded to the GPU once and only modified if needed. Transforms can be shared by multiple `Renderables` using indexing!
- Support for multiple hosted `Textures`

### Usage 

- Build the project using `wasm-pack` and `npm run-script build` to generate the WASM content and then build the Javascript fluff
- If desired, build the `possum-world-fileserver` dependency and use it to host the generated content. Alternately, any static hosting site should be able to host this project.
