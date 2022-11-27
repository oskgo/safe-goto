use heck::AsPascalCase;
use proc_macro::TokenStream;
use proc_macro2::{Delimiter, Group, Span};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::{
    parse::{Parse, ParseStream, Parser},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    FnArg, Ident, Pat, PatType, Token,
};

fn pascalize(ident: &Ident) -> Ident {
    Ident::new(&AsPascalCase(&ident.to_string()).to_string(), ident.span())
}

#[derive(Debug)]
struct GotoBlockContents(proc_macro2::TokenStream);

impl Parse for GotoBlockContents {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = proc_macro2::TokenStream::new();
        while let Ok(token) = input.parse::<proc_macro2::TokenTree>() {
            let tt = match token {
                proc_macro2::TokenTree::Group(grp) => {
                    let delim = grp.delimiter();
                    let span = grp.span();
                    let contents: GotoBlockContents = syn::parse2(grp.stream())?;
                    let mut grp = Group::new(delim, contents.0);
                    grp.set_span(span);
                    proc_macro2::TokenTree::Group(grp)
                }
                proc_macro2::TokenTree::Ident(ref ident) => {
                    if ident == "goto" {
                        let id: Ident = input.parse().map_err(|e| {
                            syn::Error::new(e.span(), "Invalid syntax for goto statement")
                        })?;
                        let variant = pascalize(&id).clone();
                        let call: Group = input.parse()?;
                        if call.delimiter() != Delimiter::Parenthesis {
                            return Err(syn::Error::new(call.span_open(), "expected `(`"));
                        }
                        let call = if call.stream().is_empty() {
                            proc_macro2::TokenStream::new()
                        } else {
                            quote!(#call)
                        };
                        syn::parse2(quote!(
                            {
                                goto = States::#variant #call;
                                continue 'goto
                            }
                        ))
                        .expect("This should parse as a group")
                    } else if ident == "safe_goto" {
                        return Err(syn::Error::new(
                            ident.span(),
                            "using safe_goto inside safe_goto is not allowed",
                        ));
                    } else {
                        proc_macro2::TokenTree::Ident(ident.clone())
                    }
                }
                tt => tt,
            };
            tokens.append(tt);
        }
        Ok(GotoBlockContents(tokens))
    }
}

impl ToTokens for GotoBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let GotoBlock {
            contents,
            delimiter,
        } = self;
        tokens.append(Group::new(*delimiter, contents.0.clone()));
    }
}

#[derive(Debug)]
struct GotoBlock {
    delimiter: Delimiter,
    contents: GotoBlockContents,
}

impl From<GotoBlock> for Group {
    fn from(gtb: GotoBlock) -> Self {
        Group::new(gtb.delimiter, gtb.contents.0)
    }
}

impl Parse for GotoBlock {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group: Group = input.parse()?;
        let delimiter = group.delimiter();
        let contents: GotoBlockContents = syn::parse2(group.stream())?;
        Ok(GotoBlock {
            delimiter,
            contents,
        })
    }
}

struct VariantArgsDelimited {
    contents: Punctuated<PatType, Token!(,)>,
}

impl Parse for VariantArgsDelimited {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group: Group = input.parse()?;
        let contents = if group.delimiter() == Delimiter::Parenthesis {
            let parser = Punctuated::<FnArg, Token![,]>::parse_terminated;
            parser.parse2(group.stream())?
        } else {
            return Err(syn::Error::new(group.span_open(), "expected `(`"));
        };
        let mut new_contents = Punctuated::<PatType, Token!(,)>::new();
        for pair in contents.pairs() {
            if let FnArg::Typed(pat) = pair.value() {
                new_contents.push_value(pat.clone())
            } else {
                return Err(syn::Error::new(contents.span(), "unexpected `self`"));
            }
            if let Some(&&punct) = pair.punct() {
                new_contents.push_punct(punct)
            }
        }
        Ok(VariantArgsDelimited {
            contents: new_contents,
        })
    }
}

impl ToTokens for VariantArgsDelimited {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.contents.is_empty() {
            let args = &self.contents;
            tokens.append_all(quote!(
                (#args)
            ))
        }
    }
}

struct GotoBranch {
    id: Ident,
    block: GotoBlock,
    variant_args: VariantArgsDelimited,
}

impl Parse for GotoBranch {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let id = input.parse()?;
        let variant_args = input.parse()?;
        let block = input.parse()?;
        Ok(GotoBranch {
            id,
            block,
            variant_args,
        })
    }
}

struct SafeGoto(Punctuated<GotoBranch, Token!(,)>);

struct VariantTypesDelimited {
    contents: Punctuated<Box<syn::Type>, Token!(,)>,
}

impl ToTokens for VariantTypesDelimited {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.contents.is_empty() {
            let args = &self.contents;
            tokens.append_all(quote!(
                (#args)
            ))
        }
    }
}

struct VariantPatsDelimited {
    contents: Punctuated<Box<Pat>, Token!(,)>,
}

impl ToTokens for VariantPatsDelimited {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        if !self.contents.is_empty() {
            let args = &self.contents;
            tokens.append_all(quote!(
                (#args)
            ))
        }
    }
}

impl SafeGoto {
    fn idents(&self) -> impl Iterator<Item = &Ident> {
        self.0.iter().map(|branch| &branch.id)
    }

    fn variant_types(&self) -> impl Iterator<Item = VariantTypesDelimited> + '_ {
        self.0.iter().map(|branch| {
            let mut ret = Punctuated::new();
            for pair in branch.variant_args.contents.pairs() {
                ret.push_value(pair.value().ty.clone());
                if let Some(&&punct) = pair.punct() {
                    ret.push_punct(punct)
                }
            }
            VariantTypesDelimited { contents: ret }
        })
    }

    fn variant_pats(&self) -> impl Iterator<Item = VariantPatsDelimited> + '_ {
        self.0.iter().map(|branch| {
            let mut ret = Punctuated::new();
            for pair in branch.variant_args.contents.pairs() {
                ret.push_value(pair.value().pat.clone());
                if let Some(&&punct) = pair.punct() {
                    ret.push_punct(punct)
                }
            }
            VariantPatsDelimited { contents: ret }
        })
    }

    fn blocks(&self) -> impl Iterator<Item = &GotoBlock> {
        self.0.iter().map(|branch| &branch.block)
    }
}

impl Parse for SafeGoto {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ret = SafeGoto(input.parse_terminated::<GotoBranch, Token!(,)>(GotoBranch::parse)?);
        let lifetimes: Vec<_> = ret.idents().collect();
        for i in 0..lifetimes.len() {
            if lifetimes[i + 1..].contains(&lifetimes[i]) {
                return Err(syn::Error::new(
                    lifetimes[i].span(),
                    "label occurs more than once",
                ));
            }
        }
        Ok(ret)
    }
}

#[proc_macro]
pub fn safe_goto(t: TokenStream) -> TokenStream {
    let input = parse_macro_input!(t as SafeGoto);
    if !input.idents().any(|id| id == "begin") {
        return syn::Error::new(Span::call_site(), "missing \'begin label")
            .to_compile_error()
            .into();
    }
    let states_enum = Ident::new("States", Span::call_site());
    let variants: Vec<_> = input.idents().map(pascalize).collect();
    let variant_pats = input.variant_pats();
    let variant_types = input.variant_types();
    let blocks = input.blocks();
    quote!(
        {
            enum #states_enum {
                #(#variants #variant_types),*
            }

            let mut goto = #states_enum::Begin;
            'goto: loop {
                break match goto {
                    #(#states_enum::#variants #variant_pats => #blocks),*
                }
            }
        }
    )
    .into()
}
