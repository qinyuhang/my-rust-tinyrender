use proc_macro::{TokenStream};
#[allow(unused)]
use proc_macro2;
use quote::quote;
use syn;

#[allow(unused)]
#[proc_macro_attribute]
pub fn make_vec(args: TokenStream, input: TokenStream) -> TokenStream {
    // println!("args: {:?}", args);
    let mut i = input.clone();

    let ipt = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &ipt.ident;
    let name_str = name.to_string();

    // dbg!(&name);
    // dbg!(args.clone().into_iter().next().unwrap());

    let mut def_token: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let mut new_token: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let mut add_token: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let mut sub_token: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let mut mul_token: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    let mut format_token: proc_macro2::TokenStream = proc_macro2::TokenStream::new();
    args.clone().into_iter().for_each(|item| {
        let m = item.to_string();
        if m != "," {
            let m = proc_macro2::Ident::new(&m, proc_macro2::Span::from(item.span()));
            def_token.extend(quote!( #m: T, ));
            new_token.extend(quote!( #m, ));
            add_token.extend(quote!( #m: self.#m + rhs.#m, ));
            sub_token.extend(quote!( #m: self.#m - rhs.#m, ));
            mul_token.extend(quote!( #m: self.#m * rhs.#m, ));
            format_token.extend(quote!( #m: {:?} ));
        }
    });

    println!("z: {:?}", def_token.to_string());

    TokenStream::from(quote!(
    pub struct #name<
        T: Copy + PartialEq + Debug
        + core::ops::Add<Output = T>
        + core::ops::Mul<Output = T>
        + core::ops::Sub<Output = T>
        + core::ops::Div<Output = T>,
    > {
        #def_token
    }
    impl<
    T: Copy + PartialEq + Debug
    + core::ops::Add<Output = T>
    + core::ops::Mul<Output = T>
    + core::ops::Sub<Output = T>
    + core::ops::Div<Output = T>,
    > #name<T> {
        pub fn new(#def_token) -> Self {
            #name {
                #new_token
            }
        }
    }
    impl<
    T: Copy + PartialEq + Debug
    + core::ops::Add<Output = T>
    + core::ops::Mul<Output = T>
    + core::ops::Sub<Output = T>
    + core::ops::Div<Output = T>,
    > core::ops::Add for #name<T> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self::Output {
            #name {
                #add_token
            }
        }
    }
    impl<
    T: Copy + PartialEq + Debug
    + core::ops::Add<Output = T>
    + core::ops::Mul<Output = T>
    + core::ops::Sub<Output = T>
    + core::ops::Div<Output = T>,
    > core::ops::Sub for #name<T> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self::Output {
            #name {
                #sub_token
            }
        }
    }
    impl<
    T: Copy + PartialEq + Debug
    + core::ops::Add<Output = T>
    + core::ops::Mul<Output = T>
    + core::ops::Sub<Output = T>
    + core::ops::Div<Output = T>,
    >  core::ops::Mul for #name<T> {
        type Output = Self;
        fn mul(self, rhs: Self) -> Self::Output {
            #name {
                #mul_token
            }
        }
    }
    impl <
    T: Copy + PartialEq + Debug
    + core::ops::Add<Output = T>
    + core::ops::Mul<Output = T>
    + core::ops::Sub<Output = T>
    + core::ops::Div<Output = T>,
    >  core::fmt::Display for #name<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let z = 1;
            f.write_str(&format!("< {} x: {} >", #name_str, z))
        }
    }
    ))
}
