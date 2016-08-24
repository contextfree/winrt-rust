using Mono.Cecil;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

namespace Generator
{
	public class MethodDef : ITracksDependencies
	{
		public TypeDef DeclaringType { get; private set; }
		public MethodDefinition Method { get; private set; }

		private HashSet<TypeDef> dependencies = new HashSet<TypeDef>();
		public IEnumerable<TypeDef> Dependencies
		{
			get
			{
				return dependencies;
			}
		}

		public IEnumerable<TypeDef> ForeignAssemblyDependencies
		{
			get
			{
				return dependencies.Where(t => t.Module.Assembly != this.DeclaringType.Module.Assembly && t.Module.Assembly.Name.Name != "Windows.Foundation");
			}
		}

		public MethodDef(TypeDef type, MethodDefinition method)
		{
			DeclaringType = type;
			Method = method;
		}

		public override string ToString()
		{
			return Method.Name;
		}

		public void AddDependency(TypeDef other)
		{
			dependencies.Add(other);
			DeclaringType.AddDependency(other);
		}

		public string GetRawName()
		{
			var rawName = Method.Name;
			var overload = Method.CustomAttributes.FirstOrDefault(a => a.AttributeType.Name == "OverloadAttribute");
			if (overload != null)
			{
				rawName = (string)overload.ConstructorArguments[0].Value;
			}
			return rawName;
		}

		public string GetWrapperName(string rawName)
		{
			string name = NameHelpers.PreventKeywords(NameHelpers.CamelToSnakeCase(rawName.Replace("put_", "set_")));
			if (rawName.Contains("_")) // name already contains '_' -> might result in a name clash after renaming
			{
				if (DeclaringType.Methods.Select(mm => new Tuple<MethodDef, string>(mm, mm.GetRawName())).Where(mm => !mm.Item2.Contains("_")).Any(mm => mm.Item1.GetWrapperName(mm.Item2) == name))
				{
					name += "_";
				}
			}
			return name;
		}
	}
}
