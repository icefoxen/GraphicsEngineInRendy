# Hello, Empty Screen

Okay, enough theoretical talk, let's get to code.  The code for this chapter is available in the
[Github repo](https://github.com/icefoxen/GraphicsEngineInRendy/blob/master/code/src/bin/03_clear.rs)
but I encourage you to write your own from scratch rather than just following along.

## Setup

First, naturally, we have to do some setup to create a window and handle an event loop.  We are using `winit`,
the de-facto standard for cross-platform windowing in Rust.  You can also use just about anything that will
give you a valid graphics context, including SDL, GLFW, or OS specific API calls.

We will also use some helper crates to make life easier as we go: `log` and `env_logger` for logging, and `failure` for error handling (inherited from `rendy`).  So, just make a new Cargo project and throw the following into your Cargo.toml:

```toml
log = "0.4"
env_logger = "0.6"
failure = "0.1"

rendy = {version = "0.4", features = ["base", "vulkan"]}
```

## Windowing

We will be using `winit` 0.19 for windowing.  `winit` 0.20 has some relatively large API changes around the event loop that are intended to make life simpler for making programs portable to Webassembly, but it's not done yet and so we will be using the older version.  It's not perfect, but it works pretty well, and is easy to set up.  Rendy also re-exports it as the `rendy::wsi::winit` module, so you can just use that and always get the version that Rendy supports.  There's work being done on making it less strongly-coupled, but for now it seems like the simplest way to go.

Here we get our first taste of `rendy`'s module structure: there isn't
really a central library, but just a loose connection of subcrates that
can occasionally use each other.  Then, for simplicity, there's a
`rendy` crate that re-exports all the other crates, which can be
individually enabled or disabled as you wish with feature flags.
`wsi` stands for "Window System Integration", and contains an API for
doing whatever is needed to connect `rendy` to a windowing setup; right
now only winit is implemented, but others are possible.

So, our main function looks something like this:

```rust
use env_logger;
use failure::Error;
use log;
use rendy::wsi::winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};

fn main() {
    env_logger::Builder::from_default_env()
        .filter_module("03_clear", log::LevelFilter::Trace)
        .init();

    let event_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_title("Hello, empty screen!")
        .build(&event_loop)
        .expect("Could not create window!");

   run(event_loop)
}
```

We pulled the actual main loop out into its own function, so let's write
that next.  It just looks like this:

```rust
fn run(
    mut event_loop: EventsLoop,
) {
    event_loop.run_forever(move |event| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            // Time to cleanup and quit!
            ControlFlow::Break
        }

        Event::WindowEvent {
            event: WindowEvent::Refresh,
            ..
        } => {
            // Drawing code will go here
            ControlFlow::Continue
        }
        _ => ControlFlow::Continue,
    })
}
```

So run this and we should get an empty window with nothing being drawn
in it at all!  Huzzah!

## Choosing a backend

The first step to drawing stuff is choosing what graphics backend to
draw it with.  This is where we decide whether our program uses Vulkan,
Metal, OpenGL or whatever, and takes the form of a generic implementing
`gfx-hal`'s `Backend` trait.  `gfx-hal` provides crates
implementing this for different graphics API's, such as
`gfx_backend_dx12` or `gfx_backend_metal`.  `rendy` conveniently has
features that enable each of these backends and that makes the
re-exported crate available under `rendy::util`.  These are disabled by
default though, so as you see in our
`Cargo.toml` we've enabled the `vulkan` feature for `rendy` and will now
have a module `rendy::util::vulkan`.

Here we see another fundamental design decision of `rendy`: It works alongside
`gfx-hal`, as a superset of it that adds more tools, and NOT as
something trying to hide `gfx-hal` as an implementation
detail.

For this tutorial, I am just going to use the `vulkan` backend for the
sake of simplicity.  Since backends are generic arguments to various
functions and types, they MUST be chosen at compile-time, and if the
platform your program is being built for doesn't support it then it
fails at compile-time.  While we all want a crate that automatically
chooses the "best" backend on a given platform, that doesn't exist yet,
and so the usual method is to do some tedious fiddling with `#[cfg()]`
options and feature flags.  This is all fairly boring and easy to find
in the `rendy` examples though, so I'm going to skip it all, and
instead just do a typedef:

```rust
type Backend = rendy::vulkan::Backend;
```

Feel free to change it and the associated feature flag to whatever is
best for your platform, or go whole hog and choose it automatically with
feature flags and `#[cfg()]` statements.

Great, but now what do we actually do with this?  `rendy` has three main
top-level types: the `Factory`, the command queue `Families`, and the
frame `Graph`.  Unlike using just bare Vulkan, `rendy` will
happily give us sensible defaults, so creating these are pretty simple.

```rust
let config: factory::Config = Default::default();
let (mut factory, mut families):
    (factory::Factory<Backend>, command::Families<Backend>) =
    factory::init(config).expect("Could not init Rendy context");
```

The `Config` object lets us choose things like what graphics device we
are using, what memory to use for our memory allocation, and all that
jazz.  But it gives us pretty good defaults, so I've yet to need
to actually mess with it.

We just pass the `Config` to the `factory::init()` method and it creates
two things for us, the `Factory` and the `Families`.  Both of these
types are generic on the backend type, as are most `rendy` types, so we
explicitly tell it what the types are and it Just Does The Right Thing
to initialize the correct backend.  The `Factory` is something we will
see a lot, it is the core structure that `rendy` uses to create and
destroy resources like images and buffers.  It contains the
`gfx-hal`/Vulkan `Instance` and `Device` objects, as well as a bunch of
other stuff.  `Families` gets used less often, it is mainly a
description of the command queues that the device supports.

So, let's carry on:

```
let surface = factory.create_surface(&window);

let graph_builder = graph::GraphBuilder::<Backend, ()>::new();

let graph = graph_builder
    .build(&mut factory, &mut families, &())
    .unwrap();

run(event_loop, factory, families, graph);
```

This is pretty straight-forward.  We create a `surface`, which is the
final render target that gets displayed on the screen, and we create a
`graph` object.  Then we pass the `factory`, `families` and `graph` all
to our `run()` function.  We don't even use the `surface` yet, that will
come in a moment.  You see that we specify two types for our
`graph_builder`, the first is our `Backend` type, and the second is just
`()`.  The second type is there for you to store auxiliary data that
gets owned by the `Graph` and shared between graphics pipelines, and is often just called `aux` in
the API.  We don't need it right now, so we just use `()`; you can see
that we provide the initial value for it in the last argument of
`graph_builder.build()`.

There's only a couple real changes that to happen to our `run()`
function, but they're important:

```rust
fn run(
    mut event_loop: EventsLoop,
    mut factory: factory::Factory<Backend>,
    mut families: command::Families<Backend>,
    graph: graph::Graph<Backend, ()>,
) {
    let mut graph = Some(graph);
    event_loop.run_forever(move |event| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => {
            if let Some(g) = graph.take() {
                g.dispose(&mut factory, &());
            }
            ControlFlow::Break
        }

        Event::WindowEvent {
            event: WindowEvent::Refresh,
            ..
        } => {
            factory.maintain(&mut families);
            if let Some(ref mut graph) = graph {
                graph.run(&mut factory, &mut families, &());
            }
            ControlFlow::Continue
        }
        _ => ControlFlow::Continue,
    })
}
```

First, we store our `Graph` in an option.  If the window gets resized
or our render target is otherwise invalidated (such as a device going to
sleep and waking back up perhaps), then we're going to have to destroy
the old `Graph` and create a new one.  You can see the disposal
happening in the `WindowEvent::CloseRequested` match arm.

This is another key pattern: `Graph::dispose()` destroys our `Graph`,
but calling it takes a reference to our
`Factory`.  Like `gfx-hal`, most types in `rendy` take some amount of
manual memory management!  Vulkan requires that each resource is
destroyed by the `Instance` that created it, since the resource is
usually representing a chunk of memory or something on a specific GPU,
and the `Instance` describes that relationship.
On multi-GPU systems, it's really not very helpful for GPU 1 to try to
allocate or free GPU 2's memory!  Since we need a reference to the correct
`Factory` to destroy an object, the objects can't just implement `Drop`.
Well, they could, but it would involve either a global `Factory` or
every single object created by a `Factory` to hold an `Rc` to it,
neither of those solutions are really desirable either.  So, we are
forced to take care of it by hand.

`rendy` does help us out to make life easier though.  The `Graph` is
designed to make it easy to create and destroy related objects in chunks
fairly automatically, and there are some tools that make it easier to
ensure things get freed properly that we will see in the next chapter.
The API is also very consistent: Anything that needs to be manually
destroyed has a `dispose()` method, which usually takes a
`&mut Factory<B>`, and it's a pretty good bet that if you either
create a thing from the `Factory` itself or from a method taking
`&mut Factory` then you will have to call its `dispose()` method
somewhere.

Anyway!  You should be able to run this code and you will get... STILL
an empty window with nothing drawn in it.  Progress!

## Render groups and the `Graph`

Okay, let's get set up to actually do something, even if it's just
clearing the screen.  This is what the
`Graph` is for.  It implements a system called a "frame graph", which
is almost but not quite entirely unlike a "scene graph" you may have
encountered as a method for representing objects in a scene.  A frame
graph instead organizes multiple render targets and coordinates the
order in which they are drawn.

So, some review.  A "render target" is a buffer of pixels (more or less)
that a scene gets rendered to; what gets shown in your actual window is
a render target.  But you can also have render targets that don't get
displayed to the screen, but are inputs to other drawing steps.
For a simple example, you might implement a mirror in a game by drawing
a scene from one angle, then using the render target as a texture and
applying that texture to the mirror quad when you draw the scene from
a different angle.
A more complicated example might be making dynamic shadows: first rendering the
scene, then for each light in the scene rendering its shadow map to a
different render target, then finally using the information in all the
shadow maps to draw shadows on top of the original scene.  This process
involves several steps rendering to different render targets, some of
which can be run in parallel with each other and some of which can't.
This is the process that the frame graph coordinates.

A `Graph` contains zero or more node's.  Each node is
essentially a structure with a couple methods that takes a set of inputs
(`Buffer`'s and `Image`'s) and does stuff with them (generally drawing
something or calculating some data).  Further, each node can say
that it depends on other nodes to execute before it, and what inputs
it depends on.

Like memory allocation, it is up to you to get this right.  If you
make one node write to a buffer and another one read from the same
buffer, and don't specify the dependency between them correctly,
`rendy` can't tell you that you have a data race and force you
to fix it.  However,
like memory allocation, `rendy` does its best to make it easier,
by letting you organize all the resources and node dependencies
in one place.  More importantly, once you DO get it organized
properly, `rendy` **takes care of the synchronization and ordering
for you**.  You generally don't have to touch Vulkan's semaphores or fences
when drawing with `rendy`, it figures it out on its own based
on the information you've given it in the `Graph`.
Even better, it handles the swapchain for you; the whole laborious
Vulkan swapchain process is just another node in the `Graph`.

Great, let's actually do it.  Clearing a render target to a color
is really just a parameter on a node, so all we need is a node
which does absolutely nothing.  We create this by creating two
structures, `ClearGroup` and `ClearGroupDesc`, and implementing
a trait on each of them, `rendy::graph::render::RenderGroup`
and `rendy::graph::render::RenderGroupDesc`.
The first is our actual renderer node that does stuff (it is
actually one stage in a render pass, but we will get to that
later), and the `Desc` (short for "Descriptor") is a structure
that describes all the inputs and outputs of your `RenderGroup`
and for actually creating it.  This is much like the description
objects that Vulkan uses to, essentially, represent dynamic type 
information for buffers or pipelines.  

So once all that gets sorted out, actually implementing these traits
is pretty simple.  Our `ClearGroup` has to implement three methods:
`prepare()`, which is called before drawing to do things like buffer
data and record command queues, `draw_inline()`, which actually records
drawing commands to a queue (the "inline" part is in the name because
it's really just adding stuff to a command queue that may already exist),
and `dispose()` which frees any resources that the `RenderGroup` may
contain.

```rust
#[derive(Debug, Clone, Default)]
struct ClearGroup;

impl RenderGroup<Backend, ()> for ClearGroup {
    fn prepare(
        &mut self,
        _factory: &factory::Factory<Backend>,
        _queue: command::QueueId,
        _index: usize,
        _subpass: hal::pass::Subpass<Backend>,
        _aux: &(),
    ) -> PrepareResult {
        PrepareResult::DrawRecord
    }

    fn draw_inline(
        &mut self,
        _encoder: command::RenderPassEncoder<Backend>,
        _index: usize,
        _subpass: hal::pass::Subpass<Backend>,
        _aux: &(),
    ) {
    }

    fn dispose(self: Box<Self>, _factory: &mut factory::Factory<Backend>, _aux: &()) {}
}
```

You see that `prepare()` returns a `PrepareResult`, which may be
`DrawRecord` or `DrawReuse`.  This lets `rendy` essentially cache
drawing commands when possible; if none of the data your `RenderGroup`
is drawing has changed, it can just reuse the previous draw calls.

You can also see that these methods get passed a LOT of objects they can
Do Stuff with, and you can see again that `()` shows up as the type of
our `aux` variable.  We will talk more about what each of these things
actually are in the next chapter.

Making our `ClearGroupDesc` is similarly trivial:

```rust
#[derive(Debug, Clone, Default)]
struct ClearGroupDesc;

impl RenderGroupDesc<Backend, ()> for ClearGroupDesc {
    fn build(
        self,
        _ctx: &graph::GraphContext<Backend>,
        _factory: &mut factory::Factory<Backend>,
        _queue: command::QueueId,
        _aux: &(),
        _framebuffer_width: u32,
        _framebuffer_height: u32,
        _subpass: hal::pass::Subpass<Backend>,
        _buffers: Vec<graph::NodeBuffer>,
        _images: Vec<graph::NodeImage>,
    ) -> Result<Box<dyn RenderGroup<Backend, ()> + 'static>, Error> {
        Ok(Box::new(ClearGroup))
    }

    fn depth(&self) -> bool {
        false
    }
}
```

Note that it is the `build()` method that actually creates a
`ClearGroup` for us.  That's the only thing that `RenderGroupDesc`
*requires* you to implement, but there's several other methods on it
that you probably want to override, because they describe the shared
buffers and images that the corresponding `RenderGroup` actually uses,
as well as things like color and depth buffer information.  You can see
here that we've overridden `depth()` to return false, since we're not
using a depth buffer in this program yet.  Getting these wrong often
gives you a failed debug assertion at runtime, but it's best not to
rely on them.

## Actually clearing the screen

Great, we have a couple types with traits that do nothing!  How do we
actually get them into our `Graph`?

Well, we created a `GraphBuilder`, so that's probably where we start.
In fact, ALL the setup of a `Graph` happens in a `GraphBuilder`; the
`Graph` itself is basically considered immutable.  If you need to change
something in a `Graph`, take it apart and reuse the pieces.  Creating
all these graphics resources and figuring out the necessary dependency
relationships and such is relatively expensive, so you don't want to
be doing it often.

In Vulkan terms, the `Graph` is in charge of coordinating `RenderPass`es
and subpasses into a pipeline.  In fact, each graph node can be multiple
`Pass`es, each composed of multiple subpasses, each potentially
containing multiple `RenderGroup`'s, and so actually turning
our `RenderGroup` into a `Graph` node takes a little bit of work:

```rust
graph_builder.add_node(
    graph::render::RenderPassNodeBuilder::new()
        .with_subpass(
            graph::render::SubpassBuilder::new()
                .with_group(ClearGroupDesc::default().builder())
                .with_color_surface(),
        )
        .with_surface(
            surface,
            Some(hal::command::ClearValue::Color(
                hal::command::ClearColor::Sfloat([0.1, 0.2, 0.3, 1.0]),
            )),
        ),
);
```

We'll go through this one step at a time:

 * First we create a `RenderPassNodeBuilder`
 * We add a subpass to it by creating a `SubpassBuilder`
 * We create an instance of our `ClearGroupDesc` type and call
   `.builder()` on it which turns it into a `graph::DescBuilder`.  This
   would let us add more information about the group like dependencies
   or resources it uses, but we don't have any of that so we just add it
   to the `SubpassBuilder`.
 * We call `.with_color_surface()` on our `SubpassBuilder` to tell it
   that we want a color attachment involved.
 * We are done creating our subpass, so next we call `.with_surface()` on
   our `RenderPassNodeBuilder`.  This gets given the `surface`
   corresponding to the window's render target we made way back when we
   created the `Factory`...
 * ..And finally, we also give it the color to clear the surface to
   before it is executed.  Note this is an `Option`; if we passed `None`
   it would just not clear the surface at all before being called.

TODO: Do we need a PresentNode???

Huzzah, we now have a `Graph` node containing a render pass which does
utterly nothing but clear the surface given to it!  There's a lot going
on in the 150ish lines of code, but we now have a pretty good framework set
up for starting to actually draw things.

## Extra credit: Resizing the window

This I will leave as an exercise to the reader!  Hint: All you really
have to do is destroy and re-create the `Graph` when a window resize
event happens.
