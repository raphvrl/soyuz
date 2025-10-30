use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput, Fields};

mod utils;

use utils::detect_vertex_format;

#[proc_macro_derive(Vertex)]
pub fn derive_vertex(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let struct_name = &input.ident;

    let fields = if let Data::Struct(data_struct) = &input.data {
        match &data_struct.fields {
            Fields::Named(fields) => &fields.named,
            Fields::Unnamed(_fields) => {
                return syn::Error::new(
                    struct_name.span(),
                    "Vertex derive ne supporte que les structs avec des champs nommés",
                )
                .to_compile_error()
                .into();
            }
            Fields::Unit => {
                return syn::Error::new(
                    struct_name.span(),
                    "Vertex derive nécessite au moins un champ",
                )
                .to_compile_error()
                .into();
            }
        }
    } else {
        return syn::Error::new(
            struct_name.span(),
            "Vertex derive ne peut être utilisé que sur des structs",
        )
        .to_compile_error()
        .into();
    };

    let mut attrs = Vec::new();
    let mut location = 0u32;

    for field in fields {
        if let Some(ident) = &field.ident {
            let field_type = &field.ty;

            let format = detect_vertex_format(field_type);

            let location_idx = location;
            attrs.push(quote! {
                wgpu::VertexAttribute {
                    format: #format,
                    offset: std::mem::offset_of!(#struct_name, #ident) as wgpu::BufferAddress,
                    shader_location: #location_idx as _,
                }
            });

            location += 1;
        }
    }

    let expanded = quote! {
        impl Copy for #struct_name {}
        impl Clone for #struct_name {
            fn clone(&self) -> Self {
                *self
            }
        }

        unsafe impl bytemuck::Pod for #struct_name {}
        unsafe impl bytemuck::Zeroable for #struct_name {}

        impl #struct_name {
            pub fn desc() -> wgpu::VertexBufferLayout<'static> {
                const ATTRIBS: [wgpu::VertexAttribute; #location as usize] = [
                    #(#attrs),*
                ];

                wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &ATTRIBS,
                }
            }
        }

        impl crate::graphics::resources::VertexTrait for #struct_name {
            fn desc() -> wgpu::VertexBufferLayout<'static> {
                <Self>::desc()
            }
        }
    };

    TokenStream::from(expanded)
}
