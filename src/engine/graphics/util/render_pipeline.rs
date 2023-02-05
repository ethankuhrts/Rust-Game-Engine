use crate::graphics::{
    Vertex,
    GraphicsBundle,
    GraphicsSettings, InstanceRaw, Texture,
};

pub fn create_render_pipeline(bundle: &GraphicsBundle, layout: &wgpu::PipelineLayout, shader: &wgpu::ShaderModule, transparent: bool, cull_back_face: bool) -> wgpu::RenderPipeline {

    let cull_mode = match cull_back_face {
        true => Some(wgpu::Face::Back),
        false => None,
    };
    let blend_state = match transparent {
        true => wgpu::BlendState::ALPHA_BLENDING,
        false => wgpu::BlendState::REPLACE
    };

    let render_pipeline = bundle.device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
        label: Some("Render Pipeline"),
        layout: Some(layout),
        vertex: wgpu::VertexState {
            module: &shader,
            entry_point: "vs_main", // 1.
            buffers: &[ Vertex::desc(), InstanceRaw::desc() ], // 2.
        },
        fragment: Some(wgpu::FragmentState { // 3.
            module: &shader,
            entry_point: "fs_main",
            targets: &[Some(wgpu::ColorTargetState { // 4.
                format: bundle.config.format,
                blend: Some(blend_state),
                write_mask: wgpu::ColorWrites::ALL,
            })],
        }),
        primitive: wgpu::PrimitiveState {
            topology: wgpu::PrimitiveTopology::TriangleList, // 1.
            strip_index_format: None,
            front_face: wgpu::FrontFace::Ccw, // 2.
            cull_mode: cull_mode,
            
            // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
            polygon_mode: wgpu::PolygonMode::Fill,
            // Requires Features::DEPTH_CLIP_CONTROL
            unclipped_depth: false,
            // Requires Features::CONSERVATIVE_RASTERIZATION
            conservative: false,
        },
        depth_stencil: Some(wgpu::DepthStencilState {
            format: Texture::DEPTH_FORMAT,
            depth_write_enabled: true,
            depth_compare: wgpu::CompareFunction::Less, // 1.
            stencil: wgpu::StencilState::default(), // 2.
            bias: wgpu::DepthBiasState::default(),
        }), // 1.
        multisample: wgpu::MultisampleState {
            count: 1, // 2.
            mask: !0, // 3.
            alpha_to_coverage_enabled: false, // 4.
        },
        multiview: None, // 5.
    });

    return render_pipeline;
}