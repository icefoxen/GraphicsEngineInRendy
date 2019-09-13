use env_logger;
use failure::Error;
use log;
use rendy::wsi::winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};

use rendy::hal;
use rendy::{
    self, command, factory, graph,
    graph::render::{PrepareResult, RenderGroup, RenderGroupDesc},
};

type Backend = rendy::vulkan::Backend;

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
}

fn run(
    mut event_loop: EventsLoop,
    mut factory: factory::Factory<Backend>,
    mut families: command::Families<Backend>,
    graph: graph::Graph<Backend, ()>,
) {
    let mut graph = Some(graph);
    let mut frames = 0;
    event_loop.run_forever(move |event| match event {
        Event::WindowEvent {
            event: WindowEvent::CloseRequested,
            ..
        } => ControlFlow::Break,
        Event::WindowEvent {
            event: WindowEvent::Refresh,
            ..
        } => {
            factory.maintain(&mut families);
            if let Some(ref mut graph) = graph {
                graph.run(&mut factory, &mut families, &());
                frames += 1;
            }
            ControlFlow::Continue
        }
        _ => ControlFlow::Continue,
    })
}

fn main() {
    env_logger::Builder::from_default_env()
        .filter_module("03_hello_triangle", log::LevelFilter::Trace)
        .init();

    let config: factory::Config = Default::default();
    let (mut factory, mut families): (factory::Factory<Backend>, _) =
        factory::init(config).expect("Could not init Rendy context");

    let event_loop = EventsLoop::new();

    let window = WindowBuilder::new()
        .with_title("Hello, triangle!")
        .build(&event_loop)
        .expect("Could not create window!");

    let surface = factory.create_surface(&window);

    let mut graph_builder = graph::GraphBuilder::<Backend, ()>::new();

    graph_builder.add_node(
        graph::render::SubpassBuilder::new()
            .with_color_surface()
            .with_group(ClearGroupDesc::default().builder())
            .into_pass()
            .with_surface(
                surface,
                Some(hal::command::ClearValue::Color(
                    hal::command::ClearColor::Sfloat([0.1, 0.2, 0.3, 1.0]),
                )),
            ),
    );
    let graph = graph_builder
        .build(&mut factory, &mut families, &())
        .unwrap();

    run(event_loop, factory, families, graph);
}
