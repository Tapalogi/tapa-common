extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(IBincodeSerializable)]
pub fn macro_derive_ibincodeserializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_ibincodeserializable(&ast)
}

fn impl_derive_ibincodeserializable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl IBincodeSerializable for #name {}
    };
    gen.into()
}

#[proc_macro_derive(ICborSerializable)]
pub fn macro_derive_icborserializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_icborserializable(&ast)
}

fn impl_derive_icborserializable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ICborSerializable for #name {}
    };
    gen.into()
}

#[proc_macro_derive(IFlexbuffersSerializable)]
pub fn macro_derive_iflexbuffersserializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_iflexbuffersserializable(&ast)
}

fn impl_derive_iflexbuffersserializable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl IFlexbuffersSerializable for #name {}
    };
    gen.into()
}

#[proc_macro_derive(IJsonSerializable)]
pub fn macro_derive_ijsonserializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_ijsonserializable(&ast)
}

fn impl_derive_ijsonserializable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl IJsonSerializable for #name {}
    };
    gen.into()
}

#[proc_macro_derive(ITomlSerializable)]
pub fn macro_derive_itomlserializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_itomlserializable(&ast)
}

fn impl_derive_itomlserializable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ITomlSerializable for #name {}
    };
    gen.into()
}

#[proc_macro_derive(IYamlSerializable)]
pub fn macro_derive_iyamlserializable(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_derive_iyamlserializable(&ast)
}

fn impl_derive_iyamlserializable(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl IYamlSerializable for #name {}
    };
    gen.into()
}
