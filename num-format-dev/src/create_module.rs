use indexmap::IndexMap;
use proc_macro2::{Delimiter, Group, Ident, Literal, Span};
use quote::quote;

use crate::rustfmt::rustfmt;
use crate::utils::Format;

/// Takes the map returned from `parse_data` and turns it into a rust module.
pub fn create_module(data: &IndexMap<String, Format>) -> Result<String, failure::Error> {
    let variant_names = data.keys().map(|s| Ident::new(s, Span::call_site()));

    let mut decimals = Vec::new();
    let mut groupings = Vec::new();
    let mut infinities = Vec::new();
    let mut minus_signs = Vec::new();
    let mut nans = Vec::new();
    let mut separators = Vec::new();
    let mut from_strs = Vec::new();
    for (variant_name, format) in data.iter() {
        let key = Ident::new(variant_name, Span::call_site());

        let value = Literal::character(format.dec);
        let group = Group::new(
            Delimiter::None,
            quote! {
                #key => #value,
            },
        );
        decimals.push(group);

        let value = format.grp.to_ident();
        let group = Group::new(
            Delimiter::None,
            quote! {
                #key => #value,
            },
        );
        groupings.push(group);

        let value = Literal::string(&format.inf);
        let group = Group::new(
            Delimiter::None,
            quote! {
                #key => #value,
            },
        );
        infinities.push(group);

        let value = Literal::string(&format.min);
        let group = Group::new(
            Delimiter::None,
            quote! {
                #key => #value,
            },
        );
        minus_signs.push(group);

        let value = Literal::string(&format.nan);
        let group = Group::new(
            Delimiter::None,
            quote! {
                #key => #value,
            },
        );
        nans.push(group);

        let value = Literal::character(format.sep);
        let group = Group::new(
            Delimiter::None,
            quote! {
                #key => Some(#value),
            },
        );
        separators.push(group);

        let value = key;
        let key = Literal::string(&format.identifier);
        let group = Group::new(
            Delimiter::None,
            quote! {
                #key => #value,
            },
        );
        from_strs.push(group);
    }

    let token_stream = quote! {
        //!Note: This module was autogenerated by num-format-dev.

        use core::str::FromStr;

        use crate::errors::Error;
        use crate::format::utils::{InfinityStr, MinusSignStr, NanStr};
        use crate::format::{Format, Grouping};

        ///<b><u>One of the most important types.</u></b> Represents formats from the
        ///[Unicode Consortium]'s [Common Locale Data Repository (CLDR)].
        ///
        ///# Example
        ///```
        ///use num_format::{format::Locale, Buffer};
        ///
        ///fn main() {
        ///    // Using the French format from the Unicode Common Locale Data Repository...
        ///    let mut buf = Buffer::new();
        ///    buf.write_formatted(&(-1000000), &Locale::fr);
        ///    assert_eq!("-1\u{202f}000\u{202f}000", buf.as_str());
        ///
        ///    // Note:
        ///    // U+202F is the "NARROW NO-BREAK SPACE" character.
        ///    // When displayed to the screen, it looks like a space.
        ///}
        ///```
        ///
        /// [Common Locale Data Repository (CLDR)]: https://en.wikipedia.org/wiki/Common_Locale_Data_Repository
        /// [Unicode Consortium]: https://en.wikipedia.org/wiki/Unicode_Consortium
        #[allow(non_camel_case_types, missing_docs)]
        #[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
        #[cfg_attr(feature = "with-serde", derive(Serialize, Deserialize))]
        pub enum Locale {
            #(#variant_names),*
        }

        impl Locale {
            #[allow(missing_docs)]
            pub fn decimal(&self) -> char {
                use self::Locale::*;
                match self {
                    #(#decimals)*
                }
            }

            #[allow(missing_docs)]
            pub fn grouping(&self) -> Grouping {
                use self::Grouping::*;
                use self::Locale::*;
                match self {
                    #(#groupings)*
                }
            }

            #[allow(missing_docs)]
            pub fn infinity(&self) -> &str {
                use self::Locale::*;
                match self {
                    #(#infinities)*
                }
            }

            #[allow(missing_docs)]
            pub fn minus_sign(&self) -> &str {
                use self::Locale::*;
                match self {
                    #(#minus_signs)*
                }
            }

            #[allow(missing_docs)]
            pub fn nan(&self) -> &str {
                use self::Locale::*;
                match self {
                    #(#nans)*
                }
            }

            #[allow(missing_docs)]
            pub fn separator(&self) -> Option<char> {
                use self::Locale::*;
                match self {
                    #(#separators)*
                }
            }
        }

        impl Format for Locale {
            fn decimal(&self) -> char {
                self.decimal()
            }

            fn grouping(&self) -> Grouping {
                self.grouping()
            }

            fn infinity(&self) -> InfinityStr<'_> {
                InfinityStr::new(self.infinity()).unwrap()
            }

            fn minus_sign(&self) -> MinusSignStr<'_> {
                MinusSignStr::new(self.minus_sign()).unwrap()
            }

            fn nan(&self) -> NanStr<'_> {
                NanStr::new(self.nan()).unwrap()
            }

            fn separator(&self) -> Option<char> {
                self.separator()
            }
        }

        impl FromStr for Locale {
            type Err = Error;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                use self::Locale::*;
                let locale = match s {
                    #(#from_strs)*
                    _ => return Err(Error::parse_locale(s)),
                };
                Ok(locale)
            }
        }
    };

    let s = format!("{}", &token_stream);
    let s = rustfmt(s)?;
    Ok(s)
}
