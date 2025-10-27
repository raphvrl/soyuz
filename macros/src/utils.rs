use quote::quote;

pub fn detect_vertex_format(ty: &syn::Type) -> proc_macro2::TokenStream {
    let type_str = quote! { #ty }.to_string();

    match type_str.as_str() {
        _ if type_str.contains("[f32; 2]") => quote! { wgpu::VertexFormat::Float32x2 },
        _ if type_str.contains("[f32; 3]") => quote! { wgpu::VertexFormat::Float32x3 },
        _ if type_str.contains("[f32; 4]") => quote! { wgpu::VertexFormat::Float32x4 },
        _ if type_str.contains("Vec2") => quote! { wgpu::VertexFormat::Float32x2 },
        _ if type_str.contains("Vec3") => quote! { wgpu::VertexFormat::Float32x3 },
        _ if type_str.contains("Vec4") => quote! { wgpu::VertexFormat::Float32x4 },
        _ => quote! { wgpu::VertexFormat::Float32x4 },
    }
}
