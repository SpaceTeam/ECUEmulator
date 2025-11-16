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

fn has_repr_isize(attrs: &[Attribute]) -> bool {
    let mut is_isize = false;
    for attr in attrs {
        if attr.path().is_ident("repr") {
            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("isize") {
                    is_isize = true;
                }
                Ok(())
            })
            .unwrap()
        }
    }
    is_isize
}

fn impl_enum_discriminate_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    if !has_repr_isize(&ast.attrs) {
        panic!("EnumDiscriminate can only be derived for enums which have the isize repr");
    }
    let generated = quote! {
        impl #name {
            pub const fn discriminant(&self) -> isize {
                // SAFETY: Because we require the enum to be marked as `repr(isize)`, its layout is a `repr(C)` `union`
                // between `repr(C)` structs, each of which has the `isize` discriminant as its first
                // field, so we can read the discriminant without offsetting the pointer.
                unsafe {
                    let ptr = self as *const Self;
                    let discriminant_ptr = ptr.cast::<isize>();
                    *discriminant_ptr
                }
            }
        }
    };
    generated.into()
}
