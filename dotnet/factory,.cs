public interface IProblem
{
    public void Execute()
    {
        Console.WriteLine("******************************************************************************************************************");
        Console.WriteLine($"");
        Console.WriteLine(Name);
        Console.WriteLine(Description);
        Console.WriteLine(Solution);
        Console.WriteLine("");
    }
    string Description { get; }
    string Name { get; }
    string Solution { get; }
}

internal sealed class ProblemFactory
{
    public IProblem CreateProblemAdapter(int problem)
    {
        return problem switch
        {
            1 => new Problem1(),
            2 => new Problem2(),
            _ => new Problem0()
        };
    }
}