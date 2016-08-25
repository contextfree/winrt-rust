using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;
using Mono.Cecil.Rocks;

namespace Generator.Types
{
	public class TypeHelpers
	{
		public static bool IsDelegate(TypeDefinition t)
		{
			return t.IsClass && t.BaseType.FullName == "System.MulticastDelegate";
		}

		public static bool IsFactoryOrStatic(Generator gen, TypeDefinition t, TypeDefinition exclusiveToType)
		{
			var trimmedName = t.Name.TrimEnd('1', '2', '3', '4', '5', '6', '7', '8', '9');
			var guessedFromName = trimmedName.EndsWith("Factory") || trimmedName.EndsWith("Statics");
			HashSet<TypeDefinition> candidates = new HashSet<TypeDefinition>();
			if (exclusiveToType != null)
			{
				candidates.Add(exclusiveToType);
			}

			if (guessedFromName)
			{
				var targetTypeName = trimmedName.Substring(0, trimmedName.Length - 7); // "Factory" and "Statics" both have length 7
				var resolved = gen.Assemblies.GetTypeDefinition(t.Namespace, targetTypeName);
				if (resolved != null)
				{
					candidates.Add(resolved);
				}
				if (targetTypeName.StartsWith("I"))
				{
					var resolved2 = gen.Assemblies.GetTypeDefinition(t.Namespace, targetTypeName.Substring(1));
					if (resolved2 != null)
					{
						candidates.Add(resolved2);
					}
				}
			}

			return candidates.Any(c => GetFactoryType(c) == t || GetStaticTypes(c).Contains(t));
		}

		// TODO: change this and others into methods of TypeDef
		public static TypeDefinition GetFactoryType(TypeDefinition t)
		{
			var activatable = t.CustomAttributes.FirstOrDefault(a => a.AttributeType.Name == "ActivatableAttribute" && a.ConstructorArguments[0].Type.FullName == "System.Type");
			if (activatable != null)
			{
				return activatable.ConstructorArguments[0].Value as TypeDefinition;
			}
			return null;
		}

		public static IEnumerable<TypeDefinition> GetStaticTypes(TypeDefinition t)
		{
			return t.CustomAttributes.Where(a => a.AttributeType.Name == "StaticAttribute").Select(a => (TypeDefinition)a.ConstructorArguments[0].Value);
		}

		public static TypeReference InstantiateType(GenericInstanceType ty, Dictionary<string, TypeReference> map)
		{
			var genericArguments = ty.GenericArguments.Select(arg =>
			{
				if (arg.MetadataType == MetadataType.Var)
				{
					return map[arg.FullName];
				}
				else if (arg is GenericInstanceType)
				{
					return InstantiateType((GenericInstanceType)arg, map);
				}
				else
				{
					return arg;
				}
			}).ToArray();
			return ty.ElementType.MakeGenericInstanceType(genericArguments);
		}

		public static Guid? GetGuidForType(TypeDefinition t)
		{
			var guidAttr = t.CustomAttributes.FirstOrDefault(attr => attr.AttributeType.Name == "GuidAttribute");
			Guid? guid = null;
			if (guidAttr != null)
			{
				var a = guidAttr.ConstructorArguments;
				guid = new Guid((uint)a[0].Value, (ushort)a[1].Value, (ushort)a[2].Value, (byte)a[3].Value, (byte)a[4].Value,
					(byte)a[5].Value, (byte)a[6].Value, (byte)a[7].Value, (byte)a[8].Value, (byte)a[9].Value, (byte)a[10].Value);
			}

			return guid;
		}

		// ---------------- The following should be split by types ---------------- //

		public static string GetTypeName(Generator gen, ITypeRequestSource source, TypeReference t, TypeUsage usage)
		{
			if (t.IsGenericParameter)
			{
				switch (usage)
				{
					case TypeUsage.Raw: return t.Name + "::Abi";
					case TypeUsage.In: return "&" + t.Name + "::In";
					case TypeUsage.Out: return t.Name + "::Out";
					case TypeUsage.GenericArg: return t.Name;
					default: throw new NotSupportedException();
				}
			}
			if (t.IsByReference)
			{
				var ty = (ByReferenceType)t;
				return "*mut " + GetTypeName(gen, source, ty.ElementType, usage);
			}
			else if (t.IsArray)
			{
				var ty = (ArrayType)t;
				if (usage == TypeUsage.Out)
				{
					return "ComArray<" + GetTypeName(gen, source, ty.ElementType, TypeUsage.Raw) + ">";
				}
				else
				{
					return "*mut " + GetTypeName(gen, source, ty.ElementType, usage);
				}
			}
			else
			{
				return GetElementTypeName(gen, source, t, usage);
			}
		}

