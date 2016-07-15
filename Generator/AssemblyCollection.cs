using Mono.Cecil;
using System;
using System.Collections.Generic;

namespace Generator
{
	class AssemblyCollection : IAssemblyResolver
	{
		Dictionary<string, AssemblyDefinition> assemblies = new Dictionary<string, AssemblyDefinition>(StringComparer.Ordinal);
		SortedList<string, AssemblyDefinition> sorted = new SortedList<string, AssemblyDefinition>(new ComparerReverser<string>(Comparer<string>.Default));

		public IEnumerable<AssemblyDefinition> Assemblies
		{
			get
			{
				return assemblies.Values;
			}
		}

		public AssemblyCollection(IEnumerable<string> paths)
		{
			foreach (var p in paths)
			{
				var asm = AssemblyDefinition.ReadAssembly(p, new ReaderParameters() { AssemblyResolver = this });
				assemblies.Add(asm.FullName, asm);
				sorted.Add(asm.Name.Name, asm);
			}
		}

		public AssemblyDefinition Resolve(string fullName)
		{
			return assemblies[fullName];
		}

		public AssemblyDefinition Resolve(AssemblyNameReference name)
		{
			return assemblies[name.FullName];
		}

		public AssemblyDefinition Resolve(string fullName, ReaderParameters parameters)
		{
			throw new NotImplementedException();
		}

		public AssemblyDefinition Resolve(AssemblyNameReference name, ReaderParameters parameters)
		{
			throw new NotImplementedException();
		}

		public TypeDefinition GetTypeDefinition(string @namespace, string name)
		{
			// because `sorted` is in reverse order, we will get the most specific one first
			foreach (var pair in sorted)
			{
				if (@namespace.StartsWith(pair.Key))
				{
					var module = pair.Value.MainModule;
					return module.Import(new TypeReference(@namespace, name, module, module)).Resolve();
				}	
			}
			throw new KeyNotFoundException();
		}
	}

	// A generic comparer that reverses the action of a wrapped comparer.
	public sealed class ComparerReverser<T> : IComparer<T>
	{
		private readonly IComparer<T> wrappedComparer;

		// Initializes an instance of a ComparerReverser that takes a wrapped comparer
		// and returns the inverse of the comparison.
		public ComparerReverser(IComparer<T> wrappedComparer)
		{
			if (wrappedComparer == null)
			{
				throw new ArgumentNullException("wrappedComparer");
			}

			this.wrappedComparer = wrappedComparer;
		}

		// Compares two objects and returns a value indicating whether 
		// one is less than, equal to, or greater than the other.
		public int Compare(T x, T y)
		{
			// to reverse compare, just invert the operands....
			return wrappedComparer.Compare(y, x);
		}
	}
}
