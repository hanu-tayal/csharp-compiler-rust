using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.IO;
using System.Threading.Tasks;

namespace CompilerShowcase
{
    // Enum demonstration
    public enum LogLevel
    {
        Debug = 0,
        Info = 1,
        Warning = 2,
        Error = 3,
        Fatal = 4
    }

    // Interface demonstration
    public interface ILogger
    {
        void Log(LogLevel level, string message);
        void LogException(Exception ex);
    }

    // Generic interface
    public interface IDataStore<T> where T : class
    {
        void Save(T item);
        T Load(string id);
        IEnumerable<T> LoadAll();
    }

    // Struct demonstration
    public struct Point3D
    {
        public double X { get; set; }
        public double Y { get; set; }
        public double Z { get; set; }

        public Point3D(double x, double y, double z)
        {
            X = x;
            Y = y;
            Z = z;
        }

        public double DistanceFromOrigin()
        {
            return Math.Sqrt(X * X + Y * Y + Z * Z);
        }

        public override string ToString()
        {
            return $"({X:F2}, {Y:F2}, {Z:F2})";
        }
    }

    // Base class with virtual members
    public abstract class Shape
    {
        public string Name { get; protected set; }
        public Point3D Center { get; set; }

        protected Shape(string name)
        {
            Name = name;
            Center = new Point3D(0, 0, 0);
        }

        public abstract double CalculateArea();
        public abstract double CalculateVolume();

        public virtual string GetDescription()
        {
            return $"{Name} at {Center}";
        }
    }

    // Derived class
    public class Sphere : Shape
    {
        public double Radius { get; set; }

        public Sphere(double radius) : base("Sphere")
        {
            Radius = radius;
        }

        public override double CalculateArea()
        {
            return 4 * Math.PI * Radius * Radius;
        }

        public override double CalculateVolume()
        {
            return (4.0 / 3.0) * Math.PI * Math.Pow(Radius, 3);
        }

        public override string GetDescription()
        {
            return $"{base.GetDescription()}, Radius: {Radius:F2}";
        }
    }

    // Generic class with constraints
    public class Repository<T> : IDataStore<T> where T : class, new()
    {
        private readonly Dictionary<string, T> storage = new Dictionary<string, T>();
        private readonly ILogger logger;

        public Repository(ILogger logger)
        {
            this.logger = logger;
        }

        public void Save(T item)
        {
            var id = Guid.NewGuid().ToString();
            storage[id] = item;
            logger.Log(LogLevel.Info, $"Saved item with ID: {id}");
        }

        public T Load(string id)
        {
            if (storage.TryGetValue(id, out T item))
            {
                logger.Log(LogLevel.Debug, $"Loaded item with ID: {id}");
                return item;
            }
            logger.Log(LogLevel.Warning, $"Item not found: {id}");
            return null;
        }

        public IEnumerable<T> LoadAll()
        {
            logger.Log(LogLevel.Debug, $"Loading all {storage.Count} items");
            return storage.Values;
        }
    }

    // Console logger implementation
    public class ConsoleLogger : ILogger
    {
        public void Log(LogLevel level, string message)
        {
            var timestamp = DateTime.Now.ToString("yyyy-MM-dd HH:mm:ss");
            var color = GetColorForLevel(level);
            
            Console.ForegroundColor = color;
            Console.WriteLine($"[{timestamp}] [{level}] {message}");
            Console.ResetColor();
        }

        public void LogException(Exception ex)
        {
            Log(LogLevel.Error, $"Exception: {ex.GetType().Name} - {ex.Message}");
            if (ex.StackTrace != null)
            {
                Console.WriteLine(ex.StackTrace);
            }
        }

        private ConsoleColor GetColorForLevel(LogLevel level)
        {
            switch (level)
            {
                case LogLevel.Debug: return ConsoleColor.Gray;
                case LogLevel.Info: return ConsoleColor.White;
                case LogLevel.Warning: return ConsoleColor.Yellow;
                case LogLevel.Error: return ConsoleColor.Red;
                case LogLevel.Fatal: return ConsoleColor.DarkRed;
                default: return ConsoleColor.White;
            }
        }
    }

