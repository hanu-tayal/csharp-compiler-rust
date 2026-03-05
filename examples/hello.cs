using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            // This is a comment
            Console.WriteLine("Hello, World!");
            
            int x = 42;
            var y = x + 10;
            
            if (y > 50)
            {
                Console.WriteLine($"y = {y}");
            }
        }
    }
}