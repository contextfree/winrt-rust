using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;
using static System.Diagnostics.Debug;

using Mono.Cecil;
using Mono.Cecil.Rocks;

namespace Generator
{
	public class ParametricInterfaceManager
	{
		private static Guid Namespace = new Guid("11F47AD5-7B73-42C0-ABAE-878B1E16ADEE");

		private static IEnumerable<Type> baseTypes = new List<Type>
		{
			//typeof(void),
			typeof(byte),
			typeof(sbyte),
			typeof(short),
			typeof(ushort),
			typeof(int),
			typeof(uint),
			typeof(long),
			typeof(ulong),
			typeof(float),
			typeof(double),
			typeof(char),
			typeof(bool),
			typeof(string),
			typeof(object),
			typeof(Guid),
		};

		private static IEnumerable<string> baseTypes2 = new List<string>
		{
			"DateTime",
			"TimeSpan",
			"Point",
			"Size",
			"Rect"
		};


		private Dictionary<string, GenericInstanceType> genericInstantiations = new Dictionary<string, GenericInstanceType>();

		public void AddBaseIReferenceInstantiations(AssemblyCollection assemblies)
		{
			var tyIReference = assemblies.GetTypeDefinition("Windows.Foundation", "IReference`1");
			var tyIReferenceArray = assemblies.GetTypeDefinition("Windows.Foundation", "IReferenceArray`1");

			var allBaseTypes = baseTypes.Select(t => assemblies.Assemblies.First().MainModule.Import(t)).Concat(baseTypes2.Select(t => assemblies.GetTypeDefinition("Windows.Foundation", t)));

			foreach (var baseType in allBaseTypes)
			{
				AddGenericInstantiation(tyIReference.MakeGenericInstanceType(baseType));
				AddGenericInstantiation(tyIReferenceArray.MakeGenericInstanceType(baseType));
			}
		}

		public void AddGenericInstantiation(GenericInstanceType ty)
		{
			if (genericInstantiations.ContainsKey(ty.FullName)) return;

			genericInstantiations.Add(ty.FullName, ty);
			// recursively add generic arguments
			foreach (var arg in ty.GenericArguments)
			{
				if (arg.IsGenericInstance)
				{
					AddGenericInstantiation((GenericInstanceType)arg);
				}
			}

			// build mapping from generic parameter names to instantiated types
			TypeDefinition def = ty.Resolve();
			var genericParameterMap = new Dictionary<string, TypeReference>();
			for (int i = 0; i < def.GenericParameters.Count; i++)
			{
				genericParameterMap.Add(def.GenericParameters[i].FullName, ty.GenericArguments[i]);
			}

			// recursively add implemented interfaces	
			foreach (var intf in def.Interfaces.Where(i => i.ContainsGenericParameter && i is GenericInstanceType).Cast<GenericInstanceType>())
			{
				var type = TypeHelpers.InstantiateType(intf, genericParameterMap);
				if (type is GenericInstanceType)
				{
					AddGenericInstantiation(type as GenericInstanceType);
				}
			}

			// recursively add other instantiations introduced by function parameters
			foreach (var m in def.Methods)
			{
				foreach (GenericInstanceType pty in m.Parameters
					.Select(p => p.ParameterType)
					.Concat(Enumerable.Repeat(m.ReturnType, 1))
					.Where(t => t.ContainsGenericParameter && t is GenericInstanceType)
					.Cast<GenericInstanceType>())
				{
					var type = TypeHelpers.InstantiateType(pty, genericParameterMap);
					if (type is GenericInstanceType)
					{
						AddGenericInstantiation(type as GenericInstanceType);
					}
				}
			}
		}

		public int Generate(Generator gen)
		{
			var instantiationsCopy = new List<GenericInstanceType>(genericInstantiations.Values);
			foreach (var t in instantiationsCopy)
			{
				WriteParametricIID(gen, t);
			}
			return genericInstantiations.Count;
		}

		public void WriteParametricIID(Generator gen, GenericInstanceType t)
		{
			TypeDefinition def;
			var desc = GetTypeIIDDescriptor(t, out def);
			Assert(def != null); // def is only null for primitive types
			var name = TypeHelpers.GetTypeName(gen, t, TypeUsage.Define);
			var iidName = "IID_" + Regex.Replace(t.FullName.Substring(t.Namespace.Length + 1), @"[\.`<>,]", "_").TrimEnd('_');
			var guid = Utility.GuidUtility.Create(Namespace, desc);
			var guidStr = Regex.Replace(guid.ToString("X"), @"[\{\}]", "");
			var module = gen.GetModule(def.Namespace);

			module.Append(@"
		RT_PINTERFACE!{ for " + name + " => [" + guidStr + "] as " + iidName + " }");
		}

		private static string GetTypeIIDDescriptor(TypeReference t, out TypeDefinition def)
		{
			def = null;

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

			def = t.Resolve();

			var guid = TypeHelpers.GetGuidForType(def);
			TypeDefinition def2;

			if (def.IsInterface)
			{
				if (t.IsGenericInstance)
				{
					var ty = (GenericInstanceType)t;
					return "pinterface({" + guid.Value.ToString() + "};" + String.Join(";", ty.GenericArguments.Select(p => GetTypeIIDDescriptor(p, out def2))) + ")";
				}
				else
				{
					return "{" + guid.Value.ToString() + "}";
				}
			}
			else if (TypeHelpers.IsDelegate(def))
			{
				if (t.IsGenericInstance)
				{
					var ty = (GenericInstanceType)t;
					return "pinterface({" + guid.Value.ToString() + "};" + String.Join(";", ty.GenericArguments.Select(p => GetTypeIIDDescriptor(p, out def2))) + ")";
				}
				else
				{
					//return "delegate({" + guid.Value.ToString() + "})";
					throw new NotImplementedException(); // This has never been tested
				}

			}
			else if (def.IsEnum)
			{

				var underlyingType = GetTypeIIDDescriptor(def.Fields.Single(f => f.Name == "value__").FieldType, out def2);
				return "enum(" + def.FullName + ";" + underlyingType + ")";
			}
			else if (def.IsValueType)
			{
				return "struct(" + def.FullName + ";" + String.Join(";", def.Fields.Select(f => GetTypeIIDDescriptor(f.FieldType, out def2))) + ")";
			}
			else if (def.IsClass)
			{
				var mainInterface = def.Interfaces[0];
				if (def.CustomAttributes.Any(a => a.AttributeType.Name == "DefaultInterfaceAttribute"))
					throw new NotImplementedException();
				TypeDefinition def3;
				return "rc(" + def.FullName + ";" + GetTypeIIDDescriptor(mainInterface, out def3) + ")";
			}
			else
			{
				throw new NotImplementedException();
			}
		}
	}
}
