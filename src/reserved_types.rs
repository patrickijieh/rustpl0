use crate::token::TokenType::{self, *};

const reserved_words: [&str; 15] = [
  "const", "var", "procedure",
  "call", "begin", "end",
	"if", "then", "else", "while", "do",
  "read", "write", "skip", "odd",
];

const reserved_types: [TokenType; 15] = [
  Constsym, Varsym, Procsym,
  Callsym, Beginsym, Endsym,
  Ifsym, Thensym, Elsesym, Whilesym, Dosym,
  Readsym, Writesym, Skipsym, Oddsym,
];

pub fn get_reserved_word(word: &str) -> TokenType {
  for i in 0..reserved_words.len() {
    if word == reserved_words[i] {
      return reserved_types[i];
    }
  }
  Identsym
}