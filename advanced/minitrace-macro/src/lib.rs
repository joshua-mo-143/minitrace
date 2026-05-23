use proc_macro::TokenStream;
use quote::quote;
use syn::{Expr, Ident, LitStr, Token, parse::Parse, parse_macro_input};

#[proc_macro]
pub fn span(item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(item as MinitraceSpanArgs);
    let span_name = args.name;
    let line = args.line;
    let col = args.col;
    let field_names = args.fields.iter().map(|field| {
        let name = field.name.to_string();
        syn::LitStr::new(&name, field.name.span())
    });
    let field_values = args.fields.iter().map(|field| {
        let name = field.name.to_string();
        let name = syn::LitStr::new(&name, field.name.span());
        let value = &field.val.0;
        quote! {
            (#name, &#value)
        }
    });
    let span_name_for_runtime = match &span_name {
        Some(name) => quote! { #name },
        None => quote! { module_path!() },
    };
    let span_name_for_metadata = match &span_name {
        Some(name) => quote! { Some(#name) },
        None => quote! { None },
    };
    quote! {{
        static CALLSITE: ::std::sync::OnceLock<::minitrace_core::callsite::Callsite> =
            ::std::sync::OnceLock::new();
        let callsite = CALLSITE.get_or_init(|| {
            ::minitrace_core::callsite::Callsite::register(
                ::minitrace_core::metadata::Metadata {
                    name: #span_name_for_metadata,
                    fields: &[#(#field_names),*],
                    file: file!(),
                    line: #line,
                    column: #col,
                    module_path: module_path!(),
                    kind: ::minitrace_core::metadata::Kind::Span,
                }
            )
        });
        if ::minitrace_core::subscriber::REGISTRY
            .get()
            .is_some_and(|registry| registry.enabled(callsite.metadata()))
        {
            let _fields = [#(#field_values),*];
            ::minitrace_core::span::Span::new(#span_name_for_runtime)
        } else {
            ::minitrace_core::span::Span::disabled()
        }
    }}
    .into()
}

struct MinitraceSpanArgs {
    name: Option<LitStr>,
    fields: Vec<Box<Field>>,
    line: u64,
    col: u64,
}

impl Parse for MinitraceSpanArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let syn_span = input.span().start();

        let mut name = None;
        let mut fields = Vec::new();

        while !input.is_empty() {
            let arg: SpanArg = input.parse()?;

            match arg {
                SpanArg::Name(expr) if name.is_none() => {
                    name = Some(expr);
                }
                SpanArg::Name(_) => return Err(input.error("Expected field after span name")),
                SpanArg::Field(field) => fields.push(field),
            }

            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        Ok(Self {
            name,
            fields,
            line: syn_span.line as u64,
            col: syn_span.column as u64,
        })
    }
}

enum SpanArg {
    Name(LitStr),
    // Box<T> required due to large diff in size
    Field(Box<Field>),
}

impl Parse for SpanArg {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        if input.peek(Ident) && input.peek2(Token![=]) {
            Ok(Self::Field(input.parse()?))
        } else {
            Ok(Self::Name(input.parse()?))
        }
    }
}

struct Field {
    name: Ident,
    _token: Token![=],
    val: Value,
}

impl Parse for Field {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            _token: input.parse()?,
            val: input.parse()?,
        })
    }
}

struct Value(Expr);

impl Parse for Value {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        Ok(Self(input.parse()?))
    }
}
