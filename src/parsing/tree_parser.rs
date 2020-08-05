use crate::elements::accent::{Color, ExpressionAccent, GenericAccent, OverSet, UnderSet};
use crate::elements::group::{
    Abs, Angles, Braces, Brackets, Ceil, Floor, Group, Matrix, Norm, Parentheses, XGroup,
};
use crate::elements::literal::{Literal, Number, PlainText, Symbol};
use crate::elements::special::{
    Expression, Frac, Integral, OIntegral, Pow, Prod, Root, Special, Sqrt, Sub, Sum,
};
use crate::elements::Element;
use crate::tokens::{Accent, FontCommand, Grouping, Misc, Operation, Text, Token};
use crate::utils::Boxed;

pub struct TreeParser {
    tokens: Vec<Token>,
    index: usize,
    group_return: bool,
}

impl TreeParser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            index: 0,
            group_return: false,
        }
    }

    pub fn parse(&mut self) -> Expression {
        self.remove_whitespace();
        self.parse_expression()
    }

    fn remove_whitespace(&mut self) {
        self.tokens = self
            .tokens
            .iter()
            .filter_map(|t| {
                if let Token::Text(Text::Whitespace) = t {
                    None
                } else {
                    Some(t.clone())
                }
            })
            .collect();
        // add null end token to ensure that everything got parsed
        self.tokens.push(Token::End)
    }

    fn step(&mut self) -> bool {
        if self.index < (self.tokens.len() - 1) {
            self.index += 1;

            true
        } else {
            false
        }
    }

    fn end_reached(&mut self) -> bool {
        self.index >= (self.tokens.len() - 1)
    }

    fn peek(&mut self) -> Option<&Token> {
        self.tokens.get(self.index + 1)
    }

    fn current_token(&self) -> &Token {
        self.tokens.get(self.index).unwrap()
    }

    fn parse_expression(&mut self) -> Expression {
        let mut expression = Expression::new();

        while !self.end_reached() {
            if let Some(element) = self.parse_element() {
                expression.add_child(element);
            }
            if self.group_return {
                break;
            }
            self.step();
        }
        self.group_return = false;

        expression
    }

    fn parse_element(&mut self) -> Option<Element> {
        let token = self.current_token().clone();
        match token {
            Token::Arrow(a) => Some(Element::Literal(Literal::Arrow(a.clone()))),
            Token::Logical(l) => Some(Element::Literal(Literal::Logical(l.clone()))),
            Token::Relation(r) => Some(Element::Literal(Literal::Relation(r.clone()))),
            Token::Greek(g) => Some(Element::Literal(Literal::Greek(g.clone()))),
            Token::Text(t) => {
                if let Some(literal) = self.parse_text(t) {
                    Some(Element::Literal(literal))
                } else {
                    None
                }
            }
            Token::Operation(op) => Some(self.parse_operation(op)),
            Token::Misc(m) => Some(self.parse_misc(m)),
            Token::Grouping(g) => {
                if let Some(group) = self.parse_matrix() {
                    Some(Element::Group(group))
                } else if let Some(group) = self.parse_group(g) {
                    Some(Element::Group(group))
                } else {
                    None
                }
            }
            Token::Font(f) => {
                if let Some(literal) = self.parse_formatted_text(f) {
                    Some(Element::Literal(literal))
                } else {
                    None
                }
            }
            Token::Accent(a) => {
                if let Some(accent) = self.parse_accent(a) {
                    Some(Element::Accent(accent))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_accent(&mut self, token: Accent) -> Option<ExpressionAccent> {
        match token {
            Accent::OverSet => {
                self.step();
                let top = self.parse_element()?.boxed();
                self.step();
                let bottom = self.parse_element()?.boxed();
                Some(ExpressionAccent::OverSet(OverSet { top, bottom }))
            }
            Accent::UnderSet => {
                self.step();
                let bottom = self.parse_element()?.boxed();
                self.step();
                let top = self.parse_element()?.boxed();
                Some(ExpressionAccent::UnderSet(UnderSet { top, bottom }))
            }
            Accent::Color(color) => {
                self.step();
                let inner = self.parse_element()?.boxed();
                Some(ExpressionAccent::Color(Color { color, inner }))
            }
            _ => {
                self.step();
                Some(ExpressionAccent::Generic(GenericAccent {
                    inner: self.parse_element()?.boxed(),
                    accent: token,
                }))
            }
        }
    }

    fn parse_formatted_text(&mut self, token: FontCommand) -> Option<Literal> {
        let next_token = if let Some(token) = self.peek() {
            Some(token.clone())
        } else {
            None
        };
        if let Some(Token::Text(Text::Plain(p))) = next_token {
            self.step();
            Some(Literal::Text(PlainText {
                text: p,
                formatting: Some(token),
            }))
        } else {
            Some(Literal::Symbol(Symbol {
                symbol: token.to_string(),
            }))
        }
    }

    fn parse_text(&self, token: Text) -> Option<Literal> {
        match token {
            Text::Symbol(s) => Some(Literal::Symbol(Symbol { symbol: s })),
            Text::Number(n) => Some(Literal::Number(Number { number: n })),
            Text::Plain(p) => Some(Literal::Text(PlainText {
                text: p,
                formatting: None,
            })),
            _ => None,
        }
    }

    fn parse_operation(&mut self, token: Operation) -> Element {
        match token {
            Operation::Sum => {
                let mut sum = Sum::new();
                sum.bottom = self.parse_sub();
                sum.top = self.parse_pow();

                Element::Special(Special::Sum(sum))
            }
            Operation::Prod => {
                let mut prod = Prod::new();
                prod.bottom = self.parse_sub();
                prod.top = self.parse_pow();

                Element::Special(Special::Prod(prod))
            }
            _ => Element::Literal(Literal::Operation(token)),
        }
    }

    fn parse_misc(&mut self, token: Misc) -> Element {
        match token {
            Misc::Pow => {
                self.step();
                Element::Special(Special::Pow(Pow {
                    exp: self.parse_element().unwrap().boxed(),
                }))
            }
            Misc::Sub => {
                self.step();
                Element::Special(Special::Sub(Sub {
                    lower: self.parse_element().unwrap().boxed(),
                }))
            }
            Misc::LatexFrac => {
                self.step();
                Element::Special(Special::Frac(Frac {
                    top: self.parse_element().unwrap().boxed(),
                    bottom: self.parse_element().unwrap().boxed(),
                }))
            }
            Misc::Sqrt => {
                self.step();
                Element::Special(Special::Sqrt(Sqrt {
                    inner: self.parse_element().unwrap().boxed(),
                }))
            }
            Misc::Root => {
                self.step();
                let base = self.parse_element().unwrap().boxed();
                self.step();
                let inner = self.parse_element().unwrap().boxed();
                Element::Special(Special::Root(Root { inner, base }))
            }
            Misc::Int => Element::Special(Special::Integral(Integral {
                bottom: self.parse_sub(),
                top: self.parse_pow(),
            })),
            Misc::OInt => Element::Special(Special::OIntegral(OIntegral {
                bottom: self.parse_sub(),
                top: self.parse_pow(),
            })),
            _ => Element::Literal(Literal::Misc(token)),
        }
    }

    fn parse_matrix(&mut self) -> Option<Group> {
        let token = self.current_token().clone();
        let start_index = self.index;
        if let Token::Grouping(Grouping::RBracket) = token {
            let mut expressions = Vec::new();

            while !self.end_reached() {
                if let Some(Token::Grouping(Grouping::RBracket)) = self.peek() {
                    self.step();
                    self.step();
                    expressions.push(self.parse_expression());
                    self.step();

                    if let Token::Grouping(Grouping::LBracket) = self.current_token() {
                        self.step();
                    }
                    if let Token::Grouping(Grouping::LBracket) = self.current_token() {
                        break;
                    }
                } else {
                    break;
                }
            }
            // Remapping the expression into a matrix
            let expression_matrix = expressions
                .iter()
                .map(|e| {
                    let children = e.children.clone();
                    let mut expressions = Vec::new();

                    for elements in children.split(|e| e == &Element::Group(Group::MSep)) {
                        expressions.push(Expression {
                            children: elements.to_vec(),
                        })
                    }
                    expressions
                })
                .collect::<Vec<Vec<Expression>>>();

            // a matrix with no elements is invalid
            if expression_matrix.len() == 0 {
                self.index = start_index;
                return None;
            }
            /// a matrix with rows of different lengths is invalid
            let first_length = expression_matrix.first().unwrap().len();
            if !expression_matrix.iter().all(|e| e.len() == first_length) {
                self.index = start_index;
                return None;
            }

            Some(Group::Matrix(Matrix {
                inner: expression_matrix,
            }))
        } else {
            None
        }
    }

    fn parse_group(&mut self, token: Grouping) -> Option<Group> {
        match token {
            Grouping::RParen => {
                self.step();
                let inner = self.parse_expression().boxed();
                Some(Group::Parentheses(Parentheses { inner }))
            }
            Grouping::RBrace => {
                self.step();
                let inner = self.parse_expression().boxed();

                Some(Group::Braces(Braces { inner }))
            }
            Grouping::RBracket => {
                self.step();
                let inner = self.parse_expression().boxed();

                Some(Group::Brackets(Brackets { inner }))
            }
            Grouping::RAngle => {
                self.step();
                let inner = self.parse_expression().boxed();

                Some(Group::Angles(Angles { inner }))
            }
            Grouping::RXPar => {
                self.step();
                let inner = self.parse_expression().boxed();

                Some(Group::XGroup(XGroup { inner }))
            }
            Grouping::Abs => {
                self.step();
                self.step();
                let inner = self.parse_expression().boxed();
                Some(Group::Abs(Abs { inner }))
            }
            Grouping::Floor => {
                self.step();
                self.step();
                let inner = self.parse_expression().boxed();
                Some(Group::Floor(Floor { inner }))
            }
            Grouping::Ceil => {
                self.step();
                self.step();
                let inner = self.parse_expression().boxed();
                Some(Group::Ceil(Ceil { inner }))
            }
            Grouping::Norm => {
                self.step();
                self.step();
                let inner = self.parse_expression().boxed();
                Some(Group::Norm(Norm { inner }))
            }
            Grouping::MSep => Some(Group::MSep),
            _ => {
                self.group_return = true;
                None
            }
        }
    }

    fn parse_sub(&mut self) -> Option<Box<Element>> {
        if let Some(Token::Misc(Misc::Sub)) = self.peek() {
            self.step();
            self.step();
            if let Some(element) = self.parse_element() {
                Some(element.boxed())
            } else {
                None
            }
        } else {
            None
        }
    }

    fn parse_pow(&mut self) -> Option<Box<Element>> {
        if let Some(Token::Misc(Misc::Pow)) = self.peek() {
            self.step();
            self.step();
            if let Some(element) = self.parse_element() {
                Some(element.boxed())
            } else {
                None
            }
        } else {
            None
        }
    }
}
