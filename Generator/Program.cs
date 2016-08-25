using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using static System.Diagnostics.Debug;

using Generator.Types;

namespace Generator
{
	class Program
	{
		static void Main(string[] args)
		{
			if (args.Length < 1) throw new ArgumentException("Please specify result file path as first argument");

			var generator = new Generator(new AssemblyCollection(Directory.GetFiles(@"C:\Windows\System32\WinMetadata\")));
			generator.CollectDependencies();
			PrintStatistics(generator.AllTypes);
			PrintDependencyStatistics(generator.AllTypes);
			//WriteDependencyGraph(generator.AllTypes);
			var emittedTypes = generator.EmitTypes();
			int pinterfaceCount = generator.EmitParametricInstances(emittedTypes);
			Console.WriteLine("Found and generated IIDs for {0} distinct generic instances.", pinterfaceCount);
			Console.Write("Writing results to " + new FileInfo(args[0]).FullName + " ...");
			using (var file = new StreamWriter(args[0]))
			{
				generator.WriteModuleTree(file);
				Console.WriteLine(" done.");
			}
		}

		static void PrintStatistics(IEnumerable<TypeDef> types)
		{
			var groups = types.GroupBy(t => t.Kind);
			var enums = groups.Single(g => g.Key == TypeKind.Enum);
			var interfaces = groups.Single(g => g.Key == TypeKind.Interface);
			var structs = groups.Single(g => g.Key == TypeKind.Struct);
			var classes = groups.Single(g => g.Key == TypeKind.Class);
			var delegates = groups.Single(g => g.Key == TypeKind.Delegate);
			var methods = interfaces.Sum(i => i.Methods.Count()) + (delegates.Sum(d => d.Methods.Count()));
			Console.WriteLine("Generated {0} type definitions ({1} enums, {2} interfaces, {3} structs, {4} classes, {5} delegates) and {6} method definitions", types.Count(), enums.Count(), interfaces.Count(), structs.Count(), classes.Count(), delegates.Count(), methods);
		}

		static void PrintDependencyStatistics(IEnumerable<TypeDef> types)
		{
			Console.WriteLine("Listing methods with dependencies from different assemblies (excluding dependencies on Windows.Foundation):");
			var typesWithForeignAssemblyDeps = types.Where(t => t.ForeignAssemblyDependencies.Any());

			var methods = types.Where(t => t.Methods != null && t.Methods.Any()).SelectMany(t => t.Methods);
			var methodsWithForeignAssemblyDeps = methods.Where(m => m.ForeignAssemblyDependencies.Any());

			var methodsByAssembly = methods.GroupBy(m => m.DeclaringType.Module.Assembly);

			foreach (var group in methodsWithForeignAssemblyDeps.GroupBy(m => m.DeclaringType.Module.Assembly))
			{
				var totalMethodsInAssembly = methodsByAssembly.Single(g => g.Key == group.Key);
				Console.WriteLine("\tMethods in {0} ({1} of {2}):", group.Key.Name.Name, group.Count(), totalMethodsInAssembly.Count());
				foreach (var a in group.SelectMany(m => m.ForeignAssemblyDependencies).GroupBy(t => t.Module.Assembly))
				{
					var asm = a.Key;
					Console.WriteLine("\t- {0:D2} methods depending on {1}", group.Count(m => m.ForeignAssemblyDependencies.Any(dep => dep.Module.Assembly == asm)), asm.Name.Name);
				}

			}

			var structsWithForeignAssemblyDeps = typesWithForeignAssemblyDeps.Where(t => t.Kind == TypeKind.Struct);
			Assert(!structsWithForeignAssemblyDeps.Any());
		}

		static void WriteDependencyGraph(IEnumerable<TypeDef> types)
		{
			StringBuilder sb = new StringBuilder();
			sb.AppendLine("digraph G {");

			foreach (var cluster in types.GroupBy(t => t.Module.Assembly.Name.Name))
			{
				sb.AppendLine(@"
subgraph cluster_" + cluster.Key.Replace(".", "_") + @" {
	style = filled;
	color = lightgrey;
	node[style = filled, color = white];");
				sb.AppendFormat("	label = \"{0}\";", cluster.Key);
				foreach (var t in cluster)
				{
					if (t.ForeignAssemblyDependencies.Any())
					{
						sb.AppendLine();
						sb.AppendFormat("	\"{0}\";", t.Type.FullName);
					}
				}
				sb.AppendLine();
				sb.AppendLine("}");
			}

			foreach (var t in types)
			{
				if (t.Methods != null && t.Methods.Any())
				{
					foreach (var m in t.Methods)
					{
						foreach (var dep in m.ForeignAssemblyDependencies)
						{
							sb.AppendFormat("\"{0}\" -> \"cluster_{1}\" [label=\"{2}\"]", t.Type.FullName, dep.Module.Assembly.Name.Name.Replace(".", "_"), m.GetRawName());
							sb.AppendLine();
						}
					}
				}
				else
				{
					foreach (var dep in t.ForeignAssemblyDependencies)
					{
						sb.AppendFormat("\"{0}\" -> \"cluster_{1}\"", t.Type.FullName, dep.Module.Assembly.Name.Name.Replace(".", "_"));
						sb.AppendLine();
					}
				}
			}

			sb.AppendLine("}");

			using (var file = new StreamWriter("dependencies.dot")) // file can be processed with the GraphViz tool `fdp`
			{
				file.WriteLine(sb.ToString());
			}
		}
	}
}