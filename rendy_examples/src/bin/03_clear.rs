use env_logger;
use log;
use rendy::wsi::winit::{ControlFlow, Event, EventsLoop, WindowBuilder, WindowEvent};

use rendy::{self, command, factory, graph};

type Backend = rendy::vulkan::Backend;

fn run(
    event_loop: EventsLoop,
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

    /*
    graph_builder.add_node(
        TriangleRenderPipeline::builder()
            .into_subpass()
            .with_color_surface()
            .into_pass()
            .with_surface(
                surface,
                Some(hal::command::ClearValue {
                    color: hal::command::ClearColor {
                        float32: [1.0, 1.0, 1.0, 1.0],
                    },
                }),
            ),
    );
    */
    let graph = graph_builder
        .build(&mut factory, &mut families, &())
        .unwrap();

    run(event_loop, factory, families, graph);
}