    // Extension methods
    public static class StringExtensions
    {
        public static string Reverse(this string str)
        {
            if (string.IsNullOrEmpty(str))
                return str;

            char[] chars = str.ToCharArray();
            Array.Reverse(chars);
            return new string(chars);
        }

        public static int WordCount(this string str)
        {
            if (string.IsNullOrWhiteSpace(str))
                return 0;

            return str.Split(new[] { ' ', '\t', '\n', '\r' }, 
                           StringSplitOptions.RemoveEmptyEntries).Length;
        }
    }

    // Main program class
    class Program
    {
        static void Main(string[] args)
        {
            var logger = new ConsoleLogger();
            logger.Log(LogLevel.Info, "C# Compiler Showcase Starting...");

            try
            {
                // Test basic features
                TestBasicFeatures(logger);

                // Test collections and LINQ
                TestCollectionsAndLinq(logger);

                // Test generics
                TestGenerics(logger);

                // Test file I/O
                TestFileIO(logger);

                // Test date/time operations
                TestDateTime(logger);

                // Test exception handling
                TestExceptionHandling(logger);

                // Test async (simplified)
                TestAsync(logger);

                logger.Log(LogLevel.Info, "All tests completed successfully!");
            }
            catch (Exception ex)
            {
                logger.LogException(ex);
            }
        }

        static void TestBasicFeatures(ILogger logger)
        {
            logger.Log(LogLevel.Info, "\n=== Testing Basic Features ===");

            // String interpolation and extension methods
            string message = "Hello, World!";
            logger.Log(LogLevel.Debug, $"Original: {message}");
            logger.Log(LogLevel.Debug, $"Reversed: {message.Reverse()}");
            logger.Log(LogLevel.Debug, $"Word count: {message.WordCount()}");

            // Math operations
            double radius = 5.0;
            var sphere = new Sphere(radius);
            logger.Log(LogLevel.Info, sphere.GetDescription());
            logger.Log(LogLevel.Info, $"Surface area: {sphere.CalculateArea():F2}");
            logger.Log(LogLevel.Info, $"Volume: {sphere.CalculateVolume():F2}");

            // Value types
            var point = new Point3D(3, 4, 5);
            logger.Log(LogLevel.Debug, $"Point: {point}, Distance: {point.DistanceFromOrigin():F2}");
        }

        static void TestCollectionsAndLinq(ILogger logger)
        {
            logger.Log(LogLevel.Info, "\n=== Testing Collections and LINQ ===");

            // Create a list of numbers
            var numbers = new List<int> { 1, 2, 3, 4, 5, 6, 7, 8, 9, 10 };

            // LINQ queries
            var evens = numbers.Where(n => n % 2 == 0).ToList();
            var squares = numbers.Select(n => n * n).ToList();
            var sum = numbers.Sum();
            var avg = numbers.Average();

            logger.Log(LogLevel.Debug, $"Even numbers: {string.Join(", ", evens)}");
            logger.Log(LogLevel.Debug, $"Squares: {string.Join(", ", squares)}");
            logger.Log(LogLevel.Info, $"Sum: {sum}, Average: {avg:F2}");

            // Dictionary usage
            var capitals = new Dictionary<string, string>
            {
                ["USA"] = "Washington D.C.",
                ["UK"] = "London",
                ["France"] = "Paris",
                ["Japan"] = "Tokyo"
            };

            foreach (var kvp in capitals)
            {
                logger.Log(LogLevel.Debug, $"{kvp.Key}: {kvp.Value}");
            }

            // Complex LINQ
            var result = numbers
                .Where(n => n > 3)
                .Select(n => new { Number = n, Square = n * n, Cube = n * n * n })
                .Where(x => x.Square < 50)
                .OrderByDescending(x => x.Number)
                .ToList();

            logger.Log(LogLevel.Info, $"Complex LINQ result count: {result.Count}");
        }

        static void TestGenerics(ILogger logger)
        {
            logger.Log(LogLevel.Info, "\n=== Testing Generics ===");

            // Generic repository
            var shapeRepo = new Repository<Shape>(logger);

            // Note: Can't instantiate Shape directly as it's abstract
            // But we can store derived types
            var sphere1 = new Sphere(3.0);
            var sphere2 = new Sphere(5.0);

            // This would work with the repository pattern
            logger.Log(LogLevel.Info, "Generic repository test completed");

            // Generic methods
            T GetDefault<T>() => default(T);
            
            int defaultInt = GetDefault<int>();
            string defaultString = GetDefault<string>();
            
            logger.Log(LogLevel.Debug, $"Default int: {defaultInt}");
            logger.Log(LogLevel.Debug, $"Default string: {defaultString ?? "null"}");
        }

