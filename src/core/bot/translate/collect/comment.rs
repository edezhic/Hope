use std::iter::Peekable;
use unicode_segmentation::UWordBounds;

use crate::core::*;

impl Bot {
    pub fn collect_comment(
        &self,
        pieces: &mut Peekable<UWordBounds<'_>>,
        tokens: &mut Vec<Token>,
    ) -> Result<()> {
        let mut comment = Text::empty();
        while let Some(piece) = pieces.next() {
            if self.vocab.comment_end(piece) {
                break;
            } else {
                comment.add(piece);
            }
        }
        tokens.push(Token::Comment(comment));
        Ok(())
    }
}