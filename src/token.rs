const NUM_TOKENS: usize = 34;

const type_to_str: [&str; NUM_TOKENS] = [
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

enum token_type {
  periodsym, constsym, semisym, commasym,
  varsym, procsym, becomessym, callsym, beginsym, endsym,
  ifsym, thensym, elsesym, whilesym, dosym, 
  readsym, writesym, skipsym,
  oddsym, lparensym, rparensym,
  identsym, numbersym, 
  eqsym, neqsym, lessym, leqsym, gtrsym, geqsym, 
  plussym, minussym, multsym, divsym, 
  eofsym
}

pub struct token {
  token_type: typ,
  filename: String,
  line: i32,
  column: i32,
  text: String,
  value: i32
}

impl token {
  pub fn new() -> Self {
    token {
      token_type: token_type::eofsym,
      filename: String::new(),
      line: 0,
      column: 0,
      text: String::new(),
      value: 0
    }
  }
  
  pub fn ttyp2str(&self, ttyp: token_type) -> String {
    type_to_str[ttyp as usize].to_string()
  }
}