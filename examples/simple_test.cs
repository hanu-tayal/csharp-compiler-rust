using System;

namespace SimpleTest
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello from the Rust C# Compiler!");
            
            int x = 10;
            int y = 20;
            int sum = x + y;
            
            Console.WriteLine("The sum of " + x + " and " + y + " is " + sum);
            
            for (int i = 0; i < 5; i++)
            {
                Console.WriteLine("Loop iteration: " + i);
            }
            
            Console.WriteLine("Test completed!");
        }
    }
}