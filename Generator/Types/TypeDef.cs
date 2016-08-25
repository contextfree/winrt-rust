using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator.Types
{
	public abstract class TypeDef : ITypeRequestSource
	{
		public Generator Generator { get; private set; }
		public Module Module { get; private set; }
		public TypeDefinition Type { get; private set; }
		public abstract TypeKind Kind { get; }

		public virtual bool CanBeSkipped
		{
			get
			{
				return false;
			}
		}

		public virtual IEnumerable<MethodDef> Methods
		{
			get
			{
				return Enumerable.Empty<MethodDef>();
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

		public static TypeDef Create(Generator gen, TypeDefinition t, AssemblyDefinition asm)
		{
			Assert(t.Attributes.HasFlag(TypeAttributes.WindowsRuntime));

			TypeDef result;
			if (t.IsEnum)
			{
				result = new EnumDef(t);
			}
			else if (t.IsInterface || TypeHelpers.IsDelegate(t))
			{
				result = new InterfaceDef(t);
			}
			else if (t.IsValueType)
			{
				result = new StructDef(t);
			}
			else if (t.IsClass)
			{
				result = new ClassDef(t);
			}
			else
			{
				throw new NotSupportedException();
			}

			result.Module = gen.GetModule(t.Namespace);
			result.Module.AssignAssembly(asm);
			result.Generator = gen; // TODO: get rid of this
			return result;
		}

		protected TypeDef(TypeDefinition t)
		{
			Type = t;
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

		public abstract void CollectDependencies();

		public abstract void Emit();
	}
}
