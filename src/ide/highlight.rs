use crate::syntax::{ast, LinkedNode, SyntaxKind, SyntaxNode};

/// Syntax highlighting categories.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Category {
    /// A line or block comment.
    Comment,
    /// Punctuation in code.
    Punctuation,
    /// An escape sequence or shorthand.
    Escape,
    /// Strong markup.
    Strong,
    /// Emphasized markup.
    Emph,
    /// A hyperlink.
    Link,
    /// Raw text.
    Raw,
    /// A label.
    Label,
    /// A reference to a label.
    Ref,
    /// A section heading.
    Heading,
    /// A marker of a list, enumeration, or term list.
    ListMarker,
    /// A term in a term list.
    ListTerm,
    /// The delimiters of a math formula.
    MathDelimiter,
    /// An operator with special meaning in a math formula.
    MathOperator,
    /// A keyword.
    Keyword,
    /// An operator in code.
    Operator,
    /// A numeric literal.
    Number,
    /// A string literal.
    String,
    /// A function or method name.
    Function,
    /// An interpolated variable in markup or math.
    Interpolated,
    /// A syntax error.
    Error,
}

impl Category {
    /// Return the recommended TextMate grammar scope for the given highlighting
    /// category.
    pub fn tm_scope(&self) -> &'static str {
        match self {
            Self::Comment => "comment.typst",
            Self::Punctuation => "punctuation.typst",
            Self::Escape => "constant.character.escape.typst",
            Self::Strong => "markup.bold.typst",
            Self::Emph => "markup.italic.typst",
            Self::Link => "markup.underline.link.typst",
            Self::Raw => "markup.raw.typst",
            Self::MathDelimiter => "punctuation.definition.math.typst",
            Self::MathOperator => "keyword.operator.math.typst",
            Self::Heading => "markup.heading.typst",
            Self::ListMarker => "punctuation.definition.list.typst",
            Self::ListTerm => "markup.list.term.typst",
            Self::Label => "entity.name.label.typst",
            Self::Ref => "markup.other.reference.typst",
            Self::Keyword => "keyword.typst",
            Self::Operator => "keyword.operator.typst",
            Self::Number => "constant.numeric.typst",
            Self::String => "string.quoted.double.typst",
            Self::Function => "entity.name.function.typst",
            Self::Interpolated => "meta.interpolation.typst",
            Self::Error => "invalid.typst",
        }
    }

    /// The recommended CSS class for the highlighting category.
    pub fn css_class(self) -> &'static str {
        match self {
            Self::Comment => "typ-comment",
            Self::Punctuation => "typ-punct",
            Self::Escape => "typ-escape",
            Self::Strong => "typ-strong",
            Self::Emph => "typ-emph",
            Self::Link => "typ-link",
            Self::Raw => "typ-raw",
            Self::Label => "typ-label",
            Self::Ref => "typ-ref",
            Self::Heading => "typ-heading",
            Self::ListMarker => "typ-marker",
            Self::ListTerm => "typ-term",
            Self::MathDelimiter => "typ-math-delim",
            Self::MathOperator => "typ-math-op",
            Self::Keyword => "typ-key",
            Self::Operator => "typ-op",
            Self::Number => "typ-num",
            Self::String => "typ-str",
            Self::Function => "typ-func",
            Self::Interpolated => "typ-pol",
            Self::Error => "typ-error",
        }
    }
}

