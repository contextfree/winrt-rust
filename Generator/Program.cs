using System;
using System.IO;

namespace Generator
{
	class Program
	{
		static void Main(string[] args)
		{
			if (args.Length < 1) throw new ArgumentException("Please specify result file path as first argument");

			var generator = new Generator(new AssemblyCollection(Directory.GetFiles(@"C:\Windows\System32\WinMetadata\")));
			var counter = generator.GenerateTypes(args[0]);
			Console.WriteLine("Generated {0} type definitions ({1} enums, {2} interfaces, {3} structs, {4} classes, {5} delegates) and {6} method definitions", counter.AllTypes(), counter.Enums, counter.Interfaces, counter.Structs, counter.Classes, counter.Delegates, counter.Methods);
			int pinterfaceCount = generator.GenerateParametricInstantiations();
			Console.WriteLine("Found and generated IIDs for {0} distinct generic instantiations.", pinterfaceCount);
			Console.Write("Writing results to " + new FileInfo(args[0]).FullName + " ...");
			using (var file = new StreamWriter(args[0]))
			{
				generator.WriteModuleTree(file);
				Console.WriteLine(" done.");
			}
		}
	}
}