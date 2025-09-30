grammar DiceExpression;

// Lexer Rules (UPPERCASE - the tokens)
NUMBER : [0-9]+ ;           // One or more digits

PLUS   : '+' ;
MINUS  : '-' ;
MULT   : '*' ;
DIV    : '/' ;

DICE   : 'd' ;              // The 'd' in dice notation

WS : [ \t\r\n]+ -> skip ;   // Ignore whitespace
