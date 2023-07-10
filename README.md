# rustpl0
PL/0 language compiler written in the Rust language.</br></br>
Credits to my college professor (Dr. Gary T. Leavens)  for creating a similar compiler written in C. ([Link](http://www.cs.ucf.edu/~leavens/COP3402/example-code/index.html))</br></br>

## Context-free Grammar

Here is the context-free grammar for the full PL/0 language:

```
⟨program⟩ ::= ⟨block⟩ .
 
⟨block⟩ ::= ⟨const-decls⟩ ⟨var-decls⟩ ⟨proc-decls⟩ ⟨stmt⟩

⟨const-decls⟩ ::= {⟨const-decl⟩}
⟨const-decl⟩ ::= const ⟨const-def⟩ {⟨comma-const-def⟩} ;
⟨const-def⟩ ::= ⟨ident⟩ = ⟨number⟩
⟨comma-const-def⟩ ::= , ⟨const-def⟩

⟨var-decls⟩ ::= {⟨var-decl⟩}
⟨var-decl⟩ ::= var ⟨idents⟩ ;
⟨idents⟩ ::= ⟨ident⟩ {⟨comma-ident⟩}
⟨comma-ident⟩ ::= , ⟨ident⟩

⟨proc-decls⟩ ::= {⟨proc-decl⟩}
⟨proc-decl⟩ ::= procedure ⟨ident⟩ ; ⟨block⟩ ;

⟨stmt⟩ ::= ⟨ident⟩ := ⟨expr⟩
  | call ⟨ident⟩
  | begin ⟨stmt⟩ {⟨semi-stmt⟩} end
  | if ⟨condition⟩ then ⟨stmt⟩ else ⟨stmt⟩
  | while ⟨condition⟩ do ⟨stmt⟩
  | read ⟨ident⟩
  | write ⟨expr⟩
  | skip
⟨semi-stmt⟩ ::= ; ⟨stmt⟩
⟨empty⟩ ::=

⟨condition⟩ ::= odd ⟨expr⟩
  | ⟨expr⟩ ⟨rel-op⟩ ⟨expr⟩
⟨rel-op⟩ ::= = | <> | < | <= | > | >=

⟨expr⟩ ::= ⟨term⟩ {⟨add-sub-term⟩}
⟨add-sub-term⟩ ::= ⟨add-sub⟩ ⟨term⟩
⟨add-sub⟩ ::= ⟨plus⟩ | ⟨minus⟩
⟨term⟩ ::= ⟨factor⟩ {⟨mult-div-factor⟩}
⟨mult-div-factor⟩ ::= ⟨mult-div⟩ ⟨factor⟩
⟨mult-div⟩ ::= ⟨mult⟩ | ⟨div⟩
⟨factor⟩ ::= ⟨ident⟩ | ⟨sign⟩ ⟨number⟩ | ( ⟨expr⟩ )
⟨sign⟩ ::= ⟨plus⟩ | ⟨minus⟩ | ⟨empty⟩
```
Credit to Dr. Gary T. Leavens for the context free grammar: [Link](http://www.cs.ucf.edu/~leavens/COP3402/homeworks/hw4-pl0-codegen.pdf)