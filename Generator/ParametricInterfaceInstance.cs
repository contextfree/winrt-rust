using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

using Mono.Cecil;

using Generator.Types;

namespace Generator
{
	public class ParametricInterfaceInstance : ITypeRequestSource
	{
		private static Guid Namespace = new Guid("11F47AD5-7B73-42C0-ABAE-878B1E16ADEE");

		public TypeDef Type { get; private set; }
		public Module Module
		{
			get
			{
				return Type.Module;
			}
		}

		private List<TypeDef> dependencies = new List<TypeDef>();
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
				return dependencies.Where(t => t.Module.Assembly != Module.Assembly && t.Module.Assembly.Name.Name != "Windows.Foundation");
			}
		}

		public string Name { get; private set; }

		private GenericInstanceType originalType;

		public ParametricInterfaceInstance(Generator gen, GenericInstanceType type)
		{
			originalType = type;
			Type = gen.GetTypeDefinition(type);
			Name = TypeHelpers.GetTypeName(gen, this, type, TypeUsage.Alias);
		}

		public void Emit()
		{
			var descriptor = GetTypeIIDDescriptor(Type.Generator, originalType);
			var iidName = "IID_" + Regex.Replace(originalType.FullName.Substring(Type.Namespace.Length + 1), @"[\.`<>,]", "_").TrimEnd('_');
			var guid = Utility.GuidUtility.Create(Namespace, descriptor);
			var guidStr = Regex.Replace(guid.ToString("X"), @"[\{\}]", "");

			var dependsOnAssemblies = new List<string>(ForeignAssemblyDependencies.GroupBy(t => t.Module.Assembly.Name.Name).Select(g => g.Key));
			var features = new FeatureConditions(dependsOnAssemblies);


			Module.Append(features.GetAttribute() + "RT_PINTERFACE!{ for " + Name + " => [" + guidStr + "] as " + iidName + " }");
		}

		private static string GetTypeIIDDescriptor(Generator gen, TypeReference t)
		{
			if (t.FullName == "System.String")
			{
				return "string";
			}
			else if (t.FullName == "System.Object")
			{
				return "cinterface(IInspectable)";
			}
			else if (t.FullName == "System.Guid")
			{
				return "g16";
			}
			else if (t.IsPrimitive)
			{
				switch (t.FullName)
				{
					case "System.Boolean": return "b1";
					case "System.SByte": return "i1";
					case "System.Int16": return "i2";
					case "System.Int32": return "i4";
					case "System.Int64": return "i8";
					case "System.Byte": return "u1";
					case "System.UInt16": return "u2";
					case "System.UInt32": return "u4";
					case "System.UInt64": return "u8";
					case "System.Single": return "f4";
					case "System.Double": return "f8";
					case "System.Char": return "c2";
					default:
						throw new NotImplementedException("Primitive type: " + t.FullName);
				}
			}

			var def = gen.GetTypeDefinition(t);
			var guid = def.GetGuid();

			if (def.Kind == TypeKind.Interface)
			{
				if (t.IsGenericInstance)
				{
					var ty = (GenericInstanceType)t;
					return "pinterface({" + guid.Value.ToString() + "};" + String.Join(";", ty.GenericArguments.Select(p => GetTypeIIDDescriptor(gen, p))) + ")";
				}
				else
				{
					return "{" + guid.Value.ToString() + "}";
				}
			}
			else if (def.Kind == TypeKind.Delegate)
			{
				if (t.IsGenericInstance)
				{
					var ty = (GenericInstanceType)t;
					return "pinterface({" + guid.Value.ToString() + "};" + String.Join(";", ty.GenericArguments.Select(p => GetTypeIIDDescriptor(gen, p))) + ")";
				}
				else
				{
					//return "delegate({" + guid.Value.ToString() + "})";
					throw new NotImplementedException(); // This has never been tested
				}

			}
			else if (def.Kind == TypeKind.Enum)
			{
				var underlyingType = GetTypeIIDDescriptor(gen, def.Type.Fields.Single(f => f.Name == "value__").FieldType);
				return "enum(" + def.Type.FullName + ";" + underlyingType + ")";
			}
			else if (def.Kind == TypeKind.Struct)
			{
				return "struct(" + def.Type.FullName + ";" + String.Join(";", def.Type.Fields.Select(f => GetTypeIIDDescriptor(gen, f.FieldType))) + ")";
			}
			else if (def.Kind == TypeKind.Class)
			{
				var defaultInterface = TypeHelpers.GetDefaultInterface(def.Type);
				return "rc(" + def.Type.FullName + ";" + GetTypeIIDDescriptor(gen, defaultInterface) + ")";
			}
			else
			{
				throw new NotImplementedException();
			}
		}

		public void AddDependency(TypeDef other)
		{
			dependencies.Add(other);
		}
	}
}
