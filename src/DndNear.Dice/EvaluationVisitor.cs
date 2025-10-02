using Antlr4.Runtime.Tree;
using Antlr4.Runtime.Misc;

namespace DndNear.Dice
{
    public class EvaluationVisitor(Random random) : DiceExpressionBaseVisitor<int>
    {
        #region Consts & Static
        #endregion Consts & Static

        #region Fields & Properties

        private Random Random { get; init; } = random;

        #endregion Fields & Properties

        #region Constructors
        #endregion Constructors

        #region Methods

        public override int VisitTerm([NotNull] DiceExpressionParser.TermContext context)
        {
            return int.Parse(context.NUMBER().GetText());
        }

        public override int VisitDice([NotNull] DiceExpressionParser.DiceContext context)
        {
            // Get number of dice (default to 1 if not specified)
            int dieCount = context.term(0) != null ? Visit(context.term(0)) : 1;

            // Get number of sides (always present)
            int dieSides = Visit(context.term(1));

            int[] rolls = new int[dieCount];
            int sum = 0;
            for (int i = 0; i < dieCount; ++i)
            {
                int roll = Random.Next(1, dieSides + 1);
                rolls[i] = roll;
                sum += roll;
            }

            return sum;
        }

        public override int VisitFactor([NotNull] DiceExpressionParser.FactorContext context)
        {
            // Visit first child
            int result = Visit(context.GetChild(0));

            // Process operators and operands in pairs
            for (int i = 1; i < context.ChildCount; i += 2)
            {
                var op = context.GetChild(i) as ITerminalNode
                    ?? throw new InvalidOperationException("Expected operator token");
                int nextOperand = Visit(context.GetChild(i + 1));

                result = op.Symbol.Type switch
                {
                    DiceExpressionLexer.MULT => result * nextOperand,
                    DiceExpressionLexer.DIV => result / nextOperand,
                    _ => throw new InvalidOperationException($"Unexpected operator: {op.GetText()}")
                };
            }

            return result;
        }

        public override int VisitExpression([NotNull] DiceExpressionParser.ExpressionContext context)
        {
            // Visit first child
            int result = Visit(context.GetChild(0));

            // Process operators and operands in pairs
            for (int i = 1; i < context.ChildCount; i += 2)
            {
                var op = context.GetChild(i) as ITerminalNode
                    ?? throw new InvalidOperationException("Expected operator token");
                int nextOperand = Visit(context.GetChild(i + 1));

                result = op.Symbol.Type switch
                {
                    DiceExpressionLexer.PLUS => result + nextOperand,
                    DiceExpressionLexer.MINUS => result - nextOperand,
                    _ => throw new InvalidOperationException($"Unexpected operator: {op.GetText()}")
                };
            }

            return result;
        }

        #endregion Methods
    }
}
