use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

// registry: ::std::sync::Arc<::std::sync::Mutex<::rtoy::ComponentRegistry>>
// let mut registry = registry.lock().unwarp();
// registry.register_component(stringify!(#struct_name, || Box::new(#struct_name::init())));

#[proc_macro_attribute]
pub fn component(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    let expanded = quote! {
        #input

        #[no_mangle]
        pub extern "C" fn rtoy_engine_entry_point(registry: &::std::sync::Arc<::std::sync::Mutex<::rtoy::component::ComponentRegistry>>) {
            let mut registry = registry.lock().unwrap();
            registry.register_component(stringify!(#struct_name), || Box::new(#struct_name::init()));
            println!("Component {} registered", stringify!(#struct_name));
        }
    };

    TokenStream::from(expanded)
}
