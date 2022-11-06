use std::{borrow::Cow, fmt::Display};

use colored::{ColoredString, Colorize};

pub struct FmtBuilder<'a> {
    options: FmtOptions,
    parts: Vec<Part<'a>>,
}

pub enum Part<'a> {
    Borrowed(&'a str),
    Owned(String),
    Colored(ColoredString),
    Eval(Box<dyn Fn() -> Part<'a>>),
    Empty,
}

#[derive(Default)]
pub struct FmtOptions {
    pub colored: bool,
}

impl<'a> FmtBuilder<'a> {
    /// Creates a new builder
    pub fn new() -> Self {
        Self {
            options: FmtOptions::default(),
            parts: Vec::new(),
        }
    }

    pub fn options(&mut self, options: FmtOptions) -> &mut Self {
        self.options = options;

        self
    }

    pub fn append<S: Into<Part<'a>>>(&mut self, part: S) -> &mut Self {
        self.parts.push(part.into());

        self
    }

    pub fn append_if<F, S>(&mut self, condition: bool, string_fn: F) -> &mut Self
    where
        F: Fn() -> S,
        S: Into<Part<'a>>,
    {
        if condition {
            self.append(string_fn());
        }
        self
    }

    /// Builds a string representation.
    pub fn build(&self) -> String {
        let string_parts = self
            .parts
            .iter()
            .filter_map(|c| c.fmt_string(&self.options))
            .collect::<Vec<_>>();
        let cap = string_parts.iter().map(|c| c.len()).sum();
        string_parts
            .into_iter()
            .fold(String::with_capacity(cap), |mut acc, e| {
                acc.push_str(e.as_ref());
                acc
            })
    }
}

impl<'a> Display for FmtBuilder<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.build().fmt(f)
    }
}

impl<'a> Part<'a> {
    fn fmt_string(&self, opts: &FmtOptions) -> Option<Cow<'_, str>> {
        match &self {
            Part::Borrowed(b) => Some(Cow::Borrowed(b)),
            Part::Owned(o) => Some(Cow::Borrowed(o)),
            Part::Colored(c) => Some(Cow::Owned(Self::fmt_colored(c, opts))),
            Part::Eval(e) => {
                let part = e();
                part.fmt_string(opts).map(Cow::into_owned).map(Cow::Owned)
            }
            Part::Empty => None,
        }
    }

    fn fmt_colored(c: &ColoredString, opts: &FmtOptions) -> String {
        if opts.colored {
            c.to_string()
        } else {
            c.clone().clear().to_string()
        }
    }
}

impl<'a> Into<Part<'a>> for String {
    fn into(self) -> Part<'a> {
        Part::Owned(self)
    }
}

impl<'a> Into<Part<'a>> for &'a str {
    fn into(self) -> Part<'a> {
        Part::Borrowed(self)
    }
}

impl<'a> Into<Part<'a>> for &'a String {
    fn into(self) -> Part<'a> {
        Part::Borrowed(self)
    }
}

impl<'a> Into<Part<'a>> for ColoredString {
    fn into(self) -> Part<'a> {
        Part::Colored(self)
    }
}

impl<'a, P: Into<Part<'a>>> Into<Part<'a>> for Option<P> {
    fn into(self) -> Part<'a> {
        self.map(P::into).unwrap_or(Part::Empty)
    }
}
