namespace DndNear.Dice.Tests;

public class UnitTest1
{
    [Fact]
    public void Test1()
    {
        var test = new Random(0);

        var random = new Random(0);
        var roller = new Roller(random);

        for (int i = 0; i < 1000; ++i)
        {
            int correct = test.Next(1, 7) + test.Next(1, 7) + 3;
            int result = roller.Roll("2d6+3");
            Assert.Equal(result, correct);
        }
    }
}