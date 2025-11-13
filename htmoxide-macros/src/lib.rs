use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, parse::Parse, parse::ParseStream, Token};

/// Attribute macro for defining components
///
/// Usage:
/// - `#[component]` - auto-generates route /function_name
/// - `#[component("/path")]` - explicit route path
/// - `#[component(prefix = "/api")]` - generates route /api/function_name
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    // Parse the attribute for route configuration
    let route_path = if attr.is_empty() {
        // Auto-generate: /function_name
        format!("/{}", fn_name_str)
    } else {
        let attr_str = attr.to_string();

        if attr_str.starts_with('"') {
            // Explicit path: #[component("/users")]
            let lit: LitStr = parse_macro_input!(attr as LitStr);
            lit.value()
        } else if attr_str.contains("prefix") {
            // Prefix: #[component(prefix = "/api")]
            let args = parse_macro_input!(attr as PrefixArgs);
            format!("{}/{}", args.prefix.value(), fn_name_str)
        } else {
            format!("/{}", fn_name_str)
        }
    };

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    // Extract state type from function signature
    let state_type = if let Some(syn::FnArg::Typed(pat_type)) = sig.inputs.first() {
        &pat_type.ty
    } else {
        return syn::Error::new_spanned(
            sig,
            "Component function must take at least one parameter (state)",
        )
        .to_compile_error()
        .into();
    };

    // Create unique handler name
    let handler_name = syn::Ident::new(
        &format!("__htmoxide_handler_{}", fn_name),
        fn_name.span(),
    );

    let output = quote! {
        // Original function
        #(#attrs)*
        #vis #sig {
            #block
        }

        // Generate axum handler wrapper
        #[doc(hidden)]
        #vis fn #handler_name(
            req: ::axum::http::Request<::axum::body::Body>,
        ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::axum::response::Response> + Send>> {
            Box::pin(async move {
                use ::axum::extract::FromRequestParts;
                use ::axum::response::IntoResponse;

                let (mut parts, _body) = req.into_parts();

                // Extract state from query parameters
                let state = match ::htmoxide::StateExtractor::<#state_type>::from_request_parts(
                    &mut parts,
                    &(),
                ).await {
                    Ok(extractor) => extractor.0,
                    Err(_) => #state_type::default(),
                };

                // Call the component function
                let result = #fn_name(state).await;
                result.into_response()
            })
        }

        // Register component in global registry
        ::htmoxide::inventory::submit! {
            ::htmoxide::ComponentInfo::new(
                stringify!(#fn_name),
                #route_path,
                #handler_name,
            )
        }
    };

    output.into()
}

/// Parse prefix = "/api" style arguments
struct PrefixArgs {
    prefix: LitStr,
}

impl Parse for PrefixArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let _prefix_ident: syn::Ident = input.parse()?;
        let _eq: Token![=] = input.parse()?;
        let prefix: LitStr = input.parse()?;
        Ok(PrefixArgs { prefix })
    }
}

/// Macro for generating component URLs with state merging
///
/// Usage:
/// - `component_url!()` - current component, current state
/// - `component_url!(param = value)` - current component, merge param
/// - `component_url!(other_fn, param = value)` - different component
#[proc_macro]
pub fn component_url(_input: TokenStream) -> TokenStream {
    // For now, just return a placeholder
    // This will need runtime support to merge state properly
    let output = quote! {
        "#"
    };
    output.into()
}