        static void TestFileIO(ILogger logger)
        {
            logger.Log(LogLevel.Info, "\n=== Testing File I/O ===");

            string testFile = "test_output.txt";
            string content = $"Test file created at {DateTime.Now}\nThis is a test of the C# compiler!";

            try
            {
                // Write file
                File.WriteAllText(testFile, content);
                logger.Log(LogLevel.Info, $"Created file: {testFile}");

                // Read file
                if (File.Exists(testFile))
                {
                    string readContent = File.ReadAllText(testFile);
                    logger.Log(LogLevel.Debug, $"File content: {readContent}");

                    // Delete file
                    File.Delete(testFile);
                    logger.Log(LogLevel.Info, "Test file deleted");
                }
            }
            catch (Exception ex)
            {
                logger.LogException(ex);
            }

            // Path operations
            string path = @"C:\Users\Test\Documents\file.txt";
            logger.Log(LogLevel.Debug, $"Directory: {Path.GetDirectoryName(path)}");
            logger.Log(LogLevel.Debug, $"Filename: {Path.GetFileName(path)}");
            logger.Log(LogLevel.Debug, $"Extension: {Path.GetExtension(path)}");
        }

        static void TestDateTime(ILogger logger)
        {
            logger.Log(LogLevel.Info, "\n=== Testing DateTime ===");

            DateTime now = DateTime.Now;
            DateTime utcNow = DateTime.UtcNow;
            DateTime tomorrow = now.AddDays(1);
            DateTime nextWeek = now.AddDays(7);

            logger.Log(LogLevel.Debug, $"Local time: {now:F}");
            logger.Log(LogLevel.Debug, $"UTC time: {utcNow:F}");
            logger.Log(LogLevel.Debug, $"Tomorrow: {tomorrow:d}");
            logger.Log(LogLevel.Debug, $"Next week: {nextWeek:D}");

            TimeSpan duration = nextWeek - now;
            logger.Log(LogLevel.Info, $"Days until next week: {duration.TotalDays}");

            // Date formatting
            logger.Log(LogLevel.Debug, $"Short date: {now:d}");
            logger.Log(LogLevel.Debug, $"Long date: {now:D}");
            logger.Log(LogLevel.Debug, $"Full date/time: {now:F}");
            logger.Log(LogLevel.Debug, $"ISO 8601: {now:yyyy-MM-ddTHH:mm:ss}");
        }

        static void TestExceptionHandling(ILogger logger)
        {
            logger.Log(LogLevel.Info, "\n=== Testing Exception Handling ===");

            try
            {
                // Test divide by zero
                int x = 10;
                int y = 0;
                // int result = x / y; // Would throw exception

                // Test null reference
                string str = null;
                // int length = str.Length; // Would throw exception

                // Test argument validation
                void ValidateAge(int age)
                {
                    if (age < 0 || age > 150)
                        throw new ArgumentException("Invalid age", nameof(age));
                }

                try
                {
                    ValidateAge(-5);
                }
                catch (ArgumentException ex)
                {
                    logger.Log(LogLevel.Warning, $"Caught expected exception: {ex.Message}");
                }

                logger.Log(LogLevel.Info, "Exception handling test completed");
            }
            catch (Exception ex)
            {
                logger.LogException(ex);
            }
        }

        static void TestAsync(ILogger logger)
        {
            logger.Log(LogLevel.Info, "\n=== Testing Async (Simplified) ===");

            // Simulate async with Task.Delay
            var task = Task.Delay(100);
            logger.Log(LogLevel.Debug, "Started async operation");
            
            task.Wait();
            logger.Log(LogLevel.Debug, "Async operation completed");

            // Task.Run simulation
            var result = Task.Run(() => {
                // Simulate work
                return 42;
            });

            logger.Log(LogLevel.Info, $"Task result: {result.Result}");
        }
    }
}