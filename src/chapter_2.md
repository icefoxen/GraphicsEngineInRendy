# Chapter 1

## The Vulkan drawing model

Buffers, instances, render targets, pipelines, shaders, instances...

## `gfx-hal` differences from Vulkan

People WILL always ask this, so we should write it down.

## Rendy's drawing model

What it does for you:

 * Resource makes resource/memory management easier, though it's still manual
 * Graph synchronizes passes
 * Command and Frame make the actual drawing, submission and swapchain
   stuff easier
 * Shader makes shader handling easier
 * WSI does what it says on the tin
