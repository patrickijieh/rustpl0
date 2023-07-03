use crate::token::TokenType::{self, *};

const RESERVED_WORDS: [&str; 15] = [
  "const", "var", "procedure",
  "call", "begin", "end",
	"if", "then", "else", "while", "do",
  "read", "write", "skip", "odd",
];

const RESERVED_TYPES: [TokenType; 15] = [
  Constsym, Varsym, Procsym,
  Callsym, Beginsym, Endsym,
  Ifsym, Thensym, Elsesym, Whilesym, Dosym,
  Readsym, Writesym, Skipsym, Oddsym,
];

pub fn get_reserved_word(word: &str) -> TokenType {
  for i in 0..RESERVED_WORDS.len() {
    if word == RESERVED_WORDS[i] {
      return RESERVED_TYPES[i];
    }
  }
  Identsym
}