// Tok Stream Library
//
//
// MIT License
//
// Copyright (c) 2021, 2022 Systopia Lab, Computer Science, University of British Columbia
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

//! # Token Representation
//!
//! A token corresponds to a lexed atom of the input soure content. It may simply be a single
//! character, or a sequence of characters from the source file or input string. Thus each
//! token has an associated [SrcSpan] capturing the position of it in the input source.
//!
//! Each token has a specific kind. The library uses generics of the actual token kinds to
//! allow users to define their own valid set of tokens depending on their language they
//! want to write a parser for.

// used standard library modules
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// used third-party libraries
use srcspan::{SrcLoc, SrcSpan};

/// Trait to be implemented by the concrete token kind type.
pub trait TokKind {
    /// whether the token is a keyword
    fn is_keyword(&self) -> bool;

    /// whether the token has a reserved value
    fn is_reserved(&self) -> bool;

    /// whether the token is a comment
    fn is_comment(&self) -> bool;

    /// whether the token is a literal, string or number, keyword, ...
    fn is_literal(&self) -> bool;

    /// whether the token is an identifier
    fn is_identifier(&self) -> bool;

    /// whether or not to keep the token when filtering it
    fn keep(&self) -> bool;
}

/// Represents a Lexical Token
///
/// The token represents a single lexical atom that will be consumed by the parser.
/// It is parameterized by the token kind type that the user needs to define.
///
/// # Example
///
/// `pub type MyTok = Tok<MyTokKind>;`
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Tok<T>
where
    T: TokKind + Display + PartialEq + Clone,
{
    /// the kind of the token, defining its actual type
    kind: T,

    /// the source position of the token
    span: SrcSpan,
}

/// The Tok Implementation
impl<T> Tok<T>
where
    T: TokKind + Display + PartialEq + Clone,
{
    /// Creates a new [Tok] with the given kind and source span
    pub fn new(kind: T, span: SrcSpan) -> Self {
        Tok { kind, span }
    }

    /// Obtains the source span for this token
    pub fn span(&self) -> &SrcSpan {
        &self.span
    }

    /// Obtains the kind of this token
    pub fn kind(&self) -> &T {
        &self.kind
    }

    /// Returns the start of the location of this token
    pub fn loc(&self) -> &SrcLoc {
        self.span.loc()
    }

    /// returns true if the token is a keyword
    pub fn is_keyword(&self) -> bool {
        self.kind.is_keyword()
    }

    /// returns true if the token is a reserved identifier
    pub fn is_reserved(&self) -> bool {
        self.kind.is_reserved()
    }

    /// returns true if the token is to be retained when filtering
    pub fn keep(&self) -> bool {
        self.kind.keep()
    }
}

/// Implementation of the [Display] trait for [Tok]
impl<T> Display for Tok<T>
where
    T: TokKind + Display + PartialEq + Clone,
{
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        Display::fmt(&self.kind, f)
    }
}
