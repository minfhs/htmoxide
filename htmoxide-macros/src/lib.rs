use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemFn, LitStr, Token, parse::Parse, parse::ParseStream, parse_macro_input};

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
/// - `#[component]` - auto-generates route /function_name (GET)
/// - `#[component("/path")]` - explicit route path (GET)
/// - `#[component(prefix = "/api")]` - generates route /api/function_name (GET)
/// - `#[component(method = "POST")]` - auto-generates route with POST method
/// - `#[component(prefix = "/api", method = "POST")]` - route /api/function_name with POST
/// - `#[component(prefix = "/todos", path = "/{id}/toggle")]` - route /todos/{id}/toggle
/// - `#[component(path = "/{id}")]` - explicit path (no prefix)
#[proc_macro_attribute]
pub fn component(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input_fn = parse_macro_input!(item as ItemFn);
    let fn_name = &input_fn.sig.ident;
    let fn_name_str = fn_name.to_string();

    // Parse the attribute for route configuration
    let (route_path, http_method) = if attr.is_empty() {
        // Auto-generate: /function_name with GET
        (format!("/{}", fn_name_str), "GET".to_string())
    } else {
        let attr_str = attr.to_string();

        if attr_str.starts_with('"') {
            // Explicit path: #[component("/users")]
            let lit: LitStr = parse_macro_input!(attr as LitStr);
            (lit.value(), "GET".to_string())
        } else if attr_str.contains("prefix")
            || attr_str.contains("method")
            || attr_str.contains("path")
        {
            // Parse component args: #[component(prefix = "/api", method = "POST", path = "/{id}")]
            let args = parse_macro_input!(attr as ComponentArgs);

            // Build final path: {prefix}{path} or {prefix}/{fn_name} or /{fn_name}
            let final_path = match (args.prefix, args.path) {
                (Some(prefix), Some(path)) => {
                    // Both prefix and path: concatenate them
                    format!("{}{}", prefix.value(), path.value())
                }
                (Some(prefix), None) => {
                    // Only prefix: append function name
                    format!("{}/{}", prefix.value(), fn_name_str)
                }
                (None, Some(path)) => {
                    // Only path: use it directly
                    path.value()
                }
                (None, None) => {
                    // Neither: auto-generate from function name
                    format!("/{}", fn_name_str)
                }
            };

            let method = args
                .method
                .map(|m| m.value())
                .unwrap_or_else(|| "GET".to_string());
            (final_path, method)
        } else {
            (format!("/{}", fn_name_str), "GET".to_string())
        }
    };

    let vis = &input_fn.vis;
    let sig = &input_fn.sig;
    let block = &input_fn.block;
    let attrs = &input_fn.attrs;

    // NEW DESIGN: Enforce first two mandatory parameters, then pass through everything else
    // Position 0: ViewState (any type, auto-hydrated from query + cookies)
    // Position 1: UrlBuilder (injected by macro)
    // Position 2+: Any Axum extractors (zero validation, pass-through)

    if sig.inputs.len() < 2 {
        return syn::Error::new_spanned(
            sig,
            "Component function must have at least 2 parameters: (ViewState, UrlBuilder, ...extractors)",
        )
        .to_compile_error()
        .into();
    }

    let params = sig.inputs.iter().collect::<Vec<_>>();

    // Position 0: ViewState
    let state_type = if let syn::FnArg::Typed(pat_type) = params[0] {
        &pat_type.ty
    } else {
        return syn::Error::new_spanned(params[0], "First parameter must be the view state type")
            .to_compile_error()
            .into();
    };

    // Position 1: UrlBuilder (validate it's UrlBuilder)
    let url_builder_valid = if let syn::FnArg::Typed(pat_type) = params[1] {
        extract_type_name(&pat_type.ty) == "UrlBuilder"
    } else {
        false
    };

    if !url_builder_valid {
        return syn::Error::new_spanned(params[1], "Second parameter must be UrlBuilder")
            .to_compile_error()
            .into();
    }

    // Position 2+: Collect all remaining extractors (no validation)
    let extractors: Vec<_> = params[2..]
        .iter()
        .enumerate()
        .map(|(idx, param)| {
            if let syn::FnArg::Typed(pat_type) = param {
                (idx + 2, pat_type, &pat_type.ty)
            } else {
                panic!("Unexpected parameter type");
            }
        })
        .collect();

    // Create unique handler name
    let handler_name = syn::Ident::new(&format!("__htmoxide_handler_{}", fn_name), fn_name.span());

    // Create PascalCase marker type name for ComponentName trait
    let marker_type_name = {
        let fn_name_str = fn_name.to_string();
        let pascal_case = to_pascal_case(&fn_name_str);
        syn::Ident::new(&pascal_case, fn_name.span())
    };

    // Build component function call with all parameters in ORIGINAL order
    let total_params = sig.inputs.len();
    let mut call_args = Vec::with_capacity(total_params);

    // Position 0: state
    call_args.push(quote! { state });

    // Position 1: url_builder
    call_args.push(quote! { url_builder });

    // Position 2+: extractors
    for (original_idx, _, _) in &extractors {
        let extractor_name = syn::Ident::new(&format!("param_{}", original_idx), fn_name.span());
        call_args.push(quote! { #extractor_name });
    }

    let call_component = quote! {
        let result = #fn_name(#(#call_args),*).await;
    };

    // Generate extraction code for all extractors
    // All but the last use FromRequestParts only
    // The last parameter can use either FromRequestParts OR FromRequest (for Form, Json, etc.)

    let num_extractors = extractors.len();

    let parts_extractors: Vec<_> = if num_extractors > 1 {
        extractors[..num_extractors - 1].iter().map(|(param_idx, _pat, ty)| {
            let extractor_name = syn::Ident::new(&format!("param_{}", param_idx), fn_name.span());
            quote! {
                // Extract from request parts (does not consume body)
                let #extractor_name = match <#ty as ::axum::extract::FromRequestParts<()>>::from_request_parts(&mut parts, &()).await {
                    Ok(v) => v,
                    Err(e) => {
                        return ::axum::response::IntoResponse::into_response((
                            ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to extract parameter {}: {:?}", stringify!(#ty), e),
                        ));
                    }
                };
            }
        }).collect()
    } else {
        vec![]
    };

    // Last extractor: Check if it's Body<T> wrapper for body extraction
    let last_extractor = if num_extractors > 0 {
        let (param_idx, _pat, ty) = &extractors[num_extractors - 1];
        let extractor_name = syn::Ident::new(&format!("param_{}", param_idx), fn_name.span());
        let type_name = extract_type_name(ty);

        // Check if this is a Body<T> wrapper (for Form, Json, etc.)
        if type_name.starts_with("Body<") {
            quote! {
                // Body<T> extractor: use FromRequest on the request body
                let req = ::axum::http::Request::from_parts(parts, body);
                let #extractor_name = match <#ty as ::axum::extract::FromRequest<()>>::from_request(req, &()).await {
                    Ok(v) => v,
                    Err(e) => {
                        return ::axum::response::IntoResponse::into_response((
                            ::axum::http::StatusCode::BAD_REQUEST,
                            format!("Failed to extract body parameter {}: {:?}", stringify!(#ty), e),
                        ));
                    }
                };
            }
        } else {
            quote! {
                // Regular extractor: use FromRequestParts
                let #extractor_name = match <#ty as ::axum::extract::FromRequestParts<()>>::from_request_parts(&mut parts, &()).await {
                    Ok(v) => v,
                    Err(e) => {
                        return ::axum::response::IntoResponse::into_response((
                            ::axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                            format!("Failed to extract parameter {}: {:?}", stringify!(#ty), e),
                        ));
                    }
                };
            }
        }
    } else {
        quote! {}
    };

    // Keep the original component function as-is (no wrapper needed)
    let component_function = quote! {
        #(#attrs)*
        #vis #sig {
            #block
        }
    };

    let output = quote! {
        // Original component function
        #component_function

        // Generate axum handler wrapper
        #[doc(hidden)]
        #vis fn #handler_name(
            req: ::axum::http::Request<::axum::body::Body>,
        ) -> ::std::pin::Pin<Box<dyn ::std::future::Future<Output = ::axum::response::Response> + Send>> {
            Box::pin(async move {
                use ::axum::extract::{FromRequestParts, FromRequest};
                use ::axum::response::IntoResponse;

                let (mut parts, body) = req.into_parts();

                // POSITION 0: Extract ViewState
                // Auto-hydrate from query params (+ cookies if persist-state feature enabled)
                let query_string = parts.uri.query().unwrap_or("").to_string();

                // Extract state from query parameters
                let mut state = match ::htmoxide::StateExtractor::<#state_type>::from_request_parts(
                    &mut parts,
                    &(),
                ).await {
                    Ok(extractor) => extractor.0,
                    Err(_) => #state_type::default(),
                };

                // Cookie hydration when persist-state feature is enabled
                #[cfg(feature = "persist-state")]
                {
                    // Extract cookies for state persistence
                    if let Ok(cookies) = ::htmoxide::tower_cookies::Cookies::from_request_parts(&mut parts, &()).await {
                        // Merge cookie values into state (query params take priority)
                        if let (Ok(default_json), Ok(mut state_json)) = (
                            ::htmoxide::serde_json::to_value(&#state_type::default()),
                            ::htmoxide::serde_json::to_value(&state)
                        ) {
                            if let (Some(default_obj), Some(state_obj)) = (
                                default_json.as_object(),
                                state_json.as_object_mut()
                            ) {
                                for (key, default_value) in default_obj {
                                    if let Some(current_value) = state_obj.get(key) {
                                        if current_value == default_value {
                                            if let Some(cookie) = cookies.get(key) {
                                                let cookie_value = cookie.value();
                                                let parsed_value = if let Ok(num) = cookie_value.parse::<i64>() {
                                                    Some(::htmoxide::serde_json::Value::Number(num.into()))
                                                } else if let Ok(num) = cookie_value.parse::<f64>() {
                                                    ::htmoxide::serde_json::Number::from_f64(num)
                                                        .map(::htmoxide::serde_json::Value::Number)
                                                } else if let Ok(b) = cookie_value.parse::<bool>() {
                                                    Some(::htmoxide::serde_json::Value::Bool(b))
                                                } else if !cookie_value.is_empty() {
                                                    Some(::htmoxide::serde_json::Value::String(cookie_value.to_string()))
                                                } else {
                                                    None
                                                };
                                                if let Some(val) = parsed_value {
                                                    state_obj.insert(key.clone(), val);
                                                }
                                            }
                                        }
                                    }
                                }
                                if let Ok(new_state) = ::htmoxide::serde_json::from_value(state_json) {
                                    state = new_state;
                                }
                            }
                        }

                        // Save current state to cookies for persistence
                        if let Ok(state_json) = ::htmoxide::serde_json::to_value(&state) {
                            if let ::htmoxide::serde_json::Value::Object(ref obj) = state_json {
                                for (key, value) in obj {
                                    let cookie_value = if let Some(value_str) = value.as_str() {
                                        Some(value_str.to_string())
                                    } else if let Some(value_num) = value.as_i64() {
                                        Some(value_num.to_string())
                                    } else if let Some(value_num) = value.as_f64() {
                                        Some(value_num.to_string())
                                    } else if let Some(value_bool) = value.as_bool() {
                                        Some(value_bool.to_string())
                                    } else {
                                        None
                                    };
                                    if let Some(val) = cookie_value {
                                        if val.is_empty() {
                                            cookies.remove(::htmoxide::tower_cookies::Cookie::from(key.to_string()));
                                        } else {
                                            let mut cookie = ::htmoxide::tower_cookies::Cookie::new(key.to_string(), val);
                                            cookie.set_path("/");
                                            cookies.add(cookie);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }

                // POSITION 1: Extract UrlBuilder
                let main_page_path = parts.headers
                    .get("HX-Current-URL")
                    .and_then(|v| v.to_str().ok())
                    .and_then(|url| url.split('?').next())
                    .map(|path| path.to_string());

                let url_builder = if let Some(page_path) = main_page_path {
                    ::htmoxide::UrlBuilder::new(#route_path, &query_string).with_main_page(page_path)
                } else {
                    ::htmoxide::UrlBuilder::new(#route_path, &query_string)
                };

                // POSITIONS 2+: Extract all additional Axum extractors
                // All but last use FromRequestParts, last can use FromRequest (Form, Json)
                #(#parts_extractors)*

                // Last extractor (supports Form, Json, etc.)
                #last_extractor

                // Call the component function with all parameters
                #call_component
                result.into_response()
            })
        }

        // Zero-sized marker type for this component (for type-safe URL building)
        #vis struct #marker_type_name;

        // Implement ComponentName trait for type-safe component references
        impl ::htmoxide::ComponentName for #marker_type_name {
            fn name() -> &'static str {
                stringify!(#fn_name)
            }
        }

        // Register component in global registry
        ::htmoxide::inventory::submit! {
            ::htmoxide::ComponentInfo::new(
                stringify!(#fn_name),
                #route_path,
                #handler_name,
                #http_method,
            )
        }
    };

    output.into()
}

/// Parse component arguments: prefix = "/api", method = "POST", path = "/{id}/action"
struct ComponentArgs {
    prefix: Option<LitStr>,
    method: Option<LitStr>,
    path: Option<LitStr>,
}

impl Parse for ComponentArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut prefix = None;
        let mut method = None;
        let mut path = None;

        // Parse comma-separated key = "value" pairs
        while !input.is_empty() {
            let key: syn::Ident = input.parse()?;
            let _eq: Token![=] = input.parse()?;
            let value: LitStr = input.parse()?;

            match key.to_string().as_str() {
                "prefix" => prefix = Some(value),
                "method" => method = Some(value),
                "path" => path = Some(value),
                _ => return Err(syn::Error::new(key.span(), "Unknown component attribute")),
            }

            // Parse optional comma
            if input.peek(Token![,]) {
                let _comma: Token![,] = input.parse()?;
            }
        }

        Ok(ComponentArgs {
            prefix,
            method,
            path,
        })
    }
}

/// Convert snake_case to PascalCase
fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect()
}
