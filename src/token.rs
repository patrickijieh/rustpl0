const NUM_TOKENS: usize = 34;

const TYPE_TO_STR: [&str; NUM_TOKENS] = [
  "periodsym", "constsym", "semisym", "commasym",
  "varsym", "procsym", "becomessym", "callsym", "beginsym", "endsym",
  "ifsym", "thensym", "elsesym", "whilesym", "dosym",
  "readsym", "writesym", "skipsym",
  "oddsym", "lparensym", "rparensym",
  "identsym", "numbersym",
  "eqsym", "neqsym", "lessym", "leqsym", "gtrsym", "geqsym",
  "plussym", "minussym", "multsym", "divsym",
  "eofsym"
];

#[derive(Debug, PartialEq)] 
pub enum TokenType {
  Periodsym, Constsym, Semisym, Commasym,
  Varsym, Procsym, Becomessym, Callsym, Beginsym, Endsym,
  Ifsym, Thensym, Elsesym, Whilesym, Dosym, 
  Readsym, Writesym, Skipsym,
  Oddsym, Lparensym, Rparensym,
  Identsym, Numbersym, 
  Eqsym, Neqsym, Lessym, Leqsym, Gtrsym, Geqsym, 
  Plussym, Minussym, Multsym, Divsym, 
  Eofsym
}

impl Copy for TokenType { }

impl Clone for TokenType {
  fn clone(&self) -> Self { *self }
}

pub struct Token {
  pub typ: TokenType,
  pub filename: String,
  pub line: i32,
  pub column: i32,
  pub text: String,
  pub value: i32
}

impl Token {
  pub fn new() -> Self {
    Token {
      typ: TokenType::Eofsym,
      filename: String::new(),
      line: 0,
      column: 0,
      text: String::new(),
      value: 0
    }
  }

  pub fn with_file(file_name: String) -> Self {
    Token {
      typ: TokenType::Eofsym,
      filename: file_name,
      line: 0,
      column: 0,
      text: String::new(),
      value: 0
    }
  }
  
  pub fn ttyp2str(&self) -> String {
    TYPE_TO_STR[self.typ as usize].to_string()
  }
}

pub fn type_to_string(ttyp: TokenType) -> String {
  TYPE_TO_STR[ttyp as usize].to_string()
}