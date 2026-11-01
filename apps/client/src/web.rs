use wasm_bindgen::{JsCast, JsValue};
use web_sys::HtmlCanvasElement;

const CANVAS_WIDTH: u32 = 800;
const CANVAS_HEIGHT: u32 = 600;

pub async fn render_triangle() -> Result<(), JsValue> {
    let window =
        web_sys::window().ok_or_else(|| JsValue::from_str("Browser window is unavailable"))?;
    let document = window
        .document()
        .ok_or_else(|| JsValue::from_str("Browser document is unavailable"))?;
    let canvas = document
        .get_element_by_id("game-canvas")
        .ok_or_else(|| JsValue::from_str("The game canvas element was not found"))?
        .dyn_into::<HtmlCanvasElement>()
        .map_err(|_| JsValue::from_str("The game canvas element is not a canvas"))?;

    canvas.set_width(CANVAS_WIDTH);
    canvas.set_height(CANVAS_HEIGHT);

    let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
    let surface = instance
        .create_surface(wgpu::SurfaceTarget::Canvas(canvas))
        .map_err(|error| JsValue::from_str(&format!("Failed to create wgpu surface: {error}")))?;
    let adapter = instance
        .request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::default(),
            compatible_surface: Some(&surface),
            force_fallback_adapter: false,
        })
        .await
        .ok_or_else(|| JsValue::from_str("No compatible GPU adapter is available"))?;
    let (device, queue) = adapter
        .request_device(
            &wgpu::DeviceDescriptor {
                label: Some("CreateCraft device"),
                required_features: wgpu::Features::empty(),
                required_limits: wgpu::Limits::default(),
                memory_hints: wgpu::MemoryHints::Performance,
            },
            None,
        )
        .await
        .map_err(|error| JsValue::from_str(&format!("Failed to request wgpu device: {error}")))?;

    let capabilities = surface.get_capabilities(&adapter);
    let format = capabilities
        .formats
        .iter()
        .copied()
        .find(wgpu::TextureFormat::is_srgb)
        .or_else(|| capabilities.formats.first().copied())
        .ok_or_else(|| JsValue::from_str("The canvas surface supports no texture formats"))?;
    let alpha_mode = capabilities
        .alpha_modes
        .first()
        .copied()
        .ok_or_else(|| JsValue::from_str("The canvas surface supports no alpha modes"))?;
    surface.configure(
        &device,
        &wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            width: CANVAS_WIDTH,
            height: CANVAS_HEIGHT,
            present_mode: wgpu::PresentMode::Fifo,
            alpha_mode,
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        },
    );

    let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
        label: Some("Colored triangle shader"),
        source: wgpu::ShaderSource::Wgsl(
            r#"
                struct VertexOutput {
                    @builtin(position) position: vec4<f32>,
                    @location(0) color: vec3<f32>,
                }

                @vertex
                fn vs_main(@builtin(vertex_index) vertex_index: u32) -> VertexOutput {
                    let positions = array<vec2<f32>, 3>(
                        vec2<f32>(0.0, 0.7),
                        vec2<f32>(-0.7, -0.7),
                        vec2<f32>(0.7, -0.7),
                    );
                    let colors = array<vec3<f32>, 3>(
                        vec3<f32>(1.0, 0.15, 0.15),
                        vec3<f32>(0.15, 1.0, 0.25),
                        vec3<f32>(0.2, 0.35, 1.0),
                    );
                    var output: VertexOutput;
                    output.position = vec4<f32>(positions[vertex_index], 0.0, 1.0);
                    output.color = colors[vertex_index];
                    return output;
                }

                @fragment
                fn fs_main(input: VertexOutput) -> @location(0) vec4<f32> {
                    return vec4<f32>(input.color, 1.0);
                }
            "#
            .into(),
        ),
    });
    let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
        label: Some("Triangle pipeline layout"),
        bind_group_layouts: &[],
        push_constant_ranges: &[],
    });
    let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Colored triangle pipeline"),
        layout: Some(&pipeline_layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: Some("vs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            buffers: &[],
        },
        primitive: wgpu::PrimitiveState::default(),
        depth_stencil: None,
        multisample: wgpu::MultisampleState::default(),
        fragment: Some(wgpu::FragmentState {
            module: &shader,
            entry_point: Some("fs_main"),
            compilation_options: wgpu::PipelineCompilationOptions::default(),
            targets: &[Some(wgpu::ColorTargetState {
                format,
                blend: Some(wgpu::BlendState::REPLACE),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        multiview: None,
        cache: None,
    });

    let frame = surface
        .get_current_texture()
        .map_err(|error| JsValue::from_str(&format!("Failed to acquire canvas frame: {error}")))?;
    let view = frame
        .texture
        .create_view(&wgpu::TextureViewDescriptor::default());
    let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
        label: Some("Triangle render encoder"),
    });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("Triangle render pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color {
                        r: 0.08,
                        g: 0.1,
                        b: 0.16,
                        a: 1.0,
                    }),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&pipeline);
        pass.draw(0..3, 0..1);
    }

    queue.submit(Some(encoder.finish()));
    frame.present();
    Ok(())
}
