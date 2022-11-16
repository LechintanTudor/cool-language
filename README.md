# Formal Languages and Compiler Design

Lab assignments for the formal languages and compiler design class.

## Scripts

- Use `./contrib/gendoc.sh` to generate the compiler documentation.
- Use `./contrib/opendoc.sh` to open the compiler documentation.

## State Machines

- Numbers:

```
    <non_zero_digit> ::= 1 | 2 | ... | 9

    <digit> ::= 0 | <non_zero_digit>
    
    <digit_seq> ::= <digit>
                  | <digit><digit_seq>

    <number> ::= <digit>
               | <non_zero_digit><digit_seq>
```

- Identifier:

```
    <letter> ::= a | b | ... | z | A | B | ... | Z

    <digit> ::= 0 | 1 | ... | 9

    <ident_start> ::= _ | <letter>

    <ident_char> ::= _ | <letter> | <digit>

    <ident_char_seq> ::= <ident_char>
                       | <ident_char><ident_char_seq>

    <ident> ::= <letter>
              | <ident_start><ident_char_seq>
```