mod english;
use crate::core::*;
use regex::Regex as R;

pub enum Pattern {
    None,
    Comment,
    Id,
    Seal,
    Text,
    Time,
    Version,
}

pub struct Vocabulary {
    ignore: R,
    whitespace: R,
    next: R,
    new: R,
    this: R,
    term: R,
    comment_start: R,
    comment_end: R,

    val_fact_false: R,
    val_fact_true: R,
    val_id: R,
    val_number: R,
    val_seal: R,
    val_text: R,
    val_time: R,
    val_version: R,

    mod_binding: R,
    mod_selection: R,
    mod_targeting: R,

    case_and: R,
    case_equal: R,
    case_if: R,
    case_then: R,

    cmd_include: R,
    cmd_send: R,
    cmd_show: R,
    cmd_sum: R,

    exp_assign: R,
    exp_divide: R,
    exp_end: R,
    exp_start: R,
    exp_minus: R,
    exp_multiply: R,
    exp_plus: R,
}

impl Vocabulary {
    pub fn check_pattern(&self, piece: &str) -> Option<Pattern> {
        match piece {
            piece if self.comment_start.is_match(piece) => Some(Pattern::Comment),
            piece if self.val_id.is_match(piece) => Some(Pattern::Id),
            piece if self.val_seal.is_match(piece) => Some(Pattern::Seal),
            piece if self.val_text.is_match(piece) => Some(Pattern::Text),
            piece if self.val_time.is_match(piece) => Some(Pattern::Time),
            piece if self.val_version.is_match(piece) => Some(Pattern::Version),
            _ => None,
        }
    }

    pub fn skip(&self, piece: &str) -> bool {
        match piece {
            piece if self.whitespace.is_match(piece) => true,
            piece if self.ignore.is_match(piece) => true,
            _ => false,
        }
    }

    pub fn whitespace(&self, piece: &str) -> bool {
        if self.whitespace.is_match(piece) {
            return true;
        }
        false
    }

    pub fn literal_end(&self, piece: &str) -> Option<Token> {
        match piece {
            piece if self.new.is_match(piece) => Some(Token::Mod(Modifier::New)),
            piece if self.next.is_match(piece) => Some(Token::Mod(Modifier::Next)),
            _ => None,
        }
    }

    pub fn number(&self, piece: &str) -> Option<Number> {
        if self.val_number.is_match(piece) {
            let number = Number::from_string(piece.replacen(",", ".", 1)).unwrap();
            Some(number)
        } else {
            None
        }
    }

    pub fn reserved(&self, piece: &str) -> Option<Token> {
        match piece {
            piece if self.next.is_match(piece) => Some(Token::Mod(Modifier::Next)),
            piece if self.new.is_match(piece) => Some(Token::Mod(Modifier::New)),
            piece if self.this.is_match(piece) => Some(Token::This),
            piece if self.case_and.is_match(piece) => Some(Token::Case(Case::And)),
            piece if self.case_equal.is_match(piece) => Some(Token::Case(Case::Equal)),
            piece if self.case_if.is_match(piece) => Some(Token::Case(Case::If)),
            piece if self.case_then.is_match(piece) => Some(Token::Case(Case::Then)),
            piece if self.cmd_send.is_match(piece) => Some(Token::Cmd(Command::Send)),
            piece if self.cmd_show.is_match(piece) => Some(Token::Cmd(Command::Show)),
            piece if self.cmd_sum.is_match(piece) => Some(Token::Cmd(Command::Sum)),
            piece if self.exp_assign.is_match(piece) => Some(Token::Exp(Expression::Assign)),
            piece if self.exp_divide.is_match(piece) => Some(Token::Exp(Expression::Divide)),
            piece if self.exp_end.is_match(piece) => Some(Token::Exp(Expression::End)),
            piece if self.exp_minus.is_match(piece) => Some(Token::Exp(Expression::Minus)),
            piece if self.exp_multiply.is_match(piece) => Some(Token::Exp(Expression::Multiply)),
            piece if self.exp_plus.is_match(piece) => Some(Token::Exp(Expression::Plus)),
            piece if self.exp_start.is_match(piece) => Some(Token::Exp(Expression::Start)),
            piece if self.mod_binding.is_match(piece) => Some(Token::Mod(Modifier::Binding)),
            piece if self.mod_selection.is_match(piece) => Some(Token::Mod(Modifier::Selection)),
            piece if self.mod_targeting.is_match(piece) => Some(Token::Mod(Modifier::Targeting)),
            piece if self.val_fact_true.is_match(piece) => {
                Some(Token::Val(Value::Fact(Fact::truth())))
            }
            piece if self.val_fact_false.is_match(piece) => {
                Some(Token::Val(Value::Fact(Fact::falsehood())))
            }
            _ => None,
        }
    }

    pub fn term(&self, piece: &str) -> Option<Token> {
        if self.term.is_match(piece) {
            Some(Token::Term(Text::from_str(piece)))
        } else {
            None
        }
    }

    pub fn text_end(&self, piece: &str) -> bool {
        if self.val_text.is_match(piece) {
            return true;
        }
        false
    }

    pub fn comment_end(&self, piece: &str) -> bool {
        if self.comment_end.is_match(piece) {
            return true;
        }
        false
    }
}
