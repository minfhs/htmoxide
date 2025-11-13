use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, LitStr, parse::Parse, parse::ParseStream, Token};

/// Helper to extract the type name from a Type for pattern matching
fn extract_type_name(ty: &syn::Type) -> String {
    match ty {
        syn::Type::Path(type_path) => {
            // Get the last segment which contains the type name
            if let Some(segment) = type_path.path.segments.last() {
                let ident = &segment.ident;
                // Check if it has generic arguments (like State<AppState>)
                if let syn::PathArguments::AngleBracketed(_) = &segment.arguments {
                    format!("{}<", ident)
                } else {
                    ident.to_string()
                }
            } else {
                String::new()
            }
        }
        _ => String::new(),
    }
}

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

    // Parse all parameters to detect state, url_builder, and other extractors
    let mut state_type = None;
    let mut has_url_builder = false;
    let mut extractors = vec![]; // All extractors after state and url_builder

    for (idx, param) in sig.inputs.iter().enumerate() {
        if let syn::FnArg::Typed(pat_type) = param {
            let type_name = extract_type_name(&pat_type.ty);

            if idx == 0 {
                // First parameter is component state
                state_type = Some(&pat_type.ty);
            } else if type_name == "UrlBuilder" {
                has_url_builder = true;
            } else {
                // Any other parameter is an extractor (Extension, State, etc.)
                extractors.push((pat_type, &pat_type.ty));
            }
        }
    }

    let state_type = match state_type {
        Some(ty) => ty,
        None => {
            return syn::Error::new_spanned(
                sig,
                "Component function must take at least one parameter (component state)",
            )
            .to_compile_error()
            .into();
        }
    };

    // Create unique handler name
    let handler_name = syn::Ident::new(
        &format!("__htmoxide_handler_{}", fn_name),
        fn_name.span(),
    );

    // Build component function call with all parameters
    let mut call_args = vec![quote! { state }];
    if has_url_builder {
        call_args.push(quote! { url_builder });
    }
    // Add all extractor arguments
    for (idx, _) in extractors.iter().enumerate() {
        let extractor_name = syn::Ident::new(&format!("extractor_{}", idx), fn_name.span());
        call_args.push(quote! { #extractor_name });
    }

    let call_component = quote! {
        let result = #fn_name(#(#call_args),*).await;
    };

    // Generate extraction code for all extractors
    let extractor_extractions: Vec<_> = extractors.iter().enumerate().map(|(idx, (_pat, ty))| {
        let extractor_name = syn::Ident::new(&format!("extractor_{}", idx), fn_name.span());
        quote! {
            // Extract additional parameter
            let #extractor_name = match #ty::from_request_parts(&mut parts, &()).await {
                Ok(v) => v,
                Err(e) => {
                    let error_msg = format!("Failed to extract parameter. Did you forget to add it via .layer()? Error: {:?}", e);
                    return ::axum::http::Response::builder()
                        .status(500)
                        .body(::axum::body::Body::from(error_msg))
                        .unwrap()
                        .into_response();
                }
            };
        }
    }).collect();


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

                // Extract and own the query string
                let query_string = parts.uri.query().unwrap_or("").to_string();

                // Extract the current page path from HX-Current-URL header (for htmx requests)
                let main_page_path = parts.headers
                    .get("HX-Current-URL")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|url| url.split('?').next())
                    .map(|path| path.to_string());

                // Extract state from query parameters
                let state = match ::htmoxide::StateExtractor::<#state_type>::from_request_parts(
                    &mut parts,
                    &(),
                ).await {
                    Ok(extractor) => extractor.0,
                    Err(_) => #state_type::default(),
                };

                // Create URL builder with main page path if available
                let url_builder = if let Some(page_path) = main_page_path {
                    ::htmoxide::UrlBuilder::new(#route_path, &query_string).with_main_page(page_path)
                } else {
                    ::htmoxide::UrlBuilder::new(#route_path, &query_string)
                };

                // Extract all additional extractors
                #(#extractor_extractions)*

                // Call the component function
                #call_component
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
