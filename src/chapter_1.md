# Background

## gfx-hal


`gfx-hal` is a low-level platform-independent graphics library written in Rust.  Its goal is for you to be able to write graphics code once and have it function the same on any platform: Windows, Linux, MacOS, mobile, whatever.  To do this it offers several "backends" that translate its own API to whatever graphics API is native to the platform.  On Windows it can use DirectX 12, Vulkan or OpenGL, on MacOS it uses OpenGL or Metal, and so on.  It is designed to be high-performance and unopinionated, so it is quite low-level; in fact, it is deliberately designed to essentially mimic (a Rust-ified version of) the Vulkan graphics standard.  The idea is that you can basically just write code that draws things with Vulkan, and it will run anywhere regardless of whether the platform natively implements Vulkan or not.

This is, of course, insane.  However, it also works out very well in practice.  There is a little run-time overhead when using non-Vulkan backends, some features are not (yet) implemented, but the translation is in general amazingly robust.  You can give it shaders in Vulkan's SPIR-V format and it will compile them to the platform's preferred shader language, you can use Vulkan's synchronization primitives and it will do the right thing, and so on.

## Rendy

The downside to this approach is that Vulkan is very low-level.  I find it mentally a lot like writing assembly language: You can do **anything**, but you have to know exactly what you want to do and how to do it, and then implement it all yourself.  You have ultimate control and flexibility, but also ultimate responsibility.  For most purposes, this is overkill.  95% of uses really don't need the *full* power of Vulkan, but it still forces you to handle it all.  You need a memory manager, you need to recreate your swapchain every time a window is resized, you need to specify exactly how each step of a texture upload is specified, you need to assemble render passes into a pipeline and ensure synchronization between them, and so on.  This gets tedious, and `gfx-hal` implements all of it.

Enter `rendy`.  `rendy` is a toolkit for handling the gory details of `gfx-hal` for you.  It is **not** a rendering library, and it does **not** hide any of the details for you.  Rather it provides a bunch of modules that give decent solutions to common tasks, presented in an a la carte fashion.  You are still writing unsafe `gfx-hal` code, but `rendy` automates a lot of the routine parts for you that need to be written for every program.  This includes:

 * Swapchains
 * Memory management, creating and destroying resources
 * Organizing render passes
 * Defining the bindings of shader resources
 * Interfacing with the OS windowing system

And so on.  Vulkan is very much designed to make it nice to have higher-level tools sitting on top of it, so
`rendy` is a natural extension of it.  Other examples inclue [AMD's Vulkan Memory
Allocator](https://github.com/GPUOpen-LibrariesAndSDKs/VulkanMemoryAllocator) (which also has [Rust
bindings](https://github.com/gwihlidal/vk-mem-rs)), [AMD's Anvil
framework](https://github.com/GPUOpen-LibrariesAndSDKs/Anvil/), and [the `wgpu`
crate](https://github.com/gfx-rs/wgpu-rs).  I think of Vulkan as a lot like assembly language: It's very
powerful but very long-winded to write by hand, but really comes into its own when used as a basis for
higher-level tools.

 `rendy` is still under development but it functions very well already, and turns the infamous 1000+ lines of
boilerplate to draw a triangle in Vulkan into something more like 400 lines, handling a lot of the incidental
complexity along the way.  Notably, it 

# Disclaimers

 * `gfx-hal` is not *quite* identical to Vulkan.  However, [the differences are pretty minor](https://github.com/gfx-rs/gfx/wiki/Deviations-from-Vulkan).
 * `rendy` does not *quite* work perfectly on all backends yet.  TODO: Fill in details here; last I checked
   the DX12 backend didn't work quite right
 * `gfx-hal` and `rendy` are unsafe API's.  It is up to you to make safe code atop them, or not, as you see fit.  This is just due to the nature of Vulkan, which does essentially no run-time checking by default, and the decision fo `gfx-hal` to not add any overhead to that.  However it's mostly pretty easy and straightforward unsafe code; there's lots of array index arithmatic and manual resource management, but not a lot of complicated pointer shenanigans or lurking synchronization sharks.  We will be doing our best to add safety where feasible.

So, let's get to it!


