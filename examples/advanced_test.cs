using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Linq;

namespace AdvancedTest
{
    // Test delegate types
    public delegate void MyDelegate(string message);
    public delegate int Calculator(int a, int b);
    
    // Test enum
    public enum DayOfWeek
    {
        Monday = 1,
        Tuesday = 2,
        Wednesday = 3,
        Thursday = 4,
        Friday = 5,
        Saturday = 6,
        Sunday = 7
    }
    
    // Test struct
    public struct Point
    {
        public double X { get; set; }
        public double Y { get; set; }
        
        public Point(double x, double y)
        {
            X = x;
            Y = y;
        }
        
        public double Distance()
        {
            return Math.Sqrt(X * X + Y * Y);
        }
    }
    
    // Test interface
    public interface IShape
    {
        double Area();
        double Perimeter();
    }
    
    // Test class implementing interface
    public class Circle : IShape
    {
        public double Radius { get; set; }
        
        public Circle(double radius)
        {
            Radius = radius;
        }
        
        public double Area()
        {
            return Math.PI * Radius * Radius;
        }
        
        public double Perimeter()
        {
            return 2 * Math.PI * Radius;
        }
    }
    
    // Test generic class
    public class Box<T>
    {
        private T content;
        
        public void Put(T item)
        {
            content = item;
        }
        
        public T Get()
        {
            return content;
        }
    }
    
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Advanced C# Features Test");
            Console.WriteLine("========================");
            
            // Test enum
            DayOfWeek today = DayOfWeek.Friday;
            Console.WriteLine("Today is: " + today);
            Console.WriteLine("Day number: " + (int)today);
            
            // Test struct
            Point p1 = new Point(3, 4);
            Console.WriteLine($"Point distance from origin: {p1.Distance()}");
            
            // Test interface and polymorphism
            IShape shape = new Circle(5);
            Console.WriteLine($"Circle area: {shape.Area():F2}");
            Console.WriteLine($"Circle perimeter: {shape.Perimeter():F2}");
            
            // Test delegates
            MyDelegate del = PrintMessage;
            del += PrintUpperMessage;
            del("Hello delegates!");
            
            Calculator calc = Add;
            Console.WriteLine($"5 + 3 = {calc(5, 3)}");
            
            calc = Multiply;
            Console.WriteLine($"5 * 3 = {calc(5, 3)}");
            
            // Test generic class
            Box<int> intBox = new Box<int>();
            intBox.Put(42);
            Console.WriteLine($"Box contains: {intBox.Get()}");
            
            Box<string> stringBox = new Box<string>();
            stringBox.Put("Hello generics!");
            Console.WriteLine($"Box contains: {stringBox.Get()}");
            
            // Test LINQ with complex queries
            List<int> numbers = new List<int> { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };
            
            var evenSquares = numbers
                .Where(n => n % 2 == 0)
                .Select(n => n * n)
                .ToList();
            
            Console.WriteLine("Even squares: " + string.Join(", ", evenSquares));
            
            // Test Dictionary
            Dictionary<string, int> scores = new Dictionary<string, int>();
            scores.Add("Alice", 95);
            scores.Add("Bob", 87);
            scores.Add("Charlie", 92);
            
            if (scores.TryGetValue("Alice", out int aliceScore))
            {
                Console.WriteLine($"Alice's score: {aliceScore}");
            }
            
            // Test TimeSpan
            TimeSpan duration = TimeSpan.FromDays(1.5);
            Console.WriteLine($"Duration: {duration.TotalHours} hours");
            
            // Test Guid
            Guid id = Guid.NewGuid();
            Console.WriteLine($"Generated GUID: {id}");
            
            // Test async/await (simplified)
            Console.WriteLine("Starting async operation...");
            Task.Delay(100).Wait();
            Console.WriteLine("Async operation completed!");
            
            // Test string formatting
            DateTime now = DateTime.Now;
            string formatted = $"Today is {now:dddd, MMMM d, yyyy} at {now:h:mm tt}";
            Console.WriteLine(formatted);
            
            // Test array operations
            int[] array = { 5, 2, 8, 1, 9 };
            Array.Sort(array);
            Console.WriteLine("Sorted array: " + string.Join(", ", array));
            
            // Test Convert methods
            string base64 = Convert.ToBase64String(new byte[] { 1, 2, 3, 4, 5 });
            Console.WriteLine($"Base64: {base64}");
            
            byte[] decoded = Convert.FromBase64String(base64);
            Console.WriteLine($"Decoded length: {decoded.Length}");
            
            Console.WriteLine("\nAll advanced tests completed!");
        }
        
        static void PrintMessage(string msg)
        {
            Console.WriteLine($"Message: {msg}");
        }
        
        static void PrintUpperMessage(string msg)
        {
            Console.WriteLine($"UPPER: {msg.ToUpper()}");
        }
        
        static int Add(int a, int b)
        {
            return a + b;
        }
        
        static int Multiply(int a, int b)
        {
            return a * b;
        }
    }
}