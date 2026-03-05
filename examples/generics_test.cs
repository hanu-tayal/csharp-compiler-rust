using System;
using System.Collections.Generic;

namespace GenericsTest
{
    // Generic class with single type parameter
    public class Box<T>
    {
        private T content;
        
        public Box(T item)
        {
            content = item;
        }
        
        public T Get()
        {
            return content;
        }
        
        public void Set(T item)
        {
            content = item;
        }
    }
    
    // Generic class with multiple type parameters
    public class Pair<TFirst, TSecond>
    {
        public TFirst First { get; set; }
        public TSecond Second { get; set; }
        
        public Pair(TFirst first, TSecond second)
        {
            First = first;
            Second = second;
        }
        
        public void Swap<T>(ref T a, ref T b)
        {
            T temp = a;
            a = b;
            b = temp;
        }
    }
    
    // Generic interface
    public interface IRepository<T> where T : class
    {
        void Add(T entity);
        T Get(int id);
        IEnumerable<T> GetAll();
        void Update(T entity);
        void Delete(T entity);
    }
    
    // Generic class with constraints
    public class Repository<T> : IRepository<T> where T : class, new()
    {
        private List<T> items = new List<T>();
        
        public void Add(T entity)
        {
            items.Add(entity);
        }
        
        public T Get(int id)
        {
            return id < items.Count ? items[id] : null;
        }
        
        public IEnumerable<T> GetAll()
        {
            return items;
        }
        
        public void Update(T entity)
        {
            // Update logic here
        }
        
        public void Delete(T entity)
        {
            items.Remove(entity);
        }
        
        public T CreateNew()
        {
            return new T();
        }
    }
    
    // Generic method examples
    public static class GenericMethods
    {
        // Generic method with type inference
        public static T Max<T>(T a, T b) where T : IComparable<T>
        {
            return a.CompareTo(b) > 0 ? a : b;
        }
        
        // Generic method with multiple type parameters
        public static TOutput Convert<TInput, TOutput>(TInput input, Func<TInput, TOutput> converter)
        {
            return converter(input);
        }
        
        // Generic extension method
        public static bool IsDefault<T>(this T value)
        {
            return EqualityComparer<T>.Default.Equals(value, default(T));
        }
    }
    
    // Test entity class
    public class Product
    {
        public int Id { get; set; }
        public string Name { get; set; }
        public decimal Price { get; set; }
        
        public Product() { }
        
        public Product(int id, string name, decimal price)
        {
            Id = id;
            Name = name;
            Price = price;
        }
    }
    
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Testing Generic Types");
            Console.WriteLine("====================");
            
            // Test Box<T>
            Box<int> intBox = new Box<int>(42);
            Console.WriteLine($"Int box contains: {intBox.Get()}");
            
            Box<string> stringBox = new Box<string>("Hello generics!");
            Console.WriteLine($"String box contains: {stringBox.Get()}");
            
            // Test Pair<T1, T2>
            Pair<string, int> pair = new Pair<string, int>("Age", 25);
            Console.WriteLine($"Pair: {pair.First} = {pair.Second}");
            
            // Test generic List<T>
            List<double> numbers = new List<double> { 1.5, 2.3, 3.7, 4.1, 5.9 };
            Console.WriteLine($"Numbers count: {numbers.Count}");
            
            // Test Repository<T>
            Repository<Product> productRepo = new Repository<Product>();
            productRepo.Add(new Product(1, "Laptop", 999.99m));
            productRepo.Add(new Product(2, "Mouse", 29.99m));
            productRepo.Add(new Product(3, "Keyboard", 79.99m));
            
            Console.WriteLine("\nProducts in repository:");
            foreach (var product in productRepo.GetAll())
            {
                Console.WriteLine($"  {product.Id}: {product.Name} - ${product.Price}");
            }
            
            // Test generic methods
            int maxInt = GenericMethods.Max(10, 20);
            Console.WriteLine($"\nMax of 10 and 20: {maxInt}");
            
            string maxString = GenericMethods.Max("apple", "banana");
            Console.WriteLine($"Max of 'apple' and 'banana': {maxString}");
            
            // Test type inference
            var converted = GenericMethods.Convert(123, x => x.ToString());
            Console.WriteLine($"Converted 123 to string: '{converted}'");
            
            // Test extension method
            int zero = 0;
            string empty = "";
            Console.WriteLine($"\nIs 0 default? {zero.IsDefault()}");
            Console.WriteLine($"Is empty string default? {empty.IsDefault()}");
            
            // Test nullable
            int? nullable = null;
            Console.WriteLine($"Is null int? default? {nullable.IsDefault()}");
            
            // Test Dictionary<K,V>
            Dictionary<string, Product> productDict = new Dictionary<string, Product>();
            productDict["laptop"] = new Product(1, "Laptop", 999.99m);
            productDict["mouse"] = new Product(2, "Mouse", 29.99m);
            
            if (productDict.TryGetValue("laptop", out Product laptop))
            {
                Console.WriteLine($"\nFound product: {laptop.Name}");
            }
            
            // Test generic variance (covariance)
            IEnumerable<Product> products = productRepo.GetAll();
            IEnumerable<object> objects = products; // Covariance
            
            // Test generic array
            Product[] productArray = new Product[]
            {
                new Product(1, "Item1", 10.0m),
                new Product(2, "Item2", 20.0m)
            };
            
            // Test Func and Action delegates
            Func<int, int, int> add = (a, b) => a + b;
            Action<string> print = msg => Console.WriteLine(msg);
            
            int sum = add(5, 3);
            print($"\n5 + 3 = {sum}");
            
            Console.WriteLine("\nAll generic tests completed!");
        }
    }
}