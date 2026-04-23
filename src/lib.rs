use proc_macro::TokenStream;
use quote::quote;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn include_minified(input: TokenStream) -> TokenStream {
    let path_lit = parse_macro_input!(input as LitStr);
    let path_str = path_lit.value();

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR not set");
    let full_path = std::path::Path::new(&manifest_dir).join(&path_str);
    let full_path_str = full_path.to_str().unwrap();

    let content = std::fs::read_to_string(&full_path).unwrap_or_else(|e| {
        panic!(
            "include_minified!: cannot read `{}`: {}",
            full_path.display(),
            e
        )
    });

    let minified: String = match extension(&path_str) {
        "html" | "htm" => minify_html(&content),
        _ => content,
    };

    quote! {{
        const _: &[u8] = include_bytes!(#full_path_str);
        #minified
    }}
    .into()
}

fn extension(path: &str) -> &str {
    path.rsplit('.').next().unwrap_or("").trim()
}

fn minify_html(src: &str) -> String {
    let cfg = minify_html::Cfg {
        keep_closing_tags: true,
        minify_css: true,
        minify_js: true,
        ..minify_html::Cfg::default()
    };
    let result = minify_html::minify(src.as_bytes(), &cfg);
    String::from_utf8(result).expect("include_minified!: minify_html produced non-UTF-8 output")
}
