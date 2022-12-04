use std::marker::PhantomData;

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

trait GotoSemantics {
    fn transform_goto(input: ParseStream) -> syn::Result<proc_macro2::TokenTree>;
}

#[derive(Debug)]
struct GotoBlockContents<T: GotoSemantics> {
    stream: proc_macro2::TokenStream,
    goto_semantics: PhantomData<T>,
}

impl<T: GotoSemantics> Parse for GotoBlockContents<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut tokens = proc_macro2::TokenStream::new();
        while let Ok(token) = input.parse::<proc_macro2::TokenTree>() {
            let tt = match token {
                proc_macro2::TokenTree::Group(grp) => {
                    let delim = grp.delimiter();
                    let span = grp.span();
                    let contents: GotoBlockContents<T> = syn::parse2(grp.stream())?;
                    let mut grp = Group::new(delim, contents.stream);
                    grp.set_span(span);
                    proc_macro2::TokenTree::Group(grp)
                }
                proc_macro2::TokenTree::Ident(ref ident) => {
                    if ident == "goto" {
                        T::transform_goto(input)?
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
        Ok(GotoBlockContents {
            stream: tokens,
            goto_semantics: PhantomData,
        })
    }
}

impl<T: GotoSemantics> ToTokens for GotoBlock<T> {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let GotoBlock {
            contents,
            delimiter,
        } = self;
        tokens.append(Group::new(*delimiter, contents.stream.clone()));
    }
}

/// A possibly invalid Rust block possibly containing goto statements
#[derive(Debug)]
struct GotoBlock<T: GotoSemantics> {
    delimiter: Delimiter,
    contents: GotoBlockContents<T>,
}

impl<T: GotoSemantics> From<GotoBlock<T>> for Group {
    fn from(gtb: GotoBlock<T>) -> Self {
        Group::new(gtb.delimiter, gtb.contents.stream)
    }
}

impl<T: GotoSemantics> Parse for GotoBlock<T> {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let group: Group = input.parse()?;
        let delimiter = group.delimiter();
        let contents: GotoBlockContents<T> = syn::parse2(group.stream())?;
        Ok(GotoBlock {
            delimiter,
            contents,
        })
    }
}

/// Comma separated list of typed patterns used as arguments for each goto block
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

/// A branch that can be a target of a goto statement
struct GotoBranch<T: GotoSemantics> {
    id: Ident,
    block: GotoBlock<T>,
    variant_args: VariantArgsDelimited,
}

impl<T: GotoSemantics> Parse for GotoBranch<T> {
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

/// Comma separated list of types that are arguments to a goto branch. Used for constructing enum
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

/// Comma separated list of patterns that are inputs to a goto branch. Used for matching
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

/// Strategy for transforming goto statements in the begin branch
struct Initial;
impl GotoSemantics for Initial {
    fn transform_goto(input: ParseStream) -> syn::Result<proc_macro2::TokenTree> {
        let id: Ident = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Invalid syntax for goto statement"))?;
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
        Ok(syn::parse2(quote!(
            {
                break 'goto States::#variant #call
            }
        ))
        .expect("This should parse as a group"))
    }
}

/// Strategy for transforming goto "statements" in the proper goto branches
struct Other;
impl GotoSemantics for Other {
    fn transform_goto(input: ParseStream) -> syn::Result<proc_macro2::TokenTree> {
        let id: Ident = input
            .parse()
            .map_err(|e| syn::Error::new(e.span(), "Invalid syntax for goto statement"))?;
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
        Ok(syn::parse2(quote!(
            {
                goto = States::#variant #call;
                continue 'goto
            }
        ))
        .expect("This should parse as a group"))
    }
}

/// half-parsed valid input of the `safe_goto` macro
struct SafeGoto {
    begin_branch: GotoBranch<Initial>,
    branches: Punctuated<GotoBranch<Other>, Token!(,)>,
}

impl SafeGoto {
    fn idents(&self) -> impl Iterator<Item = &Ident> {
        self.branches.iter().map(|branch| &branch.id)
    }

    fn variant_types(&self) -> impl Iterator<Item = VariantTypesDelimited> + '_ {
        self.branches.iter().map(|branch| {
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
        self.branches.iter().map(|branch| {
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

    fn blocks(&self) -> impl Iterator<Item = &GotoBlock<Other>> {
        self.branches.iter().map(|branch| &branch.block)
    }

    fn begin_block(&self) -> &GotoBlock<Initial> {
        &self.begin_branch.block
    }
}

impl Parse for SafeGoto {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let begin_branch = input.parse()?;
        if input.peek(Token!(,)) {
            let _comma: Token!(,) = input.parse()?;
            let ret = SafeGoto {
                begin_branch,
                branches: input
                    .parse_terminated::<GotoBranch<Other>, Token!(,)>(GotoBranch::parse)?,
            };
            let lifetimes: Vec<_> = ret.idents().collect();
            for i in 0..lifetimes.len() {
                if lifetimes[i + 1..].contains(&lifetimes[i]) {
                    return Err(syn::Error::new(
                        lifetimes[i].span(),
                        "block label occurs more than once",
                    ));
                }
            }
            Ok(ret)
        } else {
            Ok(SafeGoto {
                begin_branch,
                branches: Punctuated::new(),
            })
        }
    }
}

/// Executes the contained Rust code with possibly irreducible control flow
///
/// # Example
/// ```
/// use safe_goto::safe_goto;
/// safe_goto!{
///     begin() {
///         goto s1(3)
///     },
///     s1(n: i32) {
///         n + 1
///     }
/// };
/// ```
/// The invocation above generates the following code:
/// ```
/// 'outer_goto: {
///     enum States {
///         S1(i32)
///     }
///     let mut goto: States = 'goto: {
///         let break_val = {break 'goto States::S1(3)};
///         break 'outer_goto break_val;
///     };
///     'goto: loop {
///         let ret = match goto {
///             States::S1(n) => {
///                 n + 1
///             }
///         };
///         break ret;
///     }
/// };
/// ```
///
/// There must be a begin block with no arguments. The begin block cannot be
/// a target of a goto, but can be used for one-time moves.
/// Nested safe_goto's are not allowed,
/// though function calls can be used to get around this limitation.
/// Execution that exits any of the goto blocks will return from the macro
/// with the value at the end of the final block executed.
///
/// # Safety
///
/// The macro does not generate unsafe code unless given unsafe code as input.
/// There are no guarantees for how the macro will interact with unsafe code.
#[proc_macro]
pub fn safe_goto(t: TokenStream) -> TokenStream {
    let input = parse_macro_input!(t as SafeGoto);
    if input.idents().any(|id| id == "begin") {
        return syn::Error::new(Span::call_site(), "`begin` block should be first")
            .to_compile_error()
            .into();
    }
    let states_enum = Ident::new("States", Span::call_site());
    let variants: Vec<_> = input.idents().map(pascalize).collect();
    let variant_pats = input.variant_pats();
    let variant_types = input.variant_types();
    let blocks = input.blocks();
    let begin_branch = input.begin_block();
    quote!(
        {
            'outer_goto: {enum #states_enum {
                #(#variants #variant_types),*
            }
            let mut goto: #states_enum = 'goto: {
                let break_val = #begin_branch;
                #[allow(unreachable_code)]
                {break 'outer_goto break_val;}
            };

            'goto: loop {
                let ret = match goto {
                    #(#states_enum::#variants #variant_pats => #blocks),*
                };
                #[allow(unreachable_code)]
                {break ret}
            }}
        }
    )
    .into()
}
