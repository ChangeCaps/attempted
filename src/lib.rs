use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse_quote, parse_quote_spanned, spanned::Spanned, ItemFn};

/// Wraps the function block such that it returns an [`Option`].
/// If [`None`] is returned, the function will return the [`Default`] value for the return type.
///
/// # Examples
/// ```rust
/// # use attempted::attempt;
/// fn positive(x: i32) -> Option<i32> {
///     if x > 0 {
///         Some(x)
///     } else {
///         None
///     }
/// }
///
/// #[attempt]
/// fn test() {
///     // try something
///     let x = positive(13)?;
///     
///     // do something with x
///     println!("{} is positive", x);
/// }
/// ```
#[proc_macro_attribute]
pub fn attempt(
    _args: proc_macro::TokenStream,
    input: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);

    let block = &input.block;
    let return_type = match input.sig.output {
        syn::ReturnType::Default => parse_quote!(()),
        syn::ReturnType::Type(_, ref ty) => ty.clone(),
    };

    let return_type_default_check = quote_spanned!(return_type.span()=>
        #[allow(unused)]
        struct CheckReturnTypeDefault
        where
            #return_type: ::std::default::Default;
    );

    input.block = parse_quote_spanned!(block.span()=>{
        #return_type_default_check

        (|| ::std::option::Option::Some(#block))()
            .unwrap_or(<#return_type as ::std::default::Default>::default())
    });

    let expanded = quote! {
        #input
    };

    expanded.into()
}
