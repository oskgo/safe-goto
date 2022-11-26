use proc_macro::TokenStream;
use quote::{quote, TokenStreamExt, ToTokens};
use syn::{parse_macro_input, Ident, parse::{Parse, ParseStream}, Lifetime, Token, punctuated::Punctuated, Label};
use proc_macro2::{Span, Group, Delimiter};
use heck::AsPascalCase;

fn pascalize(ident: &Ident) -> Ident {
    Ident::new(&AsPascalCase(&ident.to_string()).to_string() , ident.span())
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
                },
                proc_macro2::TokenTree::Ident(ref ident) => {
                    if ident.to_string() == "goto" {
                        println!("goto_found");
                        let lt: Lifetime = input.parse().map_err(|e| syn::Error::new(e.span(), "Invalid syntax for goto statement"))?;
                        let variant = pascalize(&lt.ident).clone();
                        syn::parse2(quote!(
                            {
                                goto = States::#variant;
                                continue 'goto
                            }
                        )).expect("This should parse as a group")
                    } else if ident.to_string() == "safe_goto" {
                        return Err(syn::Error::new(ident.span(), "using safe_goto inside safe_goto is not allowed"));
                    } else {
                        proc_macro2::TokenTree::Ident(ident.clone())
                    }
                },
                tt => tt
            };
            println!("{tt:?}");
            tokens.append(tt);
        }
        Ok(GotoBlockContents(tokens))
    }
}

impl ToTokens for GotoBlock {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let GotoBlock{contents, delimiter} = self;
        tokens.append(Group::new(delimiter.clone(), contents.0.clone()));
    }
}

#[derive(Debug)]
struct GotoBlock{delimiter: Delimiter, contents: GotoBlockContents}

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
        Ok(GotoBlock{delimiter, contents})
    }
}

struct GotoBranch {
    lab: Label,
    e: GotoBlock
}

impl Parse for GotoBranch {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lab: Label = input.parse()?;
        let e: GotoBlock = input.parse()?;
        Ok(GotoBranch{lab, e})
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
    let variants: Vec<_> = input.labels().map(|label| pascalize(&label.name.ident)).collect();
    let blocks = input.0.into_iter().map(|branch| branch.e);
    quote!(
        {
            enum #states_enum {
                #(#variants),*
            }

            let mut goto = #states_enum::Begin;
            'goto: loop {
                break match goto {
                    #(#states_enum::#variants => #blocks),*
                }
            }
        }
    ).into()
}


