using Mono.Cecil;
using System;
using System.Collections.Generic;

namespace Generator
{
	public class AssemblyCollection : IAssemblyResolver
	{
		Dictionary<string, AssemblyDefinition> assemblies = new Dictionary<string, AssemblyDefinition>(StringComparer.Ordinal);

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
	}
}
