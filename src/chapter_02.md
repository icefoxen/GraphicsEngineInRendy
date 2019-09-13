# Chapter 1


We are going to start with a high-level explaination and then get down to the specifics, ending up in actual code for a small rendering engine.

TODO: Then, if we have time, we are going to go back up from our code

TODO: Double-check ALL of this!

At the high level, a Vulkan application is made of a few main parts:

 * Window -- Your handle to talk to the OS about graphics and event stuff.
 * Device and instance -- Your handles to talk to the GPU(s).
 * Shaders -- A program that you write that processes data to control what is actually displayed.
 * Various data buffers such as textures, vertex buffers, and so on -- Inputs to shader programs.
 * Queue -- A queue in which commands are recorded to tell the graphics card what and how to actually draw. TODO: Command pools, buffers, etc; how does that work?
 * Render pass (and subpasses) -- A single step in rendering that runs asynchronously, accepting some inputs and producing some outputs.  I think of it as like a function.
 * Pipeline -- The whole shebang that ties together how to sequence render passes, what their inputs and outputs are, and what shaders they use.  Essentially a blueprint for the entire rendering process.
 * Swapchain -- The structure that controls when your rendered frame is displayed to the screen.

So, once you have a window, device and instance, you define a pipeline that says how to draw to the screen.  This pipeline contains one or more render passes which ulimately produce a rendered raster image, and a set of shaders that that consumes the data it's given and produces the type of output it needs to.  Each render pass binds data to some shaders and records drawing commands in one or more queue's.  And then the swapchain organizes one or more pipelines running in parallel and controls the timing of when to physically send the results to the monitor, implementing double buffering, triple buffering or whatever else you tell it to do.

The parts that Rendy offers tools to simplify:

 * Window, device and instance creation
 * Swapchain management
 * Render pass sequencing
 * Buffer creation/data transfer

## The Graph

A lot of the tools `rendy` provides come together in the `Graph`.  This is, as the name implies, a set of nodes connected by edges... where each node is a render pass, and each edge is data going from one render pass to the next.  It must (TODO: Double-check!) terminate in a single `PresentNode`, which represents the final rendered image being shown on your screen.  The `Graph` does several jobs, all of which comes down to defining and controlling the flow of data through render passes.

TODO: Cycles?  Or must it be a DAG?

The `PresentNode` is also in charge of handling the swapchain.

## The Vulkan drawing model

Buffers, instances, render targets, pipelines, shaders, instances...


    Top Level Stuff
        Instance
        Surface (requires an Instance+Window)
        Adapter (requires an Instance)
        queue_group (requires an Adapter)
        device (requires an Adapter)
    The GPU Swapchain
        swapchain (requires a Surface+Adapter+Device)
        backbuffer (requires Surface+Adapter)
        render_area (comes from the Swapchain)
        frames_in_flight (comes from the Swapchain)
        fences (requires a Device + frames_in_flight)
        semaphores (requires a Device + frames_in_flight)
    RenderPass
        render_pass (requires a Device + Swapchain format)
    Targets For Rendering
        image_views (requires a Device+Backbuffer)
        framebuffers (requires ImageView values)
    Command Issuing
        Command Pool (requires Device)
        command_buffers (requires a CommandPool + Swapchain)
    Misc
        current_frame (just starts at 0)


So the Vulkan drawing model has a few main parts, with ten million
subparts:

Pipeline -- Describes how everything goes together into being able to
run a draw call.  Describes how shaders are connected to their inputs,
as well (via descriptor sets)
Command buffer/queue -- Describes the actual drawing operations that occur
Pass (with subpasses) -- Describes how to connect together things that
render until they eventually produce a single frame
Swapchain -- Describes how to sequence the rendering of multiple
frames


## `gfx-hal` differences from Vulkan

People WILL always ask this, so we should write it down.

 * Iterators instead of using only slices
 * Non-Copy handles instead of Vulkan's integer handles -- makes borrowing and thread-safety easier to reason about
 * Omits some minor features (like triangle fans) that are [harder to make portable(https://www.khronos.org/vulkan/portability-initiative)
 * Omits a buggy feature or two

Authoratative source here:
<https://github.com/gfx-rs/gfx/wiki/Deviations-from-Vulkan>


## Rendy's drawing model

What it does for you:

 * Resource makes resource/memory management easier, though it's still manual
 * Graph synchronizes passes
 * Command and Frame make the actual drawing, submission and swapchain
   stuff easier
 * Shader makes shader handling easier
 * WSI does what it says on the tin
