use proc_macro::TokenStream;
use quote::quote;
use syn::Attribute;

#[proc_macro_derive(EnumDiscriminate)]
pub fn enum_discriminate_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate.
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation.
    impl_enum_discriminate_derive(&ast)
}

fn has_repr_u8(attrs: &[Attribute]) -> bool {
    let mut is_u8 = false;
    for attr in attrs {
        if attr.path().is_ident("repr") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("u8") {
                    is_u8 = true;
                }
                Ok(())
            })
            .unwrap()
        }
    }
    is_u8
}

fn impl_enum_discriminate_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if !has_repr_u8(&ast.attrs) {
        panic!("EnumDiscriminate can only be derived for enums which have the u8 repr");
    }
    let generated = quote! {
        impl #name {
            pub const fn discriminant(&self) -> u8 {
                // SAFETY: Because we require the enum to be marked as `repr(u8)`, its layout is a `repr(C)` `union`
                // between `repr(C)` structs, each of which has the `u8` discriminant as its first
                // field, so we can read the discriminant without offsetting the pointer.
                unsafe {
                    let ptr = self as *const Self;
                    let discriminant_ptr = ptr.cast::<u8>();
                    *discriminant_ptr
                }
            }
        }
    };
    generated.into()
}