		private static string GetElementTypeName(Generator gen, ITypeRequestSource source, TypeReference t, TypeUsage usage)
		{
			if (t.FullName == "System.String")
			{
				switch (usage)
				{
					case TypeUsage.Raw: return "::w::HSTRING";
					case TypeUsage.In: return "&HStringArg";
					case TypeUsage.Out: return "HString";
					case TypeUsage.GenericArg: return "HString";
					default: throw new NotSupportedException();
				}
			}
			else if (t.FullName == "System.Object")
			{
				switch (usage)
				{
					case TypeUsage.Raw: return "*mut IInspectable";
					case TypeUsage.GenericArg: return "IInspectable";
					case TypeUsage.In: return "&IInspectable";
					case TypeUsage.Out: return "ComPtr<IInspectable>";
					default: throw new InvalidOperationException();
				}
			}
			else if (t.FullName == "System.Guid")
			{
				return "::Guid";
			}
			else if (t.IsPrimitive)
			{
				switch (t.FullName)
				{
					case "System.Boolean":
						return "bool";
					case "System.SByte":
						return "i8";
					case "System.Int16":
						return "i16";
					case "System.Int32":
						return "i32";
					case "System.Int64":
						return "i64";
					case "System.Byte":
						return "u8";
					case "System.UInt16":
						return "u16";
					case "System.UInt32":
						return "u32";
					case "System.UInt64":
						return "u64";
					case "System.Single":
						return "f32";
					case "System.Double":
						return "f64";
					case "System.Char":
						return "::Char";
					default:
						throw new NotImplementedException("Primitive type: " + t.FullName);
				}
			}
			else
			{
				var def = t.Resolve(); // TODO: remove
				gen.AddToWorklist(def); // TODO: remove
				var def2 = gen.GetTypeDefinition(t);
				source.AddDependency(def2);
				Assert(def == def2.Type);

				string name = null;
				if (def2.Module == source.Module)
				{
					name = t.Name;
				}
				else
				{
					name = "::rt::gen::" + t.Namespace.ToLower().Replace(".", "::") + "::" + t.Name;
				}

				int i = name.IndexOf('`');
				if (i >= 0)
				{
					name = name.Substring(0, i);
				}

				if (t.IsGenericInstance)
				{
					var ty = (GenericInstanceType)t;
					if (!ty.ContainsGenericParameter)
					{
						gen.AddGenericInstantiation(ty);
					}
					name += "<" + String.Join(", ", ty.GenericArguments.Select(a => GetTypeName(gen, source, a, TypeUsage.GenericArg))) + ">";
				}

				if (!t.IsValueType)
				{
					if (usage == TypeUsage.In)
					{
						name = "&" + name;
					}
					else if (usage == TypeUsage.GenericArg)
					{
						// leave name unchanged
					}
					else if (usage == TypeUsage.Raw)
					{
						name = "*mut " + name;
					}
					else if (usage == TypeUsage.Out)
					{
						name = "ComPtr<" + name + ">";
					}
				}

				return name;
			}
		}

		public static string GetInputTypeName(Generator gen, ITypeRequestSource source, TypeReference t, InputKind kind)
		{
			switch (kind)
			{
				case InputKind.Default: return GetTypeName(gen, source, t, TypeUsage.In);
				case InputKind.Raw: return GetTypeName(gen, source, t, TypeUsage.Raw);
				case InputKind.Slice: return "&[" + GetTypeName(gen, source, t, TypeUsage.Raw) + "]";
				default: throw new InvalidOperationException();
			}
		}

