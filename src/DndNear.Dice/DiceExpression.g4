grammar DiceExpression;

// Lexer Rules (UPPERCASE - the tokens)
NUMBER : [0-9]+ ;           // One or more digits

PLUS   : '+' ;
MINUS  : '-' ;
MULT   : '*' ;
DIV    : '/' ;

DIE   : 'd' ;               // The 'd' in dice notation

WS : [ \t\r\n]+ -> skip ;   // Ignore whitespace

expression : factor ((PLUS | MINUS) factor)* ;
factor     : (term | dice) ((MULT | DIV) (term | dice))* ;
dice       : term? DIE term ;
term       : NUMBER ;
