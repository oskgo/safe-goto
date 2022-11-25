use std::collections::HashMap;

use anyhow::Context;
use proc_macro::{TokenStream, TokenTree};
use quote::{quote, quote_spanned, TokenStreamExt, ToTokens, __private::ext::RepToTokensExt};
use syn::{parse_macro_input, Expr, Ident, parse::{Parse, ParseStream, self}, Lifetime, Token, ExprArray, ExprAssign, ExprAwait, ExprBinary, ExprBox, ExprCall, ExprCast, ExprClosure, ExprAsync, ExprContinue, ExprField, ExprIndex, ExprLet, ExprMacro, ExprMethodCall, ExprPath, ExprRange, ExprReference, ExprRepeat, ExprReturn, ExprStruct, ExprTry, ExprTuple, ExprType, ExprUnary, punctuated::Punctuated, token::Comma, Label, ExprBreak, spanned::Spanned};
use proc_macro2::{Span, Group, Spacing};
use heck::AsPascalCase;


struct GotoStmt {
    goto: Ident,
    target: Lifetime,
}

impl GotoStmt {
    fn to_break(self) -> ExprContinue {
        todo!()
    }
}

impl Parse for GotoStmt {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let goto: Ident = input.parse()?;
        let lt: Lifetime = input.parse()?;
        Ok(GotoStmt{goto, target: lt})
    }
}

struct GotoBlock{group: Group}

impl ToTokens for GotoBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let group = &self.group;
        let mut stream = group.stream().into_iter();
        while let Some(token) = stream.next() {
            let tt = match token {
                proc_macro2::TokenTree::Group(grp) => {
                    let delim = grp.delimiter();
                    let strm = GotoBlock{group: grp}.to_token_stream();
                    let mut grp = Group::new(delim, strm);
                    grp.set_span(grp.span());
                    proc_macro2::TokenTree::Group(grp)
                },
                proc_macro2::TokenTree::Ident(ref ident) => {
                    if ident.to_string() == "goto" {
                        let lt: Lifetime = match syn::parse2(stream.collect()) {
                            Ok(lt) => lt,
                            Err(e) => return tokens.append_all(e.into_compile_error()),
                        };
                        todo!("Ree")
                    } else {
                        proc_macro2::TokenTree::Ident(ident.clone())
                    }
                },
                tt => tt
            };
            tokens.append(tt);
        }
    }
}

struct GotoBranch {
    lab: Label,
    e: GotoBlock
}

impl Parse for GotoBranch {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lab: Label = input.parse()?;
        let group: Group = input.parse()?;
        Ok(GotoBranch{lab, e: GotoBlock{group}})
    }
}

struct SafeGoto(Punctuated<GotoBranch, Token!(,)>);

impl SafeGoto {
    fn labels(&self) -> impl Iterator<Item = &Label> {
        self.0.iter().map(|branch| &branch.lab)
    }

    fn label_lifetimes(&self) -> impl Iterator<Item = &Lifetime> {
        self.labels().map(|label| &label.name)
    }
}

impl Parse for SafeGoto {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ret = SafeGoto(input.parse_terminated::<GotoBranch, Token!(,)>(GotoBranch::parse)?);
        let lifetimes: Vec<_> = ret.label_lifetimes().collect();
        for i in 0..lifetimes.len() {
            if lifetimes[i+1..].contains(&lifetimes[i]) {
                return Err(syn::Error::new(lifetimes[i].span(), "label occurs more than once"))
            }
        }
        Ok(ret)
    }
}

#[proc_macro]
pub fn safe_goto(t: TokenStream) -> TokenStream {
    let input = parse_macro_input!(t as SafeGoto);
    if input.label_lifetimes().find(|&lt| lt == &Lifetime::new("'begin", Span::call_site())).is_none() {
        return syn::Error::new(Span::call_site(), "missing \'begin label").to_compile_error().into();
    }
    let states_enum = Ident::new("States", Span::call_site());
    let variants: Vec<_> = input.labels().map(|ident| Ident::new(&AsPascalCase(&ident.name.to_string()).to_string() , ident.name.span())).collect();
    let blocks = input.0.into_iter().map(|branch| branch.e);
    quote!(
        enum #states_enum {
            #(#variants),*
        }
        println!("hello");
        let state = #states_enum::Begin;
        'goto: loop {
            match state {
                #(#states_enum::#variants => {#blocks}),*
            }
        }
    ).into()
}


