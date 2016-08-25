using System;
using System.Collections.Generic;
using System.Linq;

using Mono.Cecil;

namespace Generator.Types
{
	public class TypeDef : ITypeRequestSource
	{
		public Module Module { get; private set; }
		public TypeDefinition Type { get; private set; }
		public TypeKind Kind { get; private set; }

		private List<MethodDef> methods;
		public IEnumerable<MethodDef> Methods
		{
			get
			{
				return methods;
			}
		}

		private HashSet<TypeDef> dependencies = new HashSet<TypeDef>();
		public IEnumerable<TypeDef> Dependencies
		{
			get
			{
				return dependencies;
			}
		}

		private HashSet<TypeDef> reverseDependencies = new HashSet<TypeDef>();
		public IEnumerable<TypeDef> ReverseDependencies
		{
			get
			{
				return reverseDependencies;
			}
		}

		public IEnumerable<TypeDef> ForeignAssemblyDependencies
		{
			get
			{
				return dependencies.Where(t => t.Module.Assembly != Module.Assembly && t.Module.Assembly.Name.Name != "Windows.Foundation");
			}
		}

		public string Name
		{
			get
			{
				return Type.Name;
			}
		}


		public string Namespace
		{
			get
			{
				return Type.Namespace;
			}
		}

		public string DefinitionName
		{
			get
			{
				var name = Type.Name;
				int i = name.IndexOf('`');
				if (i >= 0)
				{
					name = name.Substring(0, i);
				}
				return name;
			}
		}

		public TypeDef(Generator gen, TypeDefinition t, AssemblyDefinition asm)
		{
			Type = t;
			Module = gen.GetModule(t.Namespace);
			Module.AssignAssembly(asm);

			if (t.IsEnum)
			{
				Kind = TypeKind.Enum;
			}
			else if (t.IsInterface)
			{
				Kind = TypeKind.Interface;
			}
			else if (t.IsValueType)
			{
				Kind = TypeKind.Struct;
			}
			else if (TypeHelpers.IsDelegate(t))
			{
				Kind = TypeKind.Delegate;
			}
			else if (t.IsClass)
			{
				Kind = TypeKind.Class;
			}
			else
			{
				throw new NotSupportedException();
			}

			if (Kind == TypeKind.Interface || Kind == TypeKind.Delegate)
			{
				methods = t.Methods.Where(m => m.Name != ".ctor").Select(m => new MethodDef(this, m)).ToList();
			}
		}

		public void AddDependency(TypeDef other)
		{
			dependencies.Add(other);
			other.reverseDependencies.Add(this);
		}

		public override int GetHashCode()
		{
			return Type.FullName.GetHashCode() ^ Module.Assembly.FullName.GetHashCode();
		}

		public override bool Equals(object obj)
		{
			if (!(obj is TypeDef))
				return false;

			return this.Type.Equals((obj as TypeDef).Type);
		}

		public override string ToString()
		{
			return this.Type.FullName;
		}
	}
}