/// Determine the highlight category of a linked syntax node.
///
/// Returns `None` if the node should not be highlighted.
pub fn highlight(node: &LinkedNode) -> Option<Category> {
    match node.kind() {
        SyntaxKind::Markup
            if node.parent_kind() == Some(SyntaxKind::TermItem)
                && node.next_sibling_kind() == Some(SyntaxKind::Colon) =>
        {
            Some(Category::ListTerm)
        }
        SyntaxKind::Markup => None,
        SyntaxKind::Text => None,
        SyntaxKind::Space => None,
        SyntaxKind::Linebreak => Some(Category::Escape),
        SyntaxKind::Parbreak => None,
        SyntaxKind::Escape => Some(Category::Escape),
        SyntaxKind::Shorthand => Some(Category::Escape),
        SyntaxKind::SmartQuote => None,
        SyntaxKind::Strong => Some(Category::Strong),
        SyntaxKind::Emph => Some(Category::Emph),
        SyntaxKind::Raw => Some(Category::Raw),
        SyntaxKind::Link => Some(Category::Link),
        SyntaxKind::Label => Some(Category::Label),
        SyntaxKind::Ref => Some(Category::Ref),
        SyntaxKind::Heading => Some(Category::Heading),
        SyntaxKind::HeadingMarker => None,
        SyntaxKind::ListItem => None,
        SyntaxKind::ListMarker => Some(Category::ListMarker),
        SyntaxKind::EnumItem => None,
        SyntaxKind::EnumMarker => Some(Category::ListMarker),
        SyntaxKind::TermItem => None,
        SyntaxKind::TermMarker => Some(Category::ListMarker),
        SyntaxKind::Formula => None,

        SyntaxKind::Math => None,
        SyntaxKind::MathIdent => highlight_ident(node),
        SyntaxKind::MathAlignPoint => Some(Category::MathOperator),
        SyntaxKind::MathDelimited => None,
        SyntaxKind::MathAttach => None,
        SyntaxKind::MathFrac => None,

        SyntaxKind::Hashtag => highlight_hashtag(node),
        SyntaxKind::LeftBrace => Some(Category::Punctuation),
        SyntaxKind::RightBrace => Some(Category::Punctuation),
        SyntaxKind::LeftBracket => Some(Category::Punctuation),
        SyntaxKind::RightBracket => Some(Category::Punctuation),
        SyntaxKind::LeftParen => Some(Category::Punctuation),
        SyntaxKind::RightParen => Some(Category::Punctuation),
        SyntaxKind::Comma => Some(Category::Punctuation),
        SyntaxKind::Semicolon => Some(Category::Punctuation),
        SyntaxKind::Colon => Some(Category::Punctuation),
        SyntaxKind::Star => match node.parent_kind() {
            Some(SyntaxKind::Strong) => None,
            _ => Some(Category::Operator),
        },
        SyntaxKind::Underscore => match node.parent_kind() {
            Some(SyntaxKind::MathAttach) => Some(Category::MathOperator),
            _ => None,
        },
        SyntaxKind::Dollar => Some(Category::MathDelimiter),
        SyntaxKind::Plus => Some(Category::Operator),
        SyntaxKind::Minus => Some(Category::Operator),
        SyntaxKind::Slash => Some(match node.parent_kind() {
            Some(SyntaxKind::MathFrac) => Category::MathOperator,
            _ => Category::Operator,
        }),
        SyntaxKind::Hat => Some(Category::MathOperator),
        SyntaxKind::Dot => Some(Category::Punctuation),
        SyntaxKind::Eq => match node.parent_kind() {
            Some(SyntaxKind::Heading) => None,
            _ => Some(Category::Operator),
        },
        SyntaxKind::EqEq => Some(Category::Operator),
        SyntaxKind::ExclEq => Some(Category::Operator),
        SyntaxKind::Lt => Some(Category::Operator),
        SyntaxKind::LtEq => Some(Category::Operator),
        SyntaxKind::Gt => Some(Category::Operator),
        SyntaxKind::GtEq => Some(Category::Operator),
        SyntaxKind::PlusEq => Some(Category::Operator),
        SyntaxKind::HyphEq => Some(Category::Operator),
        SyntaxKind::StarEq => Some(Category::Operator),
        SyntaxKind::SlashEq => Some(Category::Operator),
        SyntaxKind::Dots => Some(Category::Operator),
        SyntaxKind::Arrow => Some(Category::Operator),

        SyntaxKind::Not => Some(Category::Keyword),
        SyntaxKind::And => Some(Category::Keyword),
        SyntaxKind::Or => Some(Category::Keyword),
        SyntaxKind::None => Some(Category::Keyword),
        SyntaxKind::Auto => Some(Category::Keyword),
        SyntaxKind::Let => Some(Category::Keyword),
        SyntaxKind::Set => Some(Category::Keyword),
        SyntaxKind::Show => Some(Category::Keyword),
        SyntaxKind::If => Some(Category::Keyword),
        SyntaxKind::Else => Some(Category::Keyword),
        SyntaxKind::For => Some(Category::Keyword),
        SyntaxKind::In => Some(Category::Keyword),
        SyntaxKind::While => Some(Category::Keyword),
        SyntaxKind::Break => Some(Category::Keyword),
        SyntaxKind::Continue => Some(Category::Keyword),
        SyntaxKind::Return => Some(Category::Keyword),
        SyntaxKind::Import => Some(Category::Keyword),
        SyntaxKind::Include => Some(Category::Keyword),
        SyntaxKind::As => Some(Category::Keyword),

        SyntaxKind::Code => None,
        SyntaxKind::Ident => highlight_ident(node),
        SyntaxKind::Bool => Some(Category::Keyword),
        SyntaxKind::Int => Some(Category::Number),
        SyntaxKind::Float => Some(Category::Number),
        SyntaxKind::Numeric => Some(Category::Number),
        SyntaxKind::Str => Some(Category::String),
        SyntaxKind::CodeBlock => None,
        SyntaxKind::ContentBlock => None,
        SyntaxKind::Parenthesized => None,
        SyntaxKind::Array => None,
        SyntaxKind::Dict => None,
        SyntaxKind::Named => None,
        SyntaxKind::Keyed => None,
        SyntaxKind::Unary => None,
        SyntaxKind::Binary => None,
        SyntaxKind::FieldAccess => None,
        SyntaxKind::FuncCall => None,
        SyntaxKind::Args => None,
        SyntaxKind::Spread => None,
        SyntaxKind::Closure => None,
        SyntaxKind::Params => None,
        SyntaxKind::LetBinding => None,
        SyntaxKind::SetRule => None,
        SyntaxKind::ShowRule => None,
        SyntaxKind::Conditional => None,
        SyntaxKind::WhileLoop => None,
        SyntaxKind::ForLoop => None,
        SyntaxKind::ForPattern => None,
        SyntaxKind::ModuleImport => None,
        SyntaxKind::ImportItems => None,
        SyntaxKind::ModuleInclude => None,
        SyntaxKind::LoopBreak => None,
        SyntaxKind::LoopContinue => None,
        SyntaxKind::FuncReturn => None,

        SyntaxKind::LineComment => Some(Category::Comment),
        SyntaxKind::BlockComment => Some(Category::Comment),
        SyntaxKind::Error => Some(Category::Error),
        SyntaxKind::Eof => None,
    }
}

