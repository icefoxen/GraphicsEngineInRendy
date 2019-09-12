# Introduction

This is a guide for how to write a simple graphics system in Rust, using `rendy` and `gfx-hal`.  
`gfx-hal` is a low-level portable drawing library based very closely on the Vulkan API, and
`rendy` is a higher-level toolkit developed by the [Amethyst](https://amethyst.rs/) project
for building higher-level tools with `gfx-hal`.
We will be going from a basic "Clear the screen" tutorial to drawing 3D shapes with shaders,
instancing and textures, and along the way pointing out the various helpers and enhancements the
`rendy` crate provides over bare Vulkan.

Note that this is NOT a tutorial for learning `gfx-hal` or Vulkan, it assumes you have at least written some basic Vulkan programs.  It also assumes you are reasonably familiar with the Rust programming language.  If this is not the case, here are some useful resources:

 * [The Official Unofficial Vulkan tutorial](https://vulkan-tutorial.com/)
 * [Lokathor's Learn `gfx-hal` Tutorial](https://github.com/rust-tutorials/learn-gfx-hal)
 * [The Rust Book](https://doc.rust-lang.org/book/)
 * [The Rustonomicon](https://doc.rust-lang.org/nomicon/index.html), which isn't as scary as it sounds.

This guide is up to date as of September 2019, and uses `rendy` 0.3 and `gfx-hal` 0.3.  We also use `winit` 0.19 for windowing.


## Acknowledgements

Thank you to the [Amethyst project](https://amethyst.rs/) for making `rendy` and for funding this tutorial.