		public static string UnwrapInputParameter(string name, TypeReference t)
		{
			if (t.IsGenericParameter)
			{
				return t.Name + "::unwrap(" + name + ")";
			}
			if (t.IsByReference)
			{
				throw new NotSupportedException();
			}
			else if (t.IsArray)
			{
				return name; // TODO
			}
			else if (t.FullName == "System.String")
			{
				return name + ".get()";
			}
			else if (t.FullName == "System.Object")
			{
				return name + " as *const _ as *mut _";
			}
			else if (t.FullName == "System.Guid")
			{
				return name;
			}
			else if (t.IsPrimitive)
			{
				switch (t.FullName)
				{
					case "System.Boolean":
					case "System.Byte":
					case "System.Int16":
					case "System.Int32":
					case "System.Int64":
					case "System.UInt16":
					case "System.UInt32":
					case "System.UInt64":
					case "System.Single":
					case "System.Double":
					case "System.Char":
						return name;
					default:
						throw new NotImplementedException("Primitive type: " + t.FullName);
				}
			}
			else if (t.IsValueType)
			{
				return name;
			}
			else // reference type
			{
				return name + " as *const _ as *mut _";
			}
		}

		public static IEnumerable<string> CreateUninitializedOutputs(string name, TypeReference t)
		{
			if (t.IsArray)
			{
				yield return "let mut " + name + "Size = 0;";
			}
			yield return "let mut " + name + " = " + CreateUninitializedOutput(t) + ";";
		}

		public static string CreateUninitializedOutput(TypeReference t)
		{
			if (t.IsGenericParameter)
			{
				return t.Name + "::uninitialized()";
			}
			if (t.IsByReference)
			{
				throw new NotSupportedException();
			}
			else if (t.IsArray)
			{
				return "::std::ptr::null_mut()"; // TODO?
			}
			else if (t.FullName == "System.String")
			{
				return "::std::ptr::null_mut()";
			}
			else if (t.FullName == "System.Object")
			{
				return "::std::ptr::null_mut()";
			}
			else if (t.FullName == "System.Guid")
			{
				return "::std::mem::zeroed()";
			}
			else if (t.IsPrimitive)
			{
				switch (t.FullName)
				{
					case "System.Boolean":
					case "System.Byte":
					case "System.Int16":
					case "System.Int32":
					case "System.Int64":
					case "System.UInt16":
					case "System.UInt32":
					case "System.UInt64":
					case "System.Single":
					case "System.Double":
					case "System.Char":
						return "::std::mem::zeroed()";
					default:
						throw new NotImplementedException("Primitive type: " + t.FullName);
				}
			}
			else if (t.IsValueType)
			{
				return "::std::mem::zeroed()";
			}
			else // reference type
			{
				return "::std::ptr::null_mut()";
			}
		}

		public static string WrapOutputParameter(string name, TypeReference t)
		{
			if (t.IsGenericParameter)
			{
				return t.Name + "::wrap(" + name + ")";
			}
			if (t.IsByReference)
			{
				throw new NotSupportedException();
			}
			else if (t.IsArray)
			{
				return "ComArray::from_raw(" + name + "Size, " + name + ")";
			}
			else if (t.FullName == "System.String")
			{
				return "HString::wrap(" + name + ")";
			}
			else if (t.FullName == "System.Object")
			{
				return "ComPtr::wrap(" + name + ")";
			}
			else if (t.FullName == "System.Guid")
			{
				return name;
			}
			else if (t.IsPrimitive)
			{
				switch (t.FullName)
				{
					case "System.Boolean":
					case "System.Byte":
					case "System.Int16":
					case "System.Int32":
					case "System.Int64":
					case "System.UInt16":
					case "System.UInt32":
					case "System.UInt64":
					case "System.Single":
					case "System.Double":
					case "System.Char":
						return name;
					default:
						throw new NotImplementedException("Primitive type: " + t.FullName);
				}
			}
			else if (t.IsValueType)
			{
				return name;
			}
			else // reference type
			{
				return "ComPtr::wrap(" + name + ")";
			}
		}
	}

	public enum TypeUsage
	{
		In,
		Out,
		Raw,
		Alias,
		GenericArg,
	}

	public enum InputKind
	{
		Default,
		Raw,
		Slice
	}
}
