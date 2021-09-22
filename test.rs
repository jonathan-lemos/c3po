#![feature(prelude_import)]
#[prelude_import]
use std::prelude::rust_2018::*;
#[macro_use]
extern crate std;
pub mod ast {
    pub mod node {
        pub mod node {
            use std::iter::IntoIterator;
            pub enum Node {
                Leaf(Leaf),
                Branch(Branch),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Node {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match (&*self,) {
                        (&Node::Leaf(ref __self_0),) => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_tuple(f, "Leaf");
                            let _ =
                                ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                            ::core::fmt::DebugTuple::finish(debug_trait_builder)
                        }
                        (&Node::Branch(ref __self_0),) => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_tuple(f, "Branch");
                            let _ =
                                ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                            ::core::fmt::DebugTuple::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for Node {
                #[inline]
                fn clone(&self) -> Node {
                    match (&*self,) {
                        (&Node::Leaf(ref __self_0),) => {
                            Node::Leaf(::core::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Node::Branch(ref __self_0),) => {
                            Node::Branch(::core::clone::Clone::clone(&(*__self_0)))
                        }
                    }
                }
            }
            impl ::core::marker::StructuralPartialEq for Node {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::PartialEq for Node {
                #[inline]
                fn eq(&self, other: &Node) -> bool {
                    {
                        let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                        let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*other) {
                                (&Node::Leaf(ref __self_0), &Node::Leaf(ref __arg_1_0)) => {
                                    (*__self_0) == (*__arg_1_0)
                                }
                                (&Node::Branch(ref __self_0), &Node::Branch(ref __arg_1_0)) => {
                                    (*__self_0) == (*__arg_1_0)
                                }
                                _ => unsafe { ::core::intrinsics::unreachable() },
                            }
                        } else {
                            false
                        }
                    }
                }
                #[inline]
                fn ne(&self, other: &Node) -> bool {
                    {
                        let __self_vi = ::core::intrinsics::discriminant_value(&*self);
                        let __arg_1_vi = ::core::intrinsics::discriminant_value(&*other);
                        if true && __self_vi == __arg_1_vi {
                            match (&*self, &*other) {
                                (&Node::Leaf(ref __self_0), &Node::Leaf(ref __arg_1_0)) => {
                                    (*__self_0) != (*__arg_1_0)
                                }
                                (&Node::Branch(ref __self_0), &Node::Branch(ref __arg_1_0)) => {
                                    (*__self_0) != (*__arg_1_0)
                                }
                                _ => unsafe { ::core::intrinsics::unreachable() },
                            }
                        } else {
                            true
                        }
                    }
                }
            }
            impl ::core::marker::StructuralEq for Node {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::Eq for Node {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Leaf>;
                        let _: ::core::cmp::AssertParamIsEq<Branch>;
                    }
                }
            }
            pub struct Leaf {
                kind: String,
                value: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Leaf {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Leaf {
                            kind: ref __self_0_0,
                            value: ref __self_0_1,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Leaf");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_1),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for Leaf {
                #[inline]
                fn clone(&self) -> Leaf {
                    match *self {
                        Leaf {
                            kind: ref __self_0_0,
                            value: ref __self_0_1,
                        } => Leaf {
                            kind: ::core::clone::Clone::clone(&(*__self_0_0)),
                            value: ::core::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl ::core::marker::StructuralPartialEq for Leaf {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::PartialEq for Leaf {
                #[inline]
                fn eq(&self, other: &Leaf) -> bool {
                    match *other {
                        Leaf {
                            kind: ref __self_1_0,
                            value: ref __self_1_1,
                        } => match *self {
                            Leaf {
                                kind: ref __self_0_0,
                                value: ref __self_0_1,
                            } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &Leaf) -> bool {
                    match *other {
                        Leaf {
                            kind: ref __self_1_0,
                            value: ref __self_1_1,
                        } => match *self {
                            Leaf {
                                kind: ref __self_0_0,
                                value: ref __self_0_1,
                            } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            impl ::core::marker::StructuralEq for Leaf {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::Eq for Leaf {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<String>;
                        let _: ::core::cmp::AssertParamIsEq<String>;
                    }
                }
            }
            pub struct Branch {
                kind: String,
                children: Vec<Node>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for Branch {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Branch {
                            kind: ref __self_0_0,
                            children: ref __self_0_1,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Branch");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "children",
                                &&(*__self_0_1),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for Branch {
                #[inline]
                fn clone(&self) -> Branch {
                    match *self {
                        Branch {
                            kind: ref __self_0_0,
                            children: ref __self_0_1,
                        } => Branch {
                            kind: ::core::clone::Clone::clone(&(*__self_0_0)),
                            children: ::core::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl ::core::marker::StructuralPartialEq for Branch {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::PartialEq for Branch {
                #[inline]
                fn eq(&self, other: &Branch) -> bool {
                    match *other {
                        Branch {
                            kind: ref __self_1_0,
                            children: ref __self_1_1,
                        } => match *self {
                            Branch {
                                kind: ref __self_0_0,
                                children: ref __self_0_1,
                            } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &Branch) -> bool {
                    match *other {
                        Branch {
                            kind: ref __self_1_0,
                            children: ref __self_1_1,
                        } => match *self {
                            Branch {
                                kind: ref __self_0_0,
                                children: ref __self_0_1,
                            } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            impl ::core::marker::StructuralEq for Branch {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::cmp::Eq for Branch {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<String>;
                        let _: ::core::cmp::AssertParamIsEq<Vec<Node>>;
                    }
                }
            }
            impl Node {
                /// Returns a new Node with no children
                ///
                /// Use `Node::new()` if the node should have children.
                ///
                /// # Arguments
                /// * `kind`     - an `AsRef<str>` containing what kind of value this node contains (e.g. comma, identifier, keyword, number, string, etc.)
                /// * `value`    - an `Into<String>` containing the contents of the node
                pub fn leaf<A: Into<String>, B: Into<String>>(kind: A, value: B) -> Self {
                    Node::Leaf(Leaf {
                        kind: kind.into(),
                        value: value.into(),
                    })
                }
                /// Returns a new Node
                ///
                /// # Arguments
                /// * `kind`     - an `AsRef<str>` containing what kind of value this node contains (e.g. comma, identifier, keyword, number, string, etc.)
                /// * `value`    - an `Into<String>` containing the contents of the node
                /// * `children` - an `IntoIter<Node>` containing the children of the node. Use `Node::leaf()` if there are no children of this node.
                pub fn branch<A: Into<String>, I: IntoIterator<Item = Node>>(
                    kind: A,
                    children: I,
                ) -> Self {
                    Node::Branch(Branch {
                        kind: kind.into(),
                        children: children.into_iter().collect(),
                    })
                }
                /// Returns the kind of the node.
                pub fn kind(&self) -> &str {
                    match self {
                        Node::Branch(n) => &n.kind,
                        Node::Leaf(n) => &n.kind,
                    }
                }
            }
        }
    }
    pub mod regex {
        pub mod regex {
            use super::super::node::node::Node;
            use crate::parser::parser::Parser;
            use crate::parsers::map::map::Map;
            use crate::parsers::regex::regexparser::RegexParser;
            pub fn regex<TPattern: Into<String>, TKind: Into<String>>(
                pattern: TPattern,
                kind: TKind,
            ) -> impl Parser<Output = Node> {
                let rep = RegexParser::new(pattern);
                let kind = kind.into();
                let kc = kind.clone();
                rep.map(|_| kc, move |s| Node::leaf(kind.clone(), s))
            }
        }
    }
    pub mod repeat {
        pub mod repeat {
            use super::super::node::node::Node;
            use crate::parser::parser::Parser;
            use crate::parsers::map::map::Map;
            use crate::parsers::repeat::repeatparser::RepeatParser;
            use std::ops::RangeBounds;
            pub fn repeat<TParser, TRange, TKind>(
                parser: TParser,
                range: TRange,
                kind: TKind,
            ) -> impl Parser<Output = Node>
            where
                TParser: Parser<Output = Node>,
                TRange: RangeBounds<usize>,
                TKind: Into<String>,
            {
                let kind = kind.into();
                let kc = kind.clone();
                RepeatParser::range(parser, range)
                    .map(|_| kc, move |v| Node::branch(kind.clone(), v))
            }
        }
    }
    pub mod string {
        pub mod string {
            use super::super::node::node::Node;
            use crate::parser::parser::Parser;
            use crate::parsers::map::map::Map;
            use crate::parsers::string::stringparser::StringParser;
            pub fn string<TValue: Into<String>, TKind: Into<String>>(
                string: TValue,
                kind: TKind,
            ) -> impl Parser<Output = Node> {
                let kind = kind.into();
                let kc = kind.clone();
                StringParser::new(string).map(|_| kc, move |s| Node::leaf(kind.clone(), s))
            }
        }
    }
}
pub mod immut_iter {
    pub mod immut_iterable {
        use super::iterator::immut_iterator::ImmutableIterator;
        /// Can retrieve the next element of a sequence with an immutable reference.
        /// The output type must be `Self`.
        ///
        /// This is meant to be the immutable reference equivalent of `Iterator::next(&mut Self)`
        pub trait ImmutableIterable: Clone + Sized {
            fn next_immut(&self) -> Option<Self>;
            fn iter_immut(&self) -> ImmutableIterator<Self> {
                ImmutableIterator::new(self.clone())
            }
        }
    }
    pub mod iterator {
        pub mod from {
            use super::super::immut_iterable::ImmutableIterable;
            use super::immut_iterator::ImmutableIterator;
            impl<I: ImmutableIterable> From<&I> for ImmutableIterator<I> {
                fn from(iterable: &I) -> Self {
                    iterable.iter_immut()
                }
            }
            impl<I: ImmutableIterable> From<I> for ImmutableIterator<I> {
                fn from(iterable: I) -> Self {
                    ImmutableIterator::new(iterable)
                }
            }
            impl<I: ImmutableIterable> From<Option<I>> for ImmutableIterator<I> {
                fn from(option: Option<I>) -> Self {
                    ImmutableIterator::from_option(option)
                }
            }
        }
        pub mod immut_iterator {
            use super::super::immut_iterable::ImmutableIterable;
            /// Iterator for an `ImmutableIterable`
            pub struct ImmutableIterator<I: ImmutableIterable> {
                pub(super) iterable: Option<I>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<I: ::core::fmt::Debug + ImmutableIterable> ::core::fmt::Debug for ImmutableIterator<I> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        ImmutableIterator {
                            iterable: ref __self_0_0,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "ImmutableIterator");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "iterable",
                                &&(*__self_0_0),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<I: ::core::clone::Clone + ImmutableIterable> ::core::clone::Clone for ImmutableIterator<I> {
                #[inline]
                fn clone(&self) -> ImmutableIterator<I> {
                    match *self {
                        ImmutableIterator {
                            iterable: ref __self_0_0,
                        } => ImmutableIterator {
                            iterable: ::core::clone::Clone::clone(&(*__self_0_0)),
                        },
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<I: ::core::marker::Copy + ImmutableIterable> ::core::marker::Copy for ImmutableIterator<I> {}
            impl<I: ImmutableIterable> ImmutableIterator<I> {
                /// Creates a new `ImmutableIterator` from the given `ImmutableIterable`
                pub fn new(iterable: I) -> Self {
                    ImmutableIterator {
                        iterable: Some(iterable),
                    }
                }
                /// Creates a new `ImmutableIterator` from the given `Option<ImmutableIterable>`. `None` will produce an empty iterator.
                pub fn from_option(option: Option<I>) -> Self {
                    ImmutableIterator { iterable: option }
                }
            }
        }
        pub mod iterator {
            use super::super::immut_iterable::ImmutableIterable;
            use super::immut_iterator::ImmutableIterator;
            impl<I: ImmutableIterable> Iterator for ImmutableIterator<I> {
                type Item = I;
                fn next(&mut self) -> Option<Self::Item> {
                    match &self.iterable {
                        None => None,
                        Some(v) => match v.next_immut() {
                            None => self.iterable.take(),
                            Some(next) => self.iterable.replace(next),
                        },
                    }
                }
            }
        }
    }
}
pub mod parser {
    pub mod cursor {
        pub mod add {
            use super::cursor::Cursor;
            use crate::immut_iter::immut_iterable::ImmutableIterable;
            use core::ops::Add;
            impl Add<usize> for Cursor<'_> {
                type Output = Option<Self>;
                /// Increments the Cursor by `n` lexemes
                ///
                /// Returns `None` if the new position of the cursor is out-of-bounds.
                fn add(self, n: usize) -> Self::Output {
                    (0..n).fold(Some(self), |a, _| a.and_then(|c| c.next_immut()))
                }
            }
            impl<'a> Add<usize> for &Cursor<'a> {
                type Output = Option<Cursor<'a>>;
                /// Increments the Cursor by `n` lexemes
                ///
                /// Returns `None` if the new position of the cursor is out-of-bounds.
                fn add(self, n: usize) -> Self::Output {
                    *self + n
                }
            }
        }
        pub mod copy {
            use super::cursor::Cursor;
            impl<'a> Clone for Cursor<'a> {
                fn clone(&self) -> Self {
                    *self
                }
            }
            impl<'a> Copy for Cursor<'a> {}
        }
        pub mod cursor {
            use std::ptr;
            /// Represents a position in a string.
            pub struct Cursor<'a> {
                pub(super) sequence: &'a str,
                pub(super) pos_bytes: usize,
                pub(super) pos_chars: usize,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a> ::core::fmt::Debug for Cursor<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        Cursor {
                            sequence: ref __self_0_0,
                            pos_bytes: ref __self_0_1,
                            pos_chars: ref __self_0_2,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "Cursor");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "sequence",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "pos_bytes",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "pos_chars",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl<'a> Cursor<'a> {
                /// Creates a new Cursor at the beginning of a string.
                pub fn new(s: &'a str) -> Option<Self> {
                    if s.len() == 0 {
                        None
                    } else {
                        Some(Self {
                            sequence: s,
                            pos_bytes: 0,
                            pos_chars: 0,
                        })
                    }
                }
                /// Gets the character that this Cursor is pointing to.
                pub fn current(&self) -> char {
                    self.current_str().chars().next().expect(
                        "Cursor is pointing to an invalid location. This should never happen.",
                    )
                }
                /// Gets the string slice that this Cursor is pointing to, starting from the byte that this Cursor points to.
                pub fn current_str(&self) -> &'a str {
                    &self.sequence[self.pos_bytes..]
                }
                /// Gets the amount of `next_immut()` calls needed to reach one Cursor from the other.
                ///
                /// Returns `None` if the two cursors have different sources.
                pub fn difference(&self, other: &Self) -> Option<usize> {
                    if ptr::eq(self.sequence, other.sequence) {
                        Some(if self.pos_chars >= other.pos_chars {
                            self.pos_chars - other.pos_chars
                        } else {
                            other.pos_chars - self.pos_chars
                        })
                    } else {
                        None
                    }
                }
                /// Gets the 0-based position of the cursor within the `source()` in characters.
                pub fn pos(&self) -> usize {
                    self.pos_chars
                }
                /// Gets the source string slice that this Cursor points to, including the characters behind this Cursor.
                pub fn source(&self) -> &'a str {
                    &self.sequence
                }
            }
        }
        pub mod eq {
            use super::cursor::Cursor;
            use std::ptr;
            impl PartialEq for Cursor<'_> {
                /// Returns `true` if the input slices are the same *pointer* and the positions of both Cursors are the same.
                ///
                /// This does not check that the input slices are value equal.
                fn eq(&self, other: &Self) -> bool {
                    ptr::eq(self.sequence, other.sequence) && self.pos_chars == other.pos_chars
                }
            }
            impl Eq for Cursor<'_> {}
        }
        pub mod immut_iterable {
            use super::cursor::Cursor;
            use crate::immut_iter::immut_iterable::ImmutableIterable;
            impl ImmutableIterable for Cursor<'_> {
                fn next_immut(&self) -> Option<Self> {
                    let length_bytes = self.current().len_utf8();
                    let ret = Cursor {
                        sequence: self.sequence,
                        pos_bytes: self.pos_bytes + length_bytes,
                        pos_chars: self.pos_chars + 1,
                    };
                    if ret.pos_bytes >= ret.sequence.len() {
                        None
                    } else {
                        Some(ret)
                    }
                }
            }
        }
        pub mod partialord {
            use super::cursor::Cursor;
            use std::cmp::Ordering;
            use std::ptr;
            impl PartialOrd for Cursor<'_> {
                /// Compares the two `pos()` values of the two Cursors.
                ///
                /// Returns `None` if the two input slices are different *pointers*.
                /// This does not check the two slices for value equality.
                fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
                    if !ptr::eq(self.sequence, other.sequence) {
                        None
                    } else {
                        PartialOrd::partial_cmp(&self.pos_chars, &other.pos_chars)
                    }
                }
            }
        }
    }
    pub mod output {
        pub mod output {
            use super::super::cursor::cursor::Cursor;
            use super::super::parse::parse::Parse;
            /// The output type of `Parser::parse`.
            pub struct ParserOutput<'a, TOutput> {
                beginning: Cursor<'a>,
                kind: String,
                parse: Parse<'a, TOutput>,
            }
            impl<'a, TOutput> ParserOutput<'a, TOutput> {
                /// Creates a new ParserOutput
                ///
                /// # Arguments
                /// * `parse`     - A parse generated by a parsing function. See `Parser::func` for details.
                /// * `beginning` - A Cursor pointing to the first token of the parse.
                /// * `kind`      - What kind of value this parse has (on success).
                pub fn new<S: Into<String>>(
                    parse: Parse<'a, TOutput>,
                    beginning: Cursor<'a>,
                    kind: S,
                ) -> Self {
                    ParserOutput {
                        beginning,
                        kind: kind.into(),
                        parse,
                    }
                }
                /// A Cursor pointing to the first token of the parse.
                pub fn beginning(&self) -> &Cursor<'a> {
                    &self.beginning
                }
                /// What kind of value this parse should have.
                pub fn kind(&self) -> &str {
                    &self.kind
                }
                /// The parse itself.
                pub fn parse(&self) -> &Parse<'a, TOutput> {
                    &self.parse
                }
            }
        }
    }
    pub mod parse {
        pub mod failedparse {
            use super::super::cursor::cursor::Cursor;
            /// Represents a failed parse result. See the `Parse` enum for details.
            pub struct FailedParse<'a> {
                pub(super) bad_token: Option<Cursor<'a>>,
                pub(super) reason: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a> ::core::fmt::Debug for FailedParse<'a> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        FailedParse {
                            bad_token: ref __self_0_0,
                            reason: ref __self_0_1,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "FailedParse");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "bad_token",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "reason",
                                &&(*__self_0_1),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a> ::core::clone::Clone for FailedParse<'a> {
                #[inline]
                fn clone(&self) -> FailedParse<'a> {
                    match *self {
                        FailedParse {
                            bad_token: ref __self_0_0,
                            reason: ref __self_0_1,
                        } => FailedParse {
                            bad_token: ::core::clone::Clone::clone(&(*__self_0_0)),
                            reason: ::core::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl<'a> ::core::marker::StructuralPartialEq for FailedParse<'a> {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a> ::core::cmp::PartialEq for FailedParse<'a> {
                #[inline]
                fn eq(&self, other: &FailedParse<'a>) -> bool {
                    match *other {
                        FailedParse {
                            bad_token: ref __self_1_0,
                            reason: ref __self_1_1,
                        } => match *self {
                            FailedParse {
                                bad_token: ref __self_0_0,
                                reason: ref __self_0_1,
                            } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &FailedParse<'a>) -> bool {
                    match *other {
                        FailedParse {
                            bad_token: ref __self_1_0,
                            reason: ref __self_1_1,
                        } => match *self {
                            FailedParse {
                                bad_token: ref __self_0_0,
                                reason: ref __self_0_1,
                            } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            impl<'a> ::core::marker::StructuralEq for FailedParse<'a> {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a> ::core::cmp::Eq for FailedParse<'a> {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Option<Cursor<'a>>>;
                        let _: ::core::cmp::AssertParamIsEq<String>;
                    }
                }
            }
            impl<'a> FailedParse<'a> {
                /// Creates a FailedParse
                ///
                /// # Arguments
                /// * `bad_token` - A cursor pointing to the first unparseable lexeme, or `None` if the parse ran out of lexemes to parse (reached end-of-file).
                /// * `reason`    - A reason why the lexemes couldn't be parsed.
                pub fn new<S: Into<String>>(bad_token: Option<Cursor<'a>>, reason: S) -> Self {
                    FailedParse {
                        bad_token,
                        reason: reason.into(),
                    }
                }
                /// A cursor pointing to the first unparseable lexeme, or `None` if the parse ran out of lexemes to parse (reached end-of-file).
                pub fn bad_token(&self) -> &Option<Cursor<'a>> {
                    &self.bad_token
                }
                /// The reason why the lexemes couldn't be parsed.
                pub fn reason(&self) -> &str {
                    &self.reason
                }
            }
        }
        pub mod parse {
            use super::super::cursor::cursor::Cursor;
            use super::failedparse::FailedParse;
            use super::successfulparse::SuccessfulParse;
            /// Represents the result of a parsing operation.
            pub enum Parse<'a, TValue> {
                Success(SuccessfulParse<'a, TValue>),
                Failure(FailedParse<'a>),
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a, TValue: ::core::fmt::Debug> ::core::fmt::Debug for Parse<'a, TValue> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match (&*self,) {
                        (&Parse::Success(ref __self_0),) => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_tuple(f, "Success");
                            let _ =
                                ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                            ::core::fmt::DebugTuple::finish(debug_trait_builder)
                        }
                        (&Parse::Failure(ref __self_0),) => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_tuple(f, "Failure");
                            let _ =
                                ::core::fmt::DebugTuple::field(debug_trait_builder, &&(*__self_0));
                            ::core::fmt::DebugTuple::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a, TValue: ::core::clone::Clone> ::core::clone::Clone for Parse<'a, TValue> {
                #[inline]
                fn clone(&self) -> Parse<'a, TValue> {
                    match (&*self,) {
                        (&Parse::Success(ref __self_0),) => {
                            Parse::Success(::core::clone::Clone::clone(&(*__self_0)))
                        }
                        (&Parse::Failure(ref __self_0),) => {
                            Parse::Failure(::core::clone::Clone::clone(&(*__self_0)))
                        }
                    }
                }
            }
            impl<'a, TValue> Parse<'a, TValue> {
                /// Creates a successful parsing result.
                ///
                /// # Arguments
                /// * `next`  - A cursor pointing to the next lexeme after the parsed section. `None` if this parse covers the last token in the cursor's source.
                /// * `value` - The parsed value.
                pub fn success(next: Option<Cursor<'a>>, value: TValue) -> Self {
                    Parse::Success(SuccessfulParse {
                        next,
                        value: value.into(),
                    })
                }
                /// Creates a failed parsing result.
                ///
                /// # Arguments
                /// * `bad_token` - A cursor pointing to the first unparseable lexeme, or `None` if the parse ran out of lexemes to parse (reached end-of-file).
                /// * `reason`    - The reason why the parse couldn't succeed.
                pub fn failure<S: Into<String>>(bad_token: Option<Cursor<'a>>, reason: S) -> Self {
                    Parse::Failure(FailedParse {
                        bad_token,
                        reason: reason.into(),
                    })
                }
                /// If successful, return a new Parse as a function of the old Parse, otherwise return the existing failure.
                ///
                /// For Haskell folk, this binds on the success case.
                ///
                /// # Arguments
                /// * `if_successful` - If the Parse is successful, run this function.
                pub fn and_then<F: FnOnce(SuccessfulParse<'a, TValue>) -> Parse<'a, TValue>>(
                    self,
                    if_successful: F,
                ) -> Self {
                    match self {
                        Parse::Success(success) => if_successful(success),
                        Parse::Failure(failure) => Parse::Failure(failure),
                    }
                }
                /// Unwraps a SuccessfulParse if the parse succeeded. Panics with the given message if it failed.
                pub fn expect<S: AsRef<str>>(self, if_not: S) -> SuccessfulParse<'a, TValue> {
                    match self {
                        Parse::Success(s) => s,
                        Parse::Failure(f) => {
                            ::std::rt::begin_panic_fmt(&match match (&f.reason(), &if_not.as_ref())
                            {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(
                                        &[
                                            "Expected a successful parse, but it failed due to ",
                                            ".\nMessage: ",
                                        ],
                                        args,
                                    )
                                },
                            })
                        }
                    }
                }
                /// Unwraps a FailedParse if the parse failed. Panics with the given message if it succeeded.
                pub fn expect_failure<S: AsRef<str>>(self, if_not: S) -> FailedParse<'a> {
                    match self {
                        Parse::Failure(f) => f,
                        Parse::Success(s) => {
                            let pos = s
                                .next()
                                .map(|s| s.pos().to_string())
                                .unwrap_or("EOF".to_owned());
                            {
                                ::std::rt::begin_panic_fmt(&match match (&pos, &if_not.as_ref()) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                } {
                                    ref args => unsafe {
                                        :: core :: fmt :: Arguments :: new_v1 (& ["Expected a failed parse, but it succeeded with before at position " , ".\nMessage: "] , args)
                                    },
                                })
                            }
                        }
                    }
                }
                /// `true` if the Parse failed, `false` if not.
                pub fn is_failure(&self) -> bool {
                    match self {
                        Parse::Failure(_) => true,
                        Parse::Success(_) => false,
                    }
                }
                /// `true` if the Parse succeeded, `false` if not.
                pub fn is_success(&self) -> bool {
                    match self {
                        Parse::Success(_) => true,
                        Parse::Failure(_) => false,
                    }
                }
                /// If this Parse is a Success, transforms the old value into a new value, otherwise returns the original Failure
                ///
                /// # Arguments
                /// * `mapper` - A function that transforms the old value into a new one.
                pub fn map<TNewValue, F: FnOnce(TValue) -> TNewValue>(
                    self,
                    mapper: F,
                ) -> Parse<'a, TNewValue> {
                    match self {
                        Parse::Success(success) => {
                            Parse::success(success.next, mapper(success.value))
                        }
                        Parse::Failure(failure) => Parse::Failure(failure),
                    }
                }
                /// If this Parse is a Failure, transforms the old reason into a new reason, otherwise returns the original Success
                ///
                /// # Arguments
                /// * `mapper` - A function that transforms the old reason into a new one.
                pub fn map_reason<S: Into<String>, F: FnOnce(String) -> S>(
                    self,
                    mapper: F,
                ) -> Self {
                    match self {
                        Parse::Success(s) => Parse::Success(s),
                        Parse::Failure(f) => Parse::failure(f.bad_token, mapper(f.reason)),
                    }
                }
                /// If unsuccessful, return a new Parse as a function of the old Parse, otherwise return the existing success.
                ///
                /// For Haskell folk, this binds on the failure case.
                ///
                /// # Arguments
                /// * `if_failure` - If the Parse is unsuccessful, run this function.
                pub fn or_else<F: FnOnce(FailedParse<'a>) -> Parse<'a, TValue>>(
                    self,
                    if_failure: F,
                ) -> Self {
                    match self {
                        Parse::Success(s) => Parse::Success(s),
                        Parse::Failure(f) => if_failure(f),
                    }
                }
                /// Unwraps a SuccessfulParse if the parse succeeded. Panics if it failed.
                pub fn unwrap(self) -> SuccessfulParse<'a, TValue> {
                    self.expect("unwrap() called on a failed parse.")
                }
                /// Unwraps a FailedParse if the parse failed. Panics if it succeeded.
                pub fn unwrap_failure(self) -> FailedParse<'a> {
                    self.expect_failure("unwrap_err() called on a successful parse.S")
                }
            }
        }
        pub mod successfulparse {
            use super::super::cursor::cursor::Cursor;
            /// Represents a successful parse result. See the `Parse` enum for details.
            pub struct SuccessfulParse<'a, TValue> {
                pub(super) next: Option<Cursor<'a>>,
                pub(super) value: TValue,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a, TValue: ::core::fmt::Debug> ::core::fmt::Debug for SuccessfulParse<'a, TValue> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        SuccessfulParse {
                            next: ref __self_0_0,
                            value: ref __self_0_1,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "SuccessfulParse");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "next",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "value",
                                &&(*__self_0_1),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a, TValue: ::core::clone::Clone> ::core::clone::Clone for SuccessfulParse<'a, TValue> {
                #[inline]
                fn clone(&self) -> SuccessfulParse<'a, TValue> {
                    match *self {
                        SuccessfulParse {
                            next: ref __self_0_0,
                            value: ref __self_0_1,
                        } => SuccessfulParse {
                            next: ::core::clone::Clone::clone(&(*__self_0_0)),
                            value: ::core::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl<'a, TValue> ::core::marker::StructuralPartialEq for SuccessfulParse<'a, TValue> {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a, TValue: ::core::cmp::PartialEq> ::core::cmp::PartialEq for SuccessfulParse<'a, TValue> {
                #[inline]
                fn eq(&self, other: &SuccessfulParse<'a, TValue>) -> bool {
                    match *other {
                        SuccessfulParse {
                            next: ref __self_1_0,
                            value: ref __self_1_1,
                        } => match *self {
                            SuccessfulParse {
                                next: ref __self_0_0,
                                value: ref __self_0_1,
                            } => (*__self_0_0) == (*__self_1_0) && (*__self_0_1) == (*__self_1_1),
                        },
                    }
                }
                #[inline]
                fn ne(&self, other: &SuccessfulParse<'a, TValue>) -> bool {
                    match *other {
                        SuccessfulParse {
                            next: ref __self_1_0,
                            value: ref __self_1_1,
                        } => match *self {
                            SuccessfulParse {
                                next: ref __self_0_0,
                                value: ref __self_0_1,
                            } => (*__self_0_0) != (*__self_1_0) || (*__self_0_1) != (*__self_1_1),
                        },
                    }
                }
            }
            impl<'a, TValue> ::core::marker::StructuralEq for SuccessfulParse<'a, TValue> {}
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<'a, TValue: ::core::cmp::Eq> ::core::cmp::Eq for SuccessfulParse<'a, TValue> {
                #[inline]
                #[doc(hidden)]
                #[no_coverage]
                fn assert_receiver_is_total_eq(&self) -> () {
                    {
                        let _: ::core::cmp::AssertParamIsEq<Option<Cursor<'a>>>;
                        let _: ::core::cmp::AssertParamIsEq<TValue>;
                    }
                }
            }
            impl<'a, TValue> SuccessfulParse<'a, TValue> {
                /// Creates a SuccessfulParse
                ///
                /// # Arguments
                /// * `next`  - A cursor pointing to the next lexeme after the parsed section. `None` if this parse covers the last token in the cursor's source.
                /// * `value` - The parsed value.
                pub fn new<V: Into<TValue>>(next: Option<Cursor<'a>>, value: V) -> Self {
                    SuccessfulParse {
                        next,
                        value: value.into(),
                    }
                }
                /// Takes ownership of the parsed value.
                pub fn into_value(self) -> TValue {
                    self.value
                }
                /// A Cursor pointing to the next lexeme after the parsed section. `None` if there are no more lexmees after this parse.
                pub fn next(&self) -> &Option<Cursor<'a>> {
                    &self.next
                }
                /// The parsed value.
                pub fn value(&self) -> &TValue {
                    &self.value
                }
            }
        }
    }
    pub mod parser {
        use super::cursor::cursor::Cursor;
        use super::parse::parse::Parse;
        pub trait Parser: Clone + Send + Sync {
            type Output;
            fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, Self::Output>;
            fn kind(&self) -> &str;
        }
    }
}
pub mod parsers {
    pub mod compose {
        pub mod clone {
            use super::composeparser::ComposeParser;
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            impl<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner> Clone
                for ComposeParser<
                    TFirstOutput,
                    TFirst,
                    TSecondOutput,
                    TSecond,
                    TFinalOutput,
                    FCombiner,
                >
            where
                TFirstOutput: Send + Sync,
                TFirst: Parser<Output = TFirstOutput>,
                TSecondOutput: Send + Sync,
                TSecond: Parser<Output = TSecondOutput>,
                TFinalOutput: Send + Sync,
                FCombiner: Fn(TFirstOutput, TSecondOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                fn clone(&self) -> Self {
                    ComposeParser {
                        first: self.first.clone(),
                        second: self.second.clone(),
                        combiner: self.combiner.clone(),
                        kind: self.kind.clone(),
                        _first_output_marker: PhantomData,
                        _second_output_marker: PhantomData,
                        _final_output_marker: PhantomData,
                    }
                }
            }
        }
        pub mod composeparser {
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            /// Composes two parsers, matching the first parser and then the second parser, returning both results.
            pub struct ComposeParser<
                TFirstOutput,
                TFirst,
                TSecondOutput,
                TSecond,
                TFinalOutput,
                FCombiner,
            >
            where
                TFirstOutput: Send + Sync,
                TFirst: Parser<Output = TFirstOutput>,
                TSecondOutput: Send + Sync,
                TSecond: Parser<Output = TSecondOutput>,
                TFinalOutput: Send + Sync,
                FCombiner: Fn(TFirstOutput, TSecondOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                pub(super) first: TFirst,
                pub(super) second: TSecond,
                pub(super) combiner: FCombiner,
                pub(super) kind: String,
                pub(super) _first_output_marker: PhantomData<TFirstOutput>,
                pub(super) _second_output_marker: PhantomData<TSecondOutput>,
                pub(super) _final_output_marker: PhantomData<TFinalOutput>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<
                    TFirstOutput: ::core::fmt::Debug,
                    TFirst: ::core::fmt::Debug,
                    TSecondOutput: ::core::fmt::Debug,
                    TSecond: ::core::fmt::Debug,
                    TFinalOutput: ::core::fmt::Debug,
                    FCombiner: ::core::fmt::Debug,
                > ::core::fmt::Debug
                for ComposeParser<
                    TFirstOutput,
                    TFirst,
                    TSecondOutput,
                    TSecond,
                    TFinalOutput,
                    FCombiner,
                >
            where
                TFirstOutput: Send + Sync,
                TFirst: Parser<Output = TFirstOutput>,
                TSecondOutput: Send + Sync,
                TSecond: Parser<Output = TSecondOutput>,
                TFinalOutput: Send + Sync,
                FCombiner: Fn(TFirstOutput, TSecondOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        ComposeParser {
                            first: ref __self_0_0,
                            second: ref __self_0_1,
                            combiner: ref __self_0_2,
                            kind: ref __self_0_3,
                            _first_output_marker: ref __self_0_4,
                            _second_output_marker: ref __self_0_5,
                            _final_output_marker: ref __self_0_6,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "ComposeParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "first",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "second",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "combiner",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_first_output_marker",
                                &&(*__self_0_4),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_second_output_marker",
                                &&(*__self_0_5),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_final_output_marker",
                                &&(*__self_0_6),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
                ComposeParser<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner>
            where
                TFirstOutput: Send + Sync,
                TFirst: Parser<Output = TFirstOutput>,
                TSecondOutput: Send + Sync,
                TSecond: Parser<Output = TSecondOutput>,
                TFinalOutput: Send + Sync,
                FCombiner: Fn(TFirstOutput, TSecondOutput) -> TFinalOutput + Clone + Send + Sync,
            {
                /// Creates a new ComposeParser that combines the two results with a function.
                ///
                /// # Arguments
                /// * `first`    - The first parser to run.
                /// * `second`   - The parser to run after `first`.
                /// * `combiner` - A function that combines the results of the two parsers.
                pub fn using_combiner(first: TFirst, second: TSecond, combiner: FCombiner) -> Self {
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&first.kind(), &second.kind()) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["", " + "], args)
                                },
                            },
                        );
                        res
                    };
                    ComposeParser {
                        first,
                        second,
                        combiner,
                        kind,
                        _first_output_marker: PhantomData,
                        _second_output_marker: PhantomData,
                        _final_output_marker: PhantomData,
                    }
                }
            }
            impl<TFirstOutput, TFirst, TSecondOutput, TSecond>
                ComposeParser<
                    TFirstOutput,
                    TFirst,
                    TSecondOutput,
                    TSecond,
                    (TFirstOutput, TSecondOutput),
                    fn(TFirstOutput, TSecondOutput) -> (TFirstOutput, TSecondOutput),
                >
            where
                TFirstOutput: Send + Sync,
                TFirst: Parser<Output = TFirstOutput>,
                TSecondOutput: Send + Sync,
                TSecond: Parser<Output = TSecondOutput>,
            {
                /// Creates a new ComposeParser that combines the two results into a tuple.
                ///
                /// # Arguments
                /// * `first`  - The first parser to run.
                /// * `second` - The parser to run after `first`.
                pub fn new(first: TFirst, second: TSecond) -> Self {
                    Self::using_combiner(first, second, |a, b| (a, b))
                }
            }
        }
        pub mod parser {
            use super::composeparser::ComposeParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl<TFirstOutput, TFirst, TSecondOutput, TSecond, TFinalOutput, FCombiner> Parser
                for ComposeParser<
                    TFirstOutput,
                    TFirst,
                    TSecondOutput,
                    TSecond,
                    TFinalOutput,
                    FCombiner,
                >
            where
                TFirstOutput: Send + Sync,
                TFirst: Parser<Output = TFirstOutput>,
                TSecondOutput: Send + Sync,
                TSecond: Parser<Output = TSecondOutput>,
                TFinalOutput: Send + Sync,
                FCombiner: Fn(TFirstOutput, TSecondOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                type Output = TFinalOutput;
                fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TFinalOutput> {
                    let first_parse = match self.first.parse(cursor) {
                        Parse::Success(success) => success,
                        Parse::Failure(failure) => {
                            let reason = {
                                let res = ::alloc::fmt::format(
                                    match match (&self.first.kind(), &failure.reason()) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    } {
                                        ref args => unsafe {
                                            ::core::fmt::Arguments::new_v1(
                                                &["Failed to parse ", ": "],
                                                args,
                                            )
                                        },
                                    },
                                );
                                res
                            };
                            return Parse::failure(cursor, reason);
                        }
                    };
                    self.second
                        .parse(*first_parse.next())
                        .map(|v| (self.combiner)(first_parse.into_value(), v))
                        .map_reason(|r| {
                            let res = ::alloc::fmt::format(
                                match match (&self.second.kind(), &r) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                } {
                                    ref args => unsafe {
                                        ::core::fmt::Arguments::new_v1(
                                            &["Failed to parse ", ": "],
                                            args,
                                        )
                                    },
                                },
                            );
                            res
                        })
                }
                fn kind(&self) -> &str {
                    &self.kind
                }
            }
        }
    }
    pub mod either {
        pub mod clone {
            use super::eitherparser::EitherParser;
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            impl<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    TFinalOutput,
                    FLeftMapper,
                    FRightMapper,
                > Clone
                for EitherParser<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    TFinalOutput,
                    FLeftMapper,
                    FRightMapper,
                >
            where
                TLeftOutput: Send + Sync,
                TLeft: Parser<Output = TLeftOutput>,
                TRightOutput: Send + Sync,
                TRight: Parser<Output = TRightOutput>,
                TFinalOutput: Send + Sync,
                FLeftMapper: Fn(TLeftOutput) -> TFinalOutput + Send + Sync + Clone,
                FRightMapper: Fn(TRightOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                fn clone(&self) -> Self {
                    Self {
                        left: self.left.clone(),
                        right: self.right.clone(),
                        if_left: self.if_left.clone(),
                        if_right: self.if_right.clone(),
                        kind: self.kind.clone(),
                        _first_output_marker: PhantomData,
                        _second_output_marker: PhantomData,
                        _final_output_marker: PhantomData,
                    }
                }
            }
        }
        pub mod eitherparser {
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            /// Tries to match one parser, then tries to match the other.
            ///
            /// It tries to match the left parser first, and only runs the right if the left doesn't match.
            pub struct EitherParser<
                TLeftOutput,
                TLeft,
                TRightOutput,
                TRight,
                TFinalOutput,
                FLeftMapper,
                FRightMapper,
            >
            where
                TLeftOutput: Send + Sync,
                TLeft: Parser<Output = TLeftOutput>,
                TRightOutput: Send + Sync,
                TRight: Parser<Output = TRightOutput>,
                TFinalOutput: Send + Sync,
                FLeftMapper: Fn(TLeftOutput) -> TFinalOutput + Send + Sync + Clone,
                FRightMapper: Fn(TRightOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                pub(super) left: TLeft,
                pub(super) right: TRight,
                pub(super) if_left: FLeftMapper,
                pub(super) if_right: FRightMapper,
                pub(super) kind: String,
                pub(super) _first_output_marker: PhantomData<TLeftOutput>,
                pub(super) _second_output_marker: PhantomData<TRightOutput>,
                pub(super) _final_output_marker: PhantomData<TFinalOutput>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<
                    TLeftOutput: ::core::fmt::Debug,
                    TLeft: ::core::fmt::Debug,
                    TRightOutput: ::core::fmt::Debug,
                    TRight: ::core::fmt::Debug,
                    TFinalOutput: ::core::fmt::Debug,
                    FLeftMapper: ::core::fmt::Debug,
                    FRightMapper: ::core::fmt::Debug,
                > ::core::fmt::Debug
                for EitherParser<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    TFinalOutput,
                    FLeftMapper,
                    FRightMapper,
                >
            where
                TLeftOutput: Send + Sync,
                TLeft: Parser<Output = TLeftOutput>,
                TRightOutput: Send + Sync,
                TRight: Parser<Output = TRightOutput>,
                TFinalOutput: Send + Sync,
                FLeftMapper: Fn(TLeftOutput) -> TFinalOutput + Send + Sync + Clone,
                FRightMapper: Fn(TRightOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        EitherParser {
                            left: ref __self_0_0,
                            right: ref __self_0_1,
                            if_left: ref __self_0_2,
                            if_right: ref __self_0_3,
                            kind: ref __self_0_4,
                            _first_output_marker: ref __self_0_5,
                            _second_output_marker: ref __self_0_6,
                            _final_output_marker: ref __self_0_7,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "EitherParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "left",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "right",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "if_left",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "if_right",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_4),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_first_output_marker",
                                &&(*__self_0_5),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_second_output_marker",
                                &&(*__self_0_6),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "_final_output_marker",
                                &&(*__self_0_7),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    TFinalOutput,
                    FLeftMapper,
                    FRightMapper,
                >
                EitherParser<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    TFinalOutput,
                    FLeftMapper,
                    FRightMapper,
                >
            where
                TLeftOutput: Send + Sync,
                TLeft: Parser<Output = TLeftOutput>,
                TRightOutput: Send + Sync,
                TRight: Parser<Output = TRightOutput>,
                TFinalOutput: Send + Sync,
                FLeftMapper: Fn(TLeftOutput) -> TFinalOutput + Send + Sync + Clone,
                FRightMapper: Fn(TRightOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                /// Creates a new EitherParser which returns the value given from one of the given mappers.
                ///
                /// # Arguments
                /// * `left`      - The first parser to run.
                /// * `right`     - The parser to run if `left` doesn't match.
                /// * `if_left`   - The mapping function to run if `left` matches.
                /// * `if_right`  - The mapping function to run if `right` matches.
                pub fn with_mappers(
                    left: TLeft,
                    right: TRight,
                    if_left: FLeftMapper,
                    if_right: FRightMapper,
                ) -> Self {
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&left.kind(), &right.kind()) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["(", ") or (", ")"], args)
                                },
                            },
                        );
                        res
                    };
                    Self {
                        left,
                        right,
                        if_left,
                        if_right,
                        kind,
                        _first_output_marker: PhantomData,
                        _second_output_marker: PhantomData,
                        _final_output_marker: PhantomData,
                    }
                }
            }
            impl<TLeftOutput, TLeft, TRightOutput, TRight>
                EitherParser<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    Result<TLeftOutput, TRightOutput>,
                    fn(TLeftOutput) -> Result<TLeftOutput, TRightOutput>,
                    fn(TRightOutput) -> Result<TLeftOutput, TRightOutput>,
                >
            where
                TLeftOutput: Send + Sync,
                TLeft: Parser<Output = TLeftOutput>,
                TRightOutput: Send + Sync,
                TRight: Parser<Output = TRightOutput>,
            {
                /// Creates a new EitherParser which returns a `Result<TLeftOutput, TRightOutput>`.
                ///
                /// The "error" case in the `Result` is not a true error, it only means that the right parser succeeded and the left one didn't.
                ///
                /// # Arguments
                /// * `left`      - The first parser to run.
                /// * `right`     - The parser to run if `left` doesn't match.
                pub fn new(left: TLeft, right: TRight) -> Self {
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&left.kind(), &right.kind()) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["(", ") or (", ")"], args)
                                },
                            },
                        );
                        res
                    };
                    Self {
                        left,
                        right,
                        if_left: Ok,
                        if_right: Err,
                        kind,
                        _first_output_marker: PhantomData,
                        _second_output_marker: PhantomData,
                        _final_output_marker: PhantomData,
                    }
                }
            }
        }
        pub mod parser {
            use super::eitherparser::EitherParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    TFinalOutput,
                    FLeftMapper,
                    FRightMapper,
                > Parser
                for EitherParser<
                    TLeftOutput,
                    TLeft,
                    TRightOutput,
                    TRight,
                    TFinalOutput,
                    FLeftMapper,
                    FRightMapper,
                >
            where
                TLeftOutput: Send + Sync,
                TLeft: Parser<Output = TLeftOutput>,
                TRightOutput: Send + Sync,
                TRight: Parser<Output = TRightOutput>,
                TFinalOutput: Send + Sync,
                FLeftMapper: Fn(TLeftOutput) -> TFinalOutput + Send + Sync + Clone,
                FRightMapper: Fn(TRightOutput) -> TFinalOutput + Send + Sync + Clone,
            {
                type Output = TFinalOutput;
                fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TFinalOutput> {
                    self.left
                        .parse(cursor)
                        .map(self.if_left.clone())
                        .or_else(|e| {
                            self.right
                                .parse(cursor)
                                .map(self.if_right.clone())
                                .map_reason(|r| {
                                    let res = ::alloc::fmt::format(
                                        match match (
                                            &self.left.kind(),
                                            &e.reason(),
                                            &self.right.kind(),
                                            &r,
                                        ) {
                                            (arg0, arg1, arg2, arg3) => [
                                                ::core::fmt::ArgumentV1::new(
                                                    arg0,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg1,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg2,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                                ::core::fmt::ArgumentV1::new(
                                                    arg3,
                                                    ::core::fmt::Display::fmt,
                                                ),
                                            ],
                                        } {
                                            ref args => unsafe {
                                                ::core::fmt::Arguments::new_v1(
                                                    &[
                                                        "Couldn\'t parse ",
                                                        " because ",
                                                        ". Couldn\'t parse ",
                                                        " because ",
                                                    ],
                                                    args,
                                                )
                                            },
                                        },
                                    );
                                    res
                                })
                        })
                }
                fn kind(&self) -> &str {
                    &self.kind
                }
            }
        }
    }
    pub mod empty {
        pub mod clone {
            use super::emptyparser::EmptyParser;
            impl<TOutput, FOutputFactory> Clone for EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                fn clone(&self) -> Self {
                    EmptyParser {
                        factory: self.factory.clone(),
                    }
                }
            }
        }
        pub mod emptyparser {
            use c3po_parser_macro::parser;
            /// A parser that doesn't consume any input and just returns a value.
            pub struct EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                pub(super) factory: FOutputFactory,
            }
            impl<
                    TOutput,
                    FOutputFactory,
                    __TOtherOutput,
                    __TOtherParser: crate::parser::parser::Parser<Output = __TOtherOutput>,
                > std::ops::Add<__TOtherParser> for EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                type Output = crate::parsers::compose::composeparser::ComposeParser<
                    TOutput,
                    Self,
                    __TOtherOutput,
                    (TOutput, __TOtherOutput),
                    fn(TOutput, __TOtherOutput) -> (TOutput, __TOtherOutput),
                    __TOtherParser,
                >;
                fn add(self, rhs: __TOtherParser) -> Self::Output {
                    crate::parsers::compose::composeparser::ComposeParser::new(self, rhs)
                }
            }
            impl<
                    TOutput,
                    FOutputFactory,
                    __TOtherOutput,
                    __TOtherParser: crate::parser::parser::Parser<Output = __TOtherOutput>,
                > std::ops::BitOr<__TOtherParser> for EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                type Output = EitherParser<
                    TOutput,
                    Self,
                    __TOtherOutput,
                    __TOtherParser,
                    Result<TOutput, __TOtherOutput>,
                    fn(TOutput) -> Result<TOutput, __TOtherOutput>,
                    fn(__TOtherOutput) -> Result<TOutput, __TOtherOutput>,
                >;
                fn bitor(self, rhs: __TOtherParser) -> Self::Output {
                    EitherParser::new(self, rhs)
                }
            }
            impl<TOutput, FOutputFactory> Clone for EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                fn clone(&self) -> Self {
                    Self {
                        factory: factory.clone(),
                    }
                }
            }
            impl<TOutput, FOutputFactory> std::ops::Mul<usize> for EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                type Output = crate::parsers::repeat::repeatparser::RepeatParser<TOutput, Self>;
                fn mul(self, rhs: usize) -> Self::Output {
                    c3po::parsers::repeat::repeatparser::RepeatParser::count(self, rhs)
                }
            }
            impl<TOutput, FOutputFactory, __TRangeBound: std::ops::RangeBounds<usize>>
                std::ops::Mul<__TRangeBound> for EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                type Output = crate::parsers::repeat::repeatparser::RepeatParser<TOutput, Self>;
                fn mul(self, rhs: __TRangeBound) -> Self::Output {
                    crate::parsers::repeat::repeatparser::RepeatParser::range(self, rhs)
                }
            }
            impl<TOutput, FOutputFactory> EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                /// Creates an EmptyParser that returns values from a factory function.
                pub fn using_factory(factory: FOutputFactory) -> Self {
                    EmptyParser { factory }
                }
            }
            impl EmptyParser<(), fn() -> ()> {
                /// Creates an EmptyParser that returns the unit value (`()`).
                pub fn new() -> Self {
                    EmptyParser { factory: || () }
                }
            }
        }
        pub mod parser {
            use super::emptyparser::EmptyParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl<TOutput, FOutputFactory> Parser for EmptyParser<TOutput, FOutputFactory>
            where
                TOutput: Send + Sync,
                FOutputFactory: Fn() -> TOutput + Clone + Send + Sync,
            {
                type Output = TOutput;
                fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TOutput> {
                    Parse::success(cursor, (self.factory)())
                }
                fn kind(&self) -> &str {
                    "empty"
                }
            }
        }
    }
    pub mod many {
        pub mod clone {
            use super::manyparser::ManyParser;
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            impl<TOutput, TParser> Clone for ManyParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                fn clone(&self) -> Self {
                    ManyParser {
                        parser: self.parser.clone(),
                        kind: self.kind.clone(),
                        o: PhantomData,
                    }
                }
            }
        }
        pub mod manyparser {
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            /// Matches a Parser 0 or more times.
            pub struct ManyParser<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> {
                pub(super) o: PhantomData<TOutput>,
                pub(super) parser: TParser,
                pub(super) kind: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<
                    TOutput: ::core::fmt::Debug + Send + Sync,
                    TParser: ::core::fmt::Debug + Parser<Output = TOutput>,
                > ::core::fmt::Debug for ManyParser<TOutput, TParser>
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        ManyParser {
                            o: ref __self_0_0,
                            parser: ref __self_0_1,
                            kind: ref __self_0_2,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "ManyParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "o",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "parser",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> ManyParser<TOutput, TParser> {
                /// Creates a new ManyParser that matches the given parser 0 or more times.
                pub fn new(parser: TParser) -> ManyParser<TOutput, TParser> {
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&parser.kind(),) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["many(", ")"], args)
                                },
                            },
                        );
                        res
                    };
                    ManyParser {
                        parser,
                        o: PhantomData,
                        kind,
                    }
                }
            }
        }
        pub mod parser {
            use super::manyparser::ManyParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl<TOutput, TParser> Parser for ManyParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                type Output = Vec<TOutput>;
                fn parse<'a>(&self, mut cursor: Option<Cursor<'a>>) -> Parse<'a, Vec<TOutput>> {
                    let mut results = Vec::new();
                    while let Some(c) = cursor {
                        let parse = self.parser.parse(Some(c));
                        if let Parse::Success(success) = parse {
                            cursor = *success.next();
                            results.push(success.into_value());
                        } else {
                            break;
                        }
                    }
                    Parse::success(cursor, results)
                }
                fn kind(&self) -> &str {
                    &self.kind
                }
            }
        }
    }
    pub mod map {
        pub mod clone {
            use super::mapparser::MapParser;
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            impl<TInput, TInputParser, TOutput, FMapper> Clone
                for MapParser<TInput, TInputParser, TOutput, FMapper>
            where
                TInput: Send + Sync,
                TInputParser: Parser<Output = TInput>,
                TOutput: Send + Sync,
                FMapper: Fn(TInput) -> TOutput + Clone + Send + Sync,
            {
                fn clone(&self) -> Self {
                    MapParser {
                        parser: self.parser.clone(),
                        kind: self.kind.clone(),
                        mapper: self.mapper.clone(),
                        o: PhantomData,
                        i: PhantomData,
                    }
                }
            }
        }
        pub mod map {
            use super::mapparser::MapParser;
            use crate::parser::parser::Parser;
            pub trait Map<TInput, TOutput>
            where
                TInput: Send + Sync,
                TOutput: Send + Sync,
                Self: Parser<Output = TInput>,
            {
                fn map<TKind, FKindMapper, FValueMapper>(
                    self,
                    kind_mapper: FKindMapper,
                    value_mapper: FValueMapper,
                ) -> MapParser<TInput, Self, TOutput, FValueMapper>
                where
                    TKind: Into<String>,
                    FKindMapper: FnOnce(&str) -> TKind,
                    FValueMapper: Fn(TInput) -> TOutput + Clone + Send + Sync;
                fn map_kind<TKind, FKindMapper>(
                    self,
                    kind_mapper: FKindMapper,
                ) -> MapParser<TInput, Self, TInput, fn(TInput) -> TInput>
                where
                    TKind: Into<String>,
                    FKindMapper: FnOnce(&str) -> TKind;
                fn map_value<FValueMapper>(
                    self,
                    value_mapper: FValueMapper,
                ) -> MapParser<TInput, Self, TOutput, FValueMapper>
                where
                    FValueMapper: Fn(TInput) -> TOutput + Clone + Send + Sync;
            }
            impl<TInput, TParser, TOutput> Map<TInput, TOutput> for TParser
            where
                TInput: Send + Sync,
                TParser: Parser<Output = TInput>,
                TOutput: Send + Sync,
            {
                fn map<TKind, FKindMapper, FValueMapper>(
                    self,
                    kind_mapper: FKindMapper,
                    value_mapper: FValueMapper,
                ) -> MapParser<TInput, Self, TOutput, FValueMapper>
                where
                    TKind: Into<String>,
                    FKindMapper: FnOnce(&str) -> TKind,
                    FValueMapper: Fn(TInput) -> TOutput + Clone + Send + Sync,
                {
                    MapParser::new(self, kind_mapper, value_mapper)
                }
                fn map_kind<TKind, FKindMapper>(
                    self,
                    kind_mapper: FKindMapper,
                ) -> MapParser<TInput, Self, TInput, fn(TInput) -> TInput>
                where
                    TKind: Into<String>,
                    FKindMapper: FnOnce(&str) -> TKind,
                {
                    MapParser::new_kind(self, kind_mapper)
                }
                fn map_value<FValueMapper>(
                    self,
                    value_mapper: FValueMapper,
                ) -> MapParser<TInput, Self, TOutput, FValueMapper>
                where
                    FValueMapper: Fn(TInput) -> TOutput + Clone + Send + Sync,
                {
                    MapParser::new_value(self, value_mapper)
                }
            }
        }
        pub mod mapparser {
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            /// Matches a Parser 0 or more times.
            pub struct MapParser<TInput, TInputParser, TOutput, FMapper>
            where
                TInput: Send + Sync,
                TInputParser: Parser<Output = TInput>,
                TOutput: Send + Sync,
                FMapper: Fn(TInput) -> TOutput + Clone + Send + Sync,
            {
                pub(super) i: PhantomData<TInput>,
                pub(super) o: PhantomData<TOutput>,
                pub(super) parser: TInputParser,
                pub(super) kind: String,
                pub(super) mapper: FMapper,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<
                    TInput: ::core::fmt::Debug,
                    TInputParser: ::core::fmt::Debug,
                    TOutput: ::core::fmt::Debug,
                    FMapper: ::core::fmt::Debug,
                > ::core::fmt::Debug for MapParser<TInput, TInputParser, TOutput, FMapper>
            where
                TInput: Send + Sync,
                TInputParser: Parser<Output = TInput>,
                TOutput: Send + Sync,
                FMapper: Fn(TInput) -> TOutput + Clone + Send + Sync,
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        MapParser {
                            i: ref __self_0_0,
                            o: ref __self_0_1,
                            parser: ref __self_0_2,
                            kind: ref __self_0_3,
                            mapper: ref __self_0_4,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "MapParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "i",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "o",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "parser",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_3),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "mapper",
                                &&(*__self_0_4),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl<TInput, TInputParser, TOutput, FMapper> MapParser<TInput, TInputParser, TOutput, FMapper>
            where
                TInput: Send + Sync,
                TInputParser: Parser<Output = TInput>,
                TOutput: Send + Sync,
                FMapper: Fn(TInput) -> TOutput + Clone + Send + Sync,
            {
                /// Maps the output of `parser` using a `value_mapper` function. Also maps the kind of `parser` using a `kind_mapper` function.
                ///
                /// # Arguments
                /// * `parser`       - The parser to transform.
                /// * `kind_mapper`  - A function that transforms the old `kind()` into a new `kind()`.
                /// * `value_mapper` - A function that transforms the old output into a new output.
                pub fn new<TKind: Into<String>, FKindMapper: FnOnce(&str) -> TKind>(
                    parser: TInputParser,
                    kind_mapper: FKindMapper,
                    value_mapper: FMapper,
                ) -> Self {
                    let kind = parser.kind();
                    let new_kind = kind_mapper(kind).into();
                    MapParser {
                        parser,
                        kind: new_kind,
                        mapper: value_mapper,
                        o: PhantomData,
                        i: PhantomData,
                    }
                }
                /// Maps the output of `parser` using a `value_mapper` function. The `kind()` of the new parser will be equal to the old one.
                ///
                /// # Arguments
                /// * `parser`       - The parser to transform.
                /// * `value_mapper` - A function that transforms the old output into a new output.
                pub fn new_value(parser: TInputParser, value_mapper: FMapper) -> Self {
                    Self::new(parser, |s| s.to_string(), value_mapper)
                }
            }
            impl<TInput, TInputParser> MapParser<TInput, TInputParser, TInput, fn(TInput) -> TInput>
            where
                TInput: Send + Sync,
                TInputParser: Parser<Output = TInput>,
            {
                /// Maps the `kind()` of `parser` with a `kind_mapper` function. The output is the same as the old parser.
                ///
                /// # Arguments
                /// * `parser`       - The parser to transform.
                /// * `kind_mapper`  - A function that transforms the old `kind()` into a new `kind()`.
                pub fn new_kind<TKind: Into<String>, FKindMapper: FnOnce(&str) -> TKind>(
                    parser: TInputParser,
                    kind_mapper: FKindMapper,
                ) -> Self {
                    Self::new(parser, kind_mapper, |v| v)
                }
            }
        }
        pub mod parser {
            use super::mapparser::MapParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl<TInput, TInputParser, TOutput, FMapper> Parser
                for MapParser<TInput, TInputParser, TOutput, FMapper>
            where
                TInput: Send + Sync,
                TInputParser: Parser<Output = TInput>,
                TOutput: Send + Sync,
                FMapper: Fn(TInput) -> TOutput + Clone + Send + Sync,
            {
                type Output = TOutput;
                fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, TOutput> {
                    self.parser.parse(cursor).map(self.mapper.clone())
                }
                fn kind(&self) -> &str {
                    &self.kind
                }
            }
        }
    }
    pub mod regex {
        pub mod parser {
            use super::regexparser::RegexParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl Parser for RegexParser {
                type Output = String;
                fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, String> {
                    let slice = match cursor {
                        Some(s) => s.current_str(),
                        None => "",
                    };
                    let mat = match self.regex.find(slice) {
                        Ok(Some(s)) => s,
                        _ => return Parse::failure(cursor, "The regex didn't match."),
                    };
                    if mat.start() != 0 {
                        Parse::failure(
                            cursor,
                            "The regex did not match the beginning of the string.",
                        )
                    } else {
                        let ret = mat.as_str().to_string();
                        Parse::success(cursor.and_then(|c| c + ret.chars().count()), ret)
                    }
                }
                fn kind(&self) -> &str {
                    &self.kind
                }
            }
        }
        pub mod regexparser {
            use fancy_regex::Regex;
            use std::str::FromStr;
            /// Matches the input using a regular expression.
            pub struct RegexParser {
                pub(super) regex: Regex,
                pub(super) kind: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for RegexParser {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        RegexParser {
                            regex: ref __self_0_0,
                            kind: ref __self_0_1,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "RegexParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "regex",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_1),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for RegexParser {
                #[inline]
                fn clone(&self) -> RegexParser {
                    match *self {
                        RegexParser {
                            regex: ref __self_0_0,
                            kind: ref __self_0_1,
                        } => RegexParser {
                            regex: ::core::clone::Clone::clone(&(*__self_0_0)),
                            kind: ::core::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl RegexParser {
                /// Creates a new `RegexParser` out of the given string.
                ///
                /// # Arguments
                /// * `s` - A `fancy_regex` style regular expression. Performance is improved if this regex starts with `^`, although this is not required for this parser to function correctly.
                pub fn new<S: Into<String>>(s: S) -> RegexParser {
                    let pattern = s.into();
                    let regex = Regex::from_str(&pattern).expect(&{
                        let res = ::alloc::fmt::format(
                            match match (&pattern,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(
                                        &["The regex \'", "\' is invalid."],
                                        args,
                                    )
                                },
                            },
                        );
                        res
                    });
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&pattern,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["regex(", ")"], args)
                                },
                            },
                        );
                        res
                    };
                    RegexParser { regex, kind }
                }
            }
        }
    }
    pub mod repeat {
        pub mod clone {
            use super::repeatparser::RepeatParser;
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            impl<TOutput, TParser> Clone for RepeatParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                fn clone(&self) -> Self {
                    Self {
                        parser: self.parser.clone(),
                        kind: self.kind.clone(),
                        bounds: self.bounds.clone(),
                        o: PhantomData,
                    }
                }
            }
        }
        pub mod mul {
            use super::repeatparser::RepeatParser;
            use crate::parser::parser::Parser;
            use std::ops::RangeBounds;
            pub trait ParserMul<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                fn multiply(self, count: usize) -> RepeatParser<TOutput, TParser>;
                fn repeat<T: RangeBounds<usize>>(self, range: T) -> RepeatParser<TOutput, TParser>;
            }
            impl<TOutput, TParser> ParserMul<TOutput, TParser> for TParser
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                fn multiply(self, count: usize) -> RepeatParser<TOutput, TParser> {
                    RepeatParser::count(self, count)
                }
                fn repeat<T: RangeBounds<usize>>(self, range: T) -> RepeatParser<TOutput, TParser> {
                    RepeatParser::range(self, range)
                }
            }
        }
        pub mod parser {
            use super::repeatparser::RepeatParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl<TOutput, TParser> Parser for RepeatParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                type Output = Vec<TOutput>;
                fn parse<'a>(&self, mut cursor: Option<Cursor<'a>>) -> Parse<'a, Vec<TOutput>> {
                    let (lb , ub) = match self . bounds { Some (s) => s , None => { return Parse :: failure (cursor , "This parser cannot parse anything because its range has no valid values.") } } ;
                    let mut results = Vec::new();
                    while let Some(c) = cursor {
                        if ub.clone().map(|n| results.len() == n).unwrap_or(false) {
                            break;
                        }
                        let parse = self.parser.parse(Some(c));
                        if let Parse::Success(success) = parse {
                            cursor = *success.next();
                            results.push(success.into_value());
                        } else {
                            break;
                        }
                    }
                    if results.len() < lb {
                        Parse::failure(cursor, {
                            let res = ::alloc::fmt::format(
                                match match (&lb, &results.len()) {
                                    (arg0, arg1) => [
                                        ::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        ),
                                        ::core::fmt::ArgumentV1::new(
                                            arg1,
                                            ::core::fmt::Display::fmt,
                                        ),
                                    ],
                                } {
                                    ref args => unsafe {
                                        ::core::fmt::Arguments::new_v1(
                                            &[
                                                "Expected at least ",
                                                " values, but was only able to parse ",
                                            ],
                                            args,
                                        )
                                    },
                                },
                            );
                            res
                        })
                    } else {
                        Parse::success(cursor, results)
                    }
                }
                fn kind(&self) -> &str {
                    &self.kind
                }
            }
        }
        pub mod repeatparser {
            use crate::parser::parser::Parser;
            use std::marker::PhantomData;
            use std::ops::Bound;
            use std::ops::RangeBounds;
            /// Matches a Parser 0 or more times.
            pub struct RepeatParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                pub(super) bounds: Option<(usize, Option<usize>)>,
                pub(super) parser: TParser,
                pub(super) kind: String,
                pub(super) o: PhantomData<TOutput>,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<TOutput: ::core::fmt::Debug, TParser: ::core::fmt::Debug> ::core::fmt::Debug
                for RepeatParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        RepeatParser {
                            bounds: ref __self_0_0,
                            parser: ref __self_0_1,
                            kind: ref __self_0_2,
                            o: ref __self_0_3,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "RepeatParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "bounds",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "parser",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_2),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "o",
                                &&(*__self_0_3),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            fn range_to_values<TRange: RangeBounds<usize>>(
                range: &TRange,
            ) -> Option<(usize, Option<usize>)> {
                let lb = match range.start_bound() {
                    Bound::Included(n) => *n,
                    Bound::Excluded(n) => n + 1,
                    Bound::Unbounded => 0,
                };
                let rb = match range.end_bound() {
                    Bound::Included(n) => Some(*n),
                    Bound::Excluded(n) => {
                        if *n == 0 || *n <= lb {
                            return None;
                        } else {
                            Some(n - 1)
                        }
                    }
                    Bound::Unbounded => None,
                };
                Some((lb, rb))
            }
            fn format_range<TRange: RangeBounds<usize>>(range: &TRange) -> String {
                let (lb, rb) = match range_to_values(range) {
                    Some((lb, rb)) => (lb, rb),
                    None => return "never".to_string(),
                };
                match rb {
                    Some(rb) => {
                        if lb == 0 {
                            {
                                let res = ::alloc::fmt::format(
                                    match match (&rb,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    } {
                                        ref args => unsafe {
                                            ::core::fmt::Arguments::new_v1(&["<="], args)
                                        },
                                    },
                                );
                                res
                            }
                        } else if lb == rb {
                            lb.to_string()
                        } else {
                            {
                                let res = ::alloc::fmt::format(
                                    match match (&lb, &rb) {
                                        (arg0, arg1) => [
                                            ::core::fmt::ArgumentV1::new(
                                                arg0,
                                                ::core::fmt::Display::fmt,
                                            ),
                                            ::core::fmt::ArgumentV1::new(
                                                arg1,
                                                ::core::fmt::Display::fmt,
                                            ),
                                        ],
                                    } {
                                        ref args => unsafe {
                                            ::core::fmt::Arguments::new_v1(&["", "-"], args)
                                        },
                                    },
                                );
                                res
                            }
                        }
                    }
                    None => {
                        if lb == 0 {
                            "any".to_string()
                        } else {
                            {
                                let res = ::alloc::fmt::format(
                                    match match (&lb,) {
                                        (arg0,) => [::core::fmt::ArgumentV1::new(
                                            arg0,
                                            ::core::fmt::Display::fmt,
                                        )],
                                    } {
                                        ref args => unsafe {
                                            ::core::fmt::Arguments::new_v1(&[">="], args)
                                        },
                                    },
                                );
                                res
                            }
                        }
                    }
                }
            }
            impl<TOutput, TParser> RepeatParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                /// Creates a new RepeatParser that matches a `parser` `range` times.
                ///
                /// # Arguments
                /// * `parser` - The parser to repeat.
                /// * `range`  - The amount of times to match `parser`. For example, `3..6` will match the parser 3 to 5 times.
                pub fn range<TRange: RangeBounds<usize>>(parser: TParser, range: TRange) -> Self {
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&parser.kind(), &format_range(&range)) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["(", ") * "], args)
                                },
                            },
                        );
                        res
                    };
                    let bounds = range_to_values(&range);
                    RepeatParser {
                        o: PhantomData,
                        bounds,
                        parser,
                        kind,
                    }
                }
                /// Creates a new RepeatParser that matches a `parser` `range` times.
                ///
                /// # Arguments
                /// * `parser` - The parser to repeat.
                /// * `count`  - The amount of times to match `parser`.
                pub fn count(parser: TParser, count: usize) -> Self {
                    Self::range(parser, count..=count)
                }
            }
        }
    }
    pub mod some {
        pub mod clone {
            use super::someparser::SomeParser;
            use crate::parser::parser::Parser;
            impl<TOutput, TParser> Clone for SomeParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                fn clone(&self) -> Self {
                    SomeParser {
                        parser: self.parser.clone(),
                        kind: self.kind.clone(),
                        base_kind: self.base_kind.clone(),
                    }
                }
            }
        }
        pub mod parser {
            use super::someparser::SomeParser;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl<TOutput, TParser> Parser for SomeParser<TOutput, TParser>
            where
                TOutput: Send + Sync,
                TParser: Parser<Output = TOutput>,
            {
                type Output = Vec<TOutput>;
                fn parse<'a>(&self, cursor: Option<Cursor<'a>>) -> Parse<'a, Vec<TOutput>> {
                    self.parser.parse(cursor).map_reason(|r| {
                        let res = ::alloc::fmt::format(
                            match match (&self.base_kind, &r) {
                                (arg0, arg1) => [
                                    ::core::fmt::ArgumentV1::new(arg0, ::core::fmt::Display::fmt),
                                    ::core::fmt::ArgumentV1::new(arg1, ::core::fmt::Display::fmt),
                                ],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(
                                        &["Expected at least one ", ", but didn\'t read it: "],
                                        args,
                                    )
                                },
                            },
                        );
                        res
                    })
                }
                fn kind(&self) -> &str {
                    &self.kind
                }
            }
        }
        pub mod someparser {
            use super::super::compose::composeparser::ComposeParser;
            use super::super::many::manyparser::ManyParser;
            use crate::parser::parser::Parser;
            /// Matches a `Parser` 1 or more times.
            pub struct SomeParser<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> {
                pub(super) parser: ComposeParser<
                    TOutput,
                    TParser,
                    Vec<TOutput>,
                    ManyParser<TOutput, TParser>,
                    Vec<TOutput>,
                    fn(TOutput, Vec<TOutput>) -> Vec<TOutput>,
                >,
                pub(super) base_kind: String,
                pub(super) kind: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl<
                    TOutput: ::core::fmt::Debug + Send + Sync,
                    TParser: ::core::fmt::Debug + Parser<Output = TOutput>,
                > ::core::fmt::Debug for SomeParser<TOutput, TParser>
            {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        SomeParser {
                            parser: ref __self_0_0,
                            base_kind: ref __self_0_1,
                            kind: ref __self_0_2,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "SomeParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "parser",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "base_kind",
                                &&(*__self_0_1),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_2),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            impl<TOutput: Send + Sync, TParser: Parser<Output = TOutput>> SomeParser<TOutput, TParser> {
                /// Creates a new `ManyParser` that matches the given parser 1 or more times.
                pub fn new(parser: TParser) -> SomeParser<TOutput, TParser> {
                    let many = ManyParser::new(parser.clone());
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&parser.kind(),) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["some(", ")"], args)
                                },
                            },
                        );
                        res
                    };
                    let base_kind = parser.kind().to_string();
                    let compose = ComposeParser::using_combiner(
                        parser,
                        many,
                        (|f, mut v| {
                            v.insert(0, f);
                            v
                        }) as fn(TOutput, Vec<TOutput>) -> Vec<TOutput>,
                    );
                    SomeParser {
                        parser: compose,
                        kind,
                        base_kind,
                    }
                }
            }
        }
    }
    pub mod string {
        pub mod parser {
            use super::stringparser::StringParser;
            use crate::immut_iter::immut_iterable::ImmutableIterable;
            use crate::parser::cursor::cursor::Cursor;
            use crate::parser::parse::parse::Parse;
            use crate::parser::parser::Parser;
            impl Parser for StringParser {
                type Output = String;
                fn parse<'a>(&self, mut cursor: Option<Cursor<'a>>) -> Parse<'a, String> {
                    let mut char_iter = self.string().chars().peekable();
                    while let Some(c) = cursor {
                        match char_iter.peek() {
                            Some(char) => {
                                if char != &c.current() {
                                    return Parse::failure(Some(c), {
                                        let res = ::alloc::fmt::format(
                                            match match (&c.current(), &char) {
                                                (arg0, arg1) => [
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg0,
                                                        ::core::fmt::Display::fmt,
                                                    ),
                                                    ::core::fmt::ArgumentV1::new(
                                                        arg1,
                                                        ::core::fmt::Display::fmt,
                                                    ),
                                                ],
                                            } {
                                                ref args => unsafe {
                                                    ::core::fmt::Arguments::new_v1(
                                                        &["Expected \'", "\', got \'", "\'"],
                                                        args,
                                                    )
                                                },
                                            },
                                        );
                                        res
                                    });
                                }
                            }
                            None => break,
                        }
                        cursor = c.next_immut();
                        char_iter.next();
                    }
                    if let Some(char) = char_iter.peek() {
                        Parse::failure(None, {
                            let res = ::alloc::fmt::format(
                                match match (&char,) {
                                    (arg0,) => [::core::fmt::ArgumentV1::new(
                                        arg0,
                                        ::core::fmt::Display::fmt,
                                    )],
                                } {
                                    ref args => unsafe {
                                        ::core::fmt::Arguments::new_v1(
                                            &[
                                                "Expected \'",
                                                "\', but reached EOF before it could be read.",
                                            ],
                                            args,
                                        )
                                    },
                                },
                            );
                            res
                        })
                    } else {
                        Parse::success(cursor, self.string.clone())
                    }
                }
                fn kind(&self) -> &str {
                    &self.string
                }
            }
        }
        pub mod stringparser {
            /// A parser that matches a string.
            pub struct StringParser {
                pub(super) string: String,
                pub(super) kind: String,
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::fmt::Debug for StringParser {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    match *self {
                        StringParser {
                            string: ref __self_0_0,
                            kind: ref __self_0_1,
                        } => {
                            let debug_trait_builder =
                                &mut ::core::fmt::Formatter::debug_struct(f, "StringParser");
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "string",
                                &&(*__self_0_0),
                            );
                            let _ = ::core::fmt::DebugStruct::field(
                                debug_trait_builder,
                                "kind",
                                &&(*__self_0_1),
                            );
                            ::core::fmt::DebugStruct::finish(debug_trait_builder)
                        }
                    }
                }
            }
            #[automatically_derived]
            #[allow(unused_qualifications)]
            impl ::core::clone::Clone for StringParser {
                #[inline]
                fn clone(&self) -> StringParser {
                    match *self {
                        StringParser {
                            string: ref __self_0_0,
                            kind: ref __self_0_1,
                        } => StringParser {
                            string: ::core::clone::Clone::clone(&(*__self_0_0)),
                            kind: ::core::clone::Clone::clone(&(*__self_0_1)),
                        },
                    }
                }
            }
            impl StringParser {
                /// Creates a new `StringParser` out of the given string.
                pub fn new<I: Into<String>>(s: I) -> StringParser {
                    let string = s.into();
                    let kind = {
                        let res = ::alloc::fmt::format(
                            match match (&&string,) {
                                (arg0,) => [::core::fmt::ArgumentV1::new(
                                    arg0,
                                    ::core::fmt::Display::fmt,
                                )],
                            } {
                                ref args => unsafe {
                                    ::core::fmt::Arguments::new_v1(&["\'", "\'"], args)
                                },
                            },
                        );
                        res
                    };
                    StringParser { string, kind }
                }
                /// The string that this parser is matching.
                pub fn string(&self) -> &str {
                    &self.string
                }
            }
        }
    }
}
