using System;

using Antlr4.Runtime;

namespace DndNear.Dice
{
    public class Roller(Random? random = null)
    {
        #region Consts & Static
        #endregion Consts & Static

        #region Fields & Properties

        private Random Random { get; init; } = random ?? new Random();

        #endregion Fields & Properties

        #region Constructors
        #endregion Constructors

        #region Methods

        public int Roll(string expression)
        {
            var inputStream = new AntlrInputStream(expression);
            var lexer = new DiceExpressionLexer(inputStream);
            var tokenStream = new CommonTokenStream(lexer);
            var parser = new DiceExpressionParser(tokenStream);

            var tree = parser.expression();

            var visitor = new EvaluationVisitor(Random);
            return visitor.Visit(tree);
        }

        #endregion Methods
    }
}