/// Highlight an identifier based on context.
fn highlight_ident(node: &LinkedNode) -> Option<Category> {
    // Are we directly before an argument list?
    let next_leaf = node.next_leaf();
    if let Some(next) = &next_leaf {
        if node.range().end == next.offset()
            && matches!(next.kind(), SyntaxKind::LeftParen | SyntaxKind::LeftBracket)
        {
            return Some(Category::Function);
        }
    }

    // Are we in math?
    if node.kind() == SyntaxKind::MathIdent {
        return Some(Category::Interpolated);
    }

    // Find the first non-field access ancestor.
    let mut ancestor = node;
    while ancestor.parent_kind() == Some(SyntaxKind::FieldAccess) {
        ancestor = ancestor.parent()?;
    }

    // Are we directly before a show rule colon?
    if next_leaf.map(|leaf| leaf.kind()) == Some(SyntaxKind::Colon)
        && ancestor.parent_kind() == Some(SyntaxKind::ShowRule)
    {
        return Some(Category::Function);
    }

    // Are we (or an ancestor field access) directly after a hashtag.
    if ancestor.prev_leaf().map(|leaf| leaf.kind()) == Some(SyntaxKind::Hashtag) {
        return Some(Category::Interpolated);
    }

    // Are we behind a dot, that is behind another identifier?
    let prev = node.prev_leaf()?;
    if prev.kind() == SyntaxKind::Dot {
        let prev_prev = prev.prev_leaf()?;
        if is_ident(&prev_prev) {
            return highlight_ident(&prev_prev);
        }
    }

    None
}

/// Highlight a hashtag based on context.
fn highlight_hashtag(node: &LinkedNode) -> Option<Category> {
    let next = node.next_sibling()?;
    let expr = next.cast::<ast::Expr>()?;
    if !expr.hashtag() {
        return None;
    }
    highlight(&next.leftmost_leaf()?)
}

/// Whether the node is one of the two identifier nodes.
fn is_ident(node: &LinkedNode) -> bool {
    matches!(node.kind(), SyntaxKind::Ident | SyntaxKind::MathIdent)
}

/// Highlight a node to an HTML `code` element.
///
/// This uses these [CSS classes for categories](Category::css_class).
pub fn highlight_html(root: &SyntaxNode) -> String {
    let mut buf = String::from("<code>");
    let node = LinkedNode::new(root);
    highlight_html_impl(&mut buf, &node);
    buf.push_str("</code>");
    buf
}

/// Highlight one source node, emitting HTML.
fn highlight_html_impl(html: &mut String, node: &LinkedNode) {
    let mut span = false;
    if let Some(category) = highlight(node) {
        if category != Category::Error {
            span = true;
            html.push_str("<span class=\"");
            html.push_str(category.css_class());
            html.push_str("\">");
        }
    }

    let text = node.text();
    if !text.is_empty() {
        for c in text.chars() {
            match c {
                '<' => html.push_str("&lt;"),
                '>' => html.push_str("&gt;"),
                '&' => html.push_str("&amp;"),
                '\'' => html.push_str("&#39;"),
                '"' => html.push_str("&quot;"),
                _ => html.push(c),
            }
        }
    } else {
        for child in node.children() {
            highlight_html_impl(html, &child);
        }
    }

    if span {
        html.push_str("</span>");
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Range;

    use super::*;
    use crate::syntax::Source;

    #[test]
    fn test_highlighting() {
        use Category::*;

        #[track_caller]
        fn test(text: &str, goal: &[(Range<usize>, Category)]) {
            let mut vec = vec![];
            let source = Source::detached(text);
            highlight_tree(&mut vec, &LinkedNode::new(source.root()));
            assert_eq!(vec, goal);
        }

        fn highlight_tree(tags: &mut Vec<(Range<usize>, Category)>, node: &LinkedNode) {
            if let Some(tag) = highlight(node) {
                tags.push((node.range(), tag));
            }

            for child in node.children() {
                highlight_tree(tags, &child);
            }
        }

        test("= *AB*", &[(0..6, Heading), (2..6, Strong)]);

        test(
            "#f(x + 1)",
            &[
                (0..1, Function),
                (1..2, Function),
                (2..3, Punctuation),
                (5..6, Operator),
                (7..8, Number),
                (8..9, Punctuation),
            ],
        );

        test(
            "#let f(x) = x",
            &[
                (0..1, Keyword),
                (1..4, Keyword),
                (5..6, Function),
                (6..7, Punctuation),
                (8..9, Punctuation),
                (10..11, Operator),
            ],
        );
    }
}
