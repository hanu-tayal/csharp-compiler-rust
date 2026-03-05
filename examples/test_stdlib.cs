using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

namespace TestStdLib
{
    class Program
    {
        static void Main(string[] args)
        {
            // Test basic Console operations
            Console.WriteLine("Hello from C# compiler written in Rust!");
            Console.WriteLine("Testing standard library bindings...");
            
            // Test string operations
            string message = "Hello, World!";
            Console.WriteLine("String length: " + message.Length);
            string substring = message.Substring(0, 5);
            Console.WriteLine("Substring: " + substring);
            
            // Test DateTime
            DateTime now = DateTime.Now;
            Console.WriteLine("Current date: " + now.ToString("yyyy-MM-dd"));
            
            // Test Math operations
            double result = Math.Sqrt(16);
            Console.WriteLine("Square root of 16: " + result);
            Console.WriteLine("PI value: " + Math.PI);
            
            // Test Convert operations
            int number = Convert.ToInt32("42");
            Console.WriteLine("Converted number: " + number);
            
            // Test List<T>
            List<string> names = new List<string>();
            names.Add("Alice");
            names.Add("Bob");
            names.Add("Charlie");
            Console.WriteLine("List count: " + names.Count);
            
            // Test LINQ
            var filtered = names.Where(n => n.Length > 3).ToList();
            Console.WriteLine("Names with more than 3 chars: " + filtered.Count);
            
            // Test StringBuilder
            StringBuilder sb = new StringBuilder();
            sb.Append("Building ");
            sb.Append("a ");
            sb.AppendLine("string!");
            Console.WriteLine(sb.ToString());
            
            // Test File operations
            string path = "test.txt";
            if (File.Exists(path))
            {
                string content = File.ReadAllText(path);
                Console.WriteLine("File content: " + content);
            }
            else
            {
                Console.WriteLine("File does not exist");
            }
            
            // Test Environment
            Console.WriteLine("Current directory: " + Environment.CurrentDirectory);
            Console.WriteLine("New line character count: " + Environment.NewLine.Length);
            
            // Test Array operations
            int[] numbers = new int[] { 1, 2, 3, 4, 5 };
            Console.WriteLine("Array length: " + numbers.Length);
            Array.Sort(numbers);
            
            // Test Exception handling
            try
            {
                throw new InvalidOperationException("Test exception");
            }
            catch (Exception ex)
            {
                Console.WriteLine("Caught exception: " + ex.Message);
            }
            
            Console.WriteLine("All tests completed!");
        }
    }
}