using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;
using static System.Diagnostics.Debug;

using Mono.Cecil;
using Mono.Cecil.Rocks;

namespace Generator
{
	class Program
	{
		struct TypeCounter
		{
			public int Enums;
			public int Interfaces;
			public int Classes;
			public int Structs;
			public int Delegates;
			public int Methods;

			public int AllTypes()
			{
				return Enums + Interfaces + Classes + Structs + Delegates;
			}
		}

		static Dictionary<string, GenericInstanceType> genericInstantiations = new Dictionary<string, GenericInstanceType>();
		static Dictionary<string, TypeDefinition> definitionsWorklist = new Dictionary<string, TypeDefinition>();
		static HashSet<string> definitionsDone = new HashSet<string>();
		static string Imports = @"use ::{ComInterface, HString, HStringRef, ComPtr, ComIid, IUnknown};
use ::rt::{RtInterface, RtType, RtValueType, IInspectable, RtResult, Char}; use ::rt::handler::IntoInterface;";

	static void Main(string[] args)
		{
			if (args.Length < 1) throw new ArgumentException("Please specify result file path as first argument");

			TypeCounter counter = new TypeCounter { Enums = 0, Interfaces = 0, Classes = 0, Structs = 0 };

			var files = Directory.GetFiles(@"C:\Windows\System32\WinMetadata\");
			var collection = new AssemblyCollection(files);

			// TODO: enable all modules (disabled because it results in a huge file and very long compilation)
			var names = new string[] { "Windows.Foundation", "Windows.Devices", /*"Windows.Storage", "Windows.Media", "Windows.System", "Windows.Graphics"*/ };

			foreach (var asm in collection.Assemblies.Where(a => names.Contains(a.Name.Name)))
			{
				foreach (var t in asm.MainModule.Types.Where(t => t.FullName != "<Module>"))
				{
					definitionsWorklist.Add(t.FullName, t);
				}
			}

			var root = new Module("");

			while (definitionsWorklist.Count > 0)
			{
				var type = definitionsWorklist.First().Value;
				definitionsWorklist.Remove(type.FullName);
				definitionsDone.Add(type.FullName);

				Assert(type.Attributes.HasFlag(TypeAttributes.WindowsRuntime));

				var module = root.FindChild(type.Namespace);

				if (type.IsEnum)
				{
					counter.Enums++;
					WriteEnum(module, type);
				}
				else if (type.IsInterface)
				{
					counter.Interfaces++;
					counter.Methods += type.Methods.Count;
					WriteInterface(module, type, false);
				}
				else if (type.IsValueType)
				{
					counter.Structs++;
					WriteStruct(module, type);
				}
				else if (IsDelegate(type))
				{
					counter.Delegates++;
					counter.Methods += 1; // Invoke
					WriteInterface(module, type, true);
				}
				else if (type.IsClass)
				{
					counter.Classes++;
					WriteClass(module, type);
				}
				else
				{
					throw new NotImplementedException();
				}
			}

			Console.WriteLine("Generated {0} type definitions ({1} enums, {2} interfaces, {3} structs, {4} classes, {5} delegates) and {6} method definitions", counter.AllTypes(), counter.Enums, counter.Interfaces, counter.Structs, counter.Classes, counter.Delegates, counter.Methods);
			var instantiationsCopy = new List<GenericInstanceType>(genericInstantiations.Values);
			foreach (var t in instantiationsCopy)
			{
				WriteParametricIID(root, t);
			}
			Console.WriteLine("Found and generated IIDs for {0} distinct generic instantiations.", genericInstantiations.Count);
			Console.Write("Writing results to " + new FileInfo(args[0]).FullName + " ...");
			using (var file = new StreamWriter(args[0]))
			{
				file.WriteLine("// DO NOT MODIFY THIS FILE - IT IS AUTOMATICALLY GENERATED!");
				file.WriteLine(@"#![allow(non_camel_case_types, unused_imports)]");
				foreach (var child in root.Children.Values)
				{
					WriteModuleTree(child, file);
				}
				Console.WriteLine(" done.");
			}
			
			Console.ReadLine();
		}

		static bool IsDelegate(TypeDefinition t)
		{
			return t.IsClass && t.BaseType.FullName == "System.MulticastDelegate";
		}

		static void WriteEnum(Module module, TypeDefinition t)
		{
			var isFlags = t.CustomAttributes.Any(a => a.AttributeType.Name == "FlagsAttribute");
			var underlyingType = t.Fields.Single(f => f.Name == "value__").FieldType;
			string name = GetTypeName(t, TypeUsage.Define);
			module.Append(@"
		RT_ENUM! { enum " + name + ": " + GetTypeName(underlyingType, TypeUsage.Raw) + @" {
			" + String.Join(", ", t.Fields.Where(f => f.Name != "value__").Select(f => PreventKeywords(f.Name) + " (" + t.Name + "_" + f.Name + ") = " + f.Constant)) + @",
		}}");
		}

		static void WriteStruct(Module module, TypeDefinition t)
		{
			string name = GetTypeName(t, TypeUsage.Define);
			// TODO: derive(Eq) whenever possible?
			module.Append(@"
		RT_STRUCT! { struct " + name + @" {
			" + String.Join(", ", t.Fields.Select(f => PreventKeywords(f.Name) + ": " + GetTypeName(f.FieldType, TypeUsage.Raw))) + (t.Fields.Any() ? "," : "") +  @"
		}}");
		}

		static void WriteInterface(Module module, TypeDefinition t, bool isDelegate)
		{
			var guid = t.CustomAttributes.First(a => a.AttributeType.Name == "GuidAttribute");
			var exclusiveTo = t.CustomAttributes.SingleOrDefault(a => a.AttributeType.Name == "ExclusiveToAttribute");
			TypeDefinition exclusiveToType = null; // currently not used
			if (exclusiveTo != null)
			{
				Assert(exclusiveTo.ConstructorArguments[0].Type.FullName == "System.Type");
				exclusiveToType = exclusiveTo.ConstructorArguments[0].Value as TypeDefinition;
			}

			var name = GetTypeName(t, TypeUsage.Define);

			module.Append(@"
		DEFINE_IID!(IID_" + name + ", " + String.Join(", ", guid.ConstructorArguments.Select(a => a.Value)) + ");");

			string generic = "";
			string genericWithBounds = "";
			if (t.HasGenericParameters)
			{
				if (t.GenericParameters.Count > 2) throw new NotImplementedException("Not yet supported by RT_INTERFACE macro");
				generic = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name)) + ">";
				genericWithBounds = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name + ": RtType")) + ">";
			}

			var methods = t.Methods.Where(m => m.Name != ".ctor");

			if (!isDelegate)
			{
				module.Append(@"
		RT_INTERFACE!{interface " + name + generic + "(" + name + "Vtbl): IInspectable(IInspectableVtbl) [IID_" + name + @"] {
			" + String.Join(",\r\n			", methods.Select(m => GetRawMethodDeclaration(m))) + @"
		}}");
			}
			else
			{
				module.Append(@"
		RT_DELEGATE!{delegate " + name + generic + "(" + name + "Vtbl, " + name + "Impl) [IID_" + name + @"] {
			" + String.Join(",\r\n			", methods.Select(m => GetRawMethodDeclaration(m))) + @"
		}}");
			}

			module.Append(@"
		impl" + genericWithBounds + " " + name + generic + @" {
			" + String.Join("\r\n			", methods.Select(m => GetMethodWrapperDefinition(m)).Where(m => m != null)) + @"
		}");
		}

		static string GetRawMethodDeclaration(MethodDefinition m)
		{
			var name = GetRawMethodName(m);
			return "fn " + name + "(" + String.Join(", ", GetMethodParameterDeclarations(m)) + ") -> ::w::HRESULT";
		}

		static IEnumerable<string> GetMethodParameterDeclarations(MethodDefinition m)
		{
			yield return "&mut self";
			foreach (var p in m.Parameters)
			{
				Assert(!p.IsReturnValue);
				if (p.ParameterType.IsArray)
				{
					// need additional input size parameter (no matter if array is input or output)
					yield return p.Name + "Size: u32";
				}
				yield return PreventKeywords(FirstToLower(p.Name)) + ": " + GetTypeName(p.ParameterType, TypeUsage.Raw);
			}
			if (m.ReturnType.FullName != "System.Void")
			{
				yield return "out: *mut " + GetTypeName(m.ReturnType, TypeUsage.Raw);
			}
		}

		static string GetRawMethodName(MethodDefinition m)
		{
			var rawName = m.Name;
			var overload = m.CustomAttributes.FirstOrDefault(a => a.AttributeType.Name == "OverloadAttribute");
			if (overload != null)
			{
				rawName = (string)overload.ConstructorArguments[0].Value;
			}
			return rawName;
		}

		static string GetMethodWrapperName(MethodDefinition m, string rawName)
		{
			string name = PreventKeywords(CamelToSnakeCase(rawName.Replace("put_", "set_")));
			if (rawName.Contains("_")) // name already contains '_' -> might result in a name clash after renaming
			{
				if (m.DeclaringType.Methods.Select(mm => new Tuple<MethodDefinition, string>(mm, GetRawMethodName(mm))).Where(mm => !mm.Item2.Contains("_")).Any(mm => GetMethodWrapperName(mm.Item1, mm.Item2) == name))
				{
					name += "_";
				}
			}
			return name;
		}

		static string GetMethodWrapperDefinition(MethodDefinition m)
		{
			if (m.Parameters.Any(p => p.ParameterType.IsArray || p.ParameterType.IsByReference && (p.ParameterType as ByReferenceType).ElementType.IsArray) || m.ReturnType.IsArray)
			{
				Console.WriteLine("WARNING: Skipping method wrapper for {0} (arrays are not yet supported)", m.FullName);
				return null;
			}

			string rawName = GetRawMethodName(m);
			string name = GetMethodWrapperName(m, rawName);

			var input = new List<Tuple<string, TypeReference>>();
			var output = new List<Tuple<string, TypeReference>>();

			if (m.ReturnType.FullName != "System.Void")
			{
				output.Add(new Tuple<string, TypeReference>("out", m.ReturnType));
			}

			foreach (var p in m.Parameters)
			{
				string pname = PreventKeywords(FirstToLower(p.Name));
				if (p.ParameterType.IsByReference)
				{
					Assert(p.IsOut);
					output.Add(new Tuple<string, TypeReference>(pname, ((ByReferenceType)p.ParameterType).ElementType));
				}
				else
				{
					input.Add(new Tuple<string, TypeReference>(pname, p.ParameterType));
				}
			}

			string outType = String.Join(", ", output.Select(o => GetTypeName(o.Item2, TypeUsage.Out)));
			if (output.Count != 1)
			{
				outType = "(" + outType + ")"; // also works for count == 0 (empty tuple)
			}
			var inputParameters = new string[] { "&mut self" }.Concat(input.Select(i => i.Item1 + ": " + GetTypeName(i.Item2, TypeUsage.In)));
			var rawParams = new List<string> { "self" };
			foreach (var p in m.Parameters)
			{
				var pname = PreventKeywords(FirstToLower(p.Name));
				if (p.ParameterType.IsByReference)
				{
					// output parameter
					rawParams.Add("&mut " + pname);
				}
				else
				{
					// input parameter
					rawParams.Add(UnwrapInputParameter(pname, p.ParameterType));
				}
			}

			if (m.ReturnType.FullName != "System.Void")
			{
				rawParams.Add("&mut out");
			}

			var outInit = String.Join(" ", output.Select(o => "let mut " + o.Item1 + " = " + CreateUninitializedOutput(o.Item2) + ";"));
			if (outInit != "") outInit = "\r\n\t\t\t\t" + outInit;

			var outWrap = String.Join(", ", output.Select(o => WrapOutputParameter(o.Item1, o.Item2)));
			if (output.Count != 1)
			{
				outWrap = "(" + outWrap + ")"; // also works for count == 0 (empty tuple)
			}
			return "#[inline] pub unsafe fn " + name + "(" + String.Join(", ", inputParameters) + ") -> RtResult<" + outType + @"> {" + outInit + @"
				let hr = ((*self.lpVtbl)." + rawName + ")(" + String.Join(", ", rawParams) + ");" + @"
				if hr == ::w::S_OK { Ok(" + outWrap + @") } else { Err(hr) }
			}";
		}

		static string UnwrapInputParameter(string name, TypeReference t)
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
				return name + ".as_iid()";
			}
			else if (t.IsPrimitive)
			{
				switch (t.FullName)
				{
					case "System.Boolean":
						return "if " + name + "{ ::w::TRUE } else { ::w::FALSE }";
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

		static string CreateUninitializedOutput(TypeReference t)
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

		static string WrapOutputParameter(string name, TypeReference t)
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
				return name; // TODO
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
				return "::Guid::from(" + name + ")";
			}
			else if (t.IsPrimitive)
			{
				switch (t.FullName)
				{
					case "System.Boolean":
						return name + " == ::w::TRUE";
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

		static string CamelToSnakeCase(string name)
		{
			var newName = new StringBuilder();
			bool noUnderscore = true;
			bool previousUpper = false;
			foreach (var c in name)
			{
				if (char.IsUpper(c))
				{
					if (!noUnderscore && !previousUpper) newName.Append("_");
					newName.Append(char.ToLowerInvariant(c));
					previousUpper = true;
				}
				else
				{
					newName.Append(c);
					previousUpper = false;
				}

				noUnderscore = false;
				if (c == '_') noUnderscore = true;
			}
			return newName.ToString();
		}

		static string PreventKeywords(string name)
		{
			if (name == "type" || name == "Self" || name == "box" || name == "move") // TODO: add more keywords
			{
				name += "_";
			}

			return name;
		}

		static string FirstToLower(string name)
		{
			return name[0].ToString().ToLower() + name.Substring(1);
		}

		static void WriteClass(Module module, TypeDefinition t)
		{
			var activatable = t.CustomAttributes.FirstOrDefault(a => a.AttributeType.Name == "ActivatableAttribute");
			TypeDefinition factory = null;
			if (activatable != null && activatable.ConstructorArguments[0].Type.FullName == "System.Type")
			{
				factory = activatable.ConstructorArguments[0].Value as TypeDefinition;
			}

			if (t.Interfaces.Count == 0)
			{
				Console.WriteLine("WARNING: Skipping " + t + " (class without interfaces)");
				return;
			}
			var mainInterface = t.Interfaces[0];
			var aliasedType = GetTypeName(mainInterface, TypeUsage.Alias);
			var classType = GetTypeName(t, TypeUsage.Define);
			if (aliasedType.Contains("'a")) // if we had to introduce a lifetime parameter ...
			{
				// use static lifetime for classes
				aliasedType = aliasedType.Replace("'a", "'static");
			}

			bool needClassID = false;
			if (factory == null)
			{
				module.Append(@"
		RT_CLASS!{class " + classType + ": " + aliasedType + "}");
			}
			else
			{
				needClassID = true;
				module.Append(@"
		RT_CLASS!{class " + classType + ": " + aliasedType + " [" + GetTypeName(factory, TypeUsage.Alias) + "] [CLSID_" + classType + "]}");
			}

			var statics = t.CustomAttributes.Where(a => a.AttributeType.Name == "StaticAttribute").Select(a => (TypeDefinition)a.ConstructorArguments[0].Value);
			foreach (var staticType in statics)
			{
				var staticName = GetTypeName(staticType, TypeUsage.Define);
				needClassID = true;
				module.Append(@"
		RT_ACTIVATABLE!{" + staticName + " [CLSID_" + classType + "]}");
			}

			if (needClassID)
			{
				module.Append(@"
		DEFINE_CLSID!(CLSID_" + classType + " = &[" + StringToUTF16WithZero(t.FullName) + @"]);");
			}
		}

		// Returns the string as an UTF16 encoded, null-terminated sequence of u16 values
		static string StringToUTF16WithZero(string str)
		{
			return String.Join(",", str.Select(c => ((ushort)c).ToString()).Concat(new string[] {"0"}));
		}

		enum TypeUsage
		{
			In,
			Out,
			Raw,
			Define,
			Alias,
			GenericArg,
			GenericArgWithLifetime
		}

		static string GetTypeName(TypeReference t, TypeUsage usage)
		{
			if (t.IsGenericParameter)
			{
				switch (usage)
				{
					case TypeUsage.Raw: return t.Name + "::Abi";
					case TypeUsage.In: return t.Name + "::In";
					case TypeUsage.Out: return t.Name + "::Out";
					case TypeUsage.GenericArg:
					case TypeUsage.GenericArgWithLifetime:
						return t.Name;
					default: throw new NotSupportedException();
				}
			}
			if (t.IsByReference)
			{
				var ty = (ByReferenceType)t;
				return "*mut " + GetTypeName(ty.ElementType, usage);
			}
			else if (t.IsArray)
			{
				var ty = (ArrayType)t;
				return "*mut " + GetTypeName(ty.ElementType, usage);
			}
			else
			{
				return GetElementTypeName(t, usage);
			}
		}

		static string GetElementTypeName(TypeReference t, TypeUsage usage)
		{
			if (t.FullName == "System.String")
			{
				switch (usage)
				{
					case TypeUsage.Raw: return "::w::HSTRING";
					case TypeUsage.In: return "HStringRef";
					case TypeUsage.Out: return "HString";
					case TypeUsage.GenericArg: return "&str";
					case TypeUsage.GenericArgWithLifetime: return "&'a str";
					default: throw new NotSupportedException();
				}
			}
			else if (t.FullName == "System.Object")
			{
				switch (usage)
				{
					case TypeUsage.Raw: return "*mut IInspectable";
					case TypeUsage.GenericArg: return "&IInspectable";
					case TypeUsage.GenericArgWithLifetime: return "&'a IInspectable";
					case TypeUsage.Define: throw new NotSupportedException();
					case TypeUsage.In: return "&IInspectable";
					case TypeUsage.Out: return "ComPtr<IInspectable>";
					default: throw new InvalidOperationException();
				}
			}
			else if (t.FullName == "System.Guid")
			{
				Assert(usage != TypeUsage.Define);
				return usage == TypeUsage.Raw ? "::w::GUID" : "::Guid";
			}
			else if (t.IsPrimitive)
			{
				Assert(usage != TypeUsage.Define);
				switch (t.FullName)
				{
					case "System.Boolean":
						return usage == TypeUsage.Raw ? "::w::BOOL" : "bool";
					case "System.Byte":
						return "u8";
					case "System.Int16":
						return "i16";
					case "System.Int32":
						return "i32";
					case "System.Int64":
						return "i64";
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
						return "Char";
					default:
						throw new NotImplementedException("Primitive type: " + t.FullName);
				}
			}
			else
			{
				var def = t.Resolve();
				if (!definitionsWorklist.ContainsKey(def.FullName) && !definitionsDone.Contains(def.FullName))
				{
					definitionsWorklist.Add(def.FullName, def);
				}

				string name = null;
				if (usage == TypeUsage.Define)
				{
					name = t.Name;
				}
				else
				{
					name = "::rt::gen::" + t.Namespace.ToLower().Replace(".", "::") + "::" + t.Name;
				}

				int i = name.IndexOf('`');
				if (i >= 0) {
					name = name.Substring(0, i);
				}

				if (t.IsGenericInstance)
				{
					var ty = (GenericInstanceType)t;
					if (!ty.ContainsGenericParameter)
					{
						AddGenericInstantiation(ty);
					}
					var argUsage = (usage == TypeUsage.Define || usage == TypeUsage.Alias || usage == TypeUsage.GenericArgWithLifetime) ? TypeUsage.GenericArgWithLifetime : TypeUsage.GenericArg;
					name += "<" + String.Join(", ", ty.GenericArguments.Select(a => GetTypeName(a, argUsage))) + ">";
				}

				if (!t.IsValueType)
				{

					if (usage == TypeUsage.GenericArg || usage == TypeUsage.In)
					{
						name = "&" + name;
					}
					else if (usage == TypeUsage.GenericArgWithLifetime)
					{
						name = "&'a " + name;
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

		static void WriteModuleTree(Module mod, StreamWriter file, string path = null)
		{
			string name = mod.Name.ToLower();
			string newPath = path == null ? mod.Name : (path + "." + mod.Name);
			file.WriteLine("pub mod " + name + " { // " + newPath);
			file.Write(Imports);
			file.WriteLine(mod.Text.ToString());
			foreach (var child in mod.Children.Values)
			{
				WriteModuleTree(child, file, newPath);
			}
			file.WriteLine("} // " + newPath);
		}

		static TypeReference InstantiateType(GenericInstanceType ty, Dictionary<string, TypeReference> map)
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

		static void AddGenericInstantiation(GenericInstanceType ty)
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
				var type = InstantiateType(intf, genericParameterMap);
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
					var type = InstantiateType(pty, genericParameterMap);
					if (type is GenericInstanceType)
					{
						AddGenericInstantiation(type as GenericInstanceType);
					}
				}
			}
		}

		static Guid PinterfaceNamespace = new Guid("11F47AD5-7B73-42C0-ABAE-878B1E16ADEE");

		static void WriteParametricIID(Module root, GenericInstanceType t)
		{
			TypeDefinition def;
			var desc = GetTypeIIDDescriptor(t, out def);
			Assert(def != null); // def is only null for primitive types
			var name = GetTypeName(t, TypeUsage.Define);
			string lifetime = "";
			if (name.Contains("'a"))
			{
				lifetime = "<'a>";
			}
			var iidName = "IID_" + Regex.Replace(t.FullName.Substring(t.Namespace.Length + 1), @"[\.`<>,]", "_").TrimEnd('_');
			var guid = Utility.GuidUtility.Create(PinterfaceNamespace, desc);
			var guidStr = Regex.Replace(guid.ToString("X"), @"[\{\}]", "");
			var module = root.FindChild(def.Namespace);

			module.Append(@"
		RT_PINTERFACE!{ for" + lifetime + " " + name + " => [" + guidStr + "] as " + iidName + " }");
		}

		static string GetTypeIIDDescriptor(TypeReference t, out TypeDefinition def)
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
					case "System.Byte": return "u1";
					case "System.Int16": return "i2";
					case "System.Int32": return "i4";
					case "System.Int64": return "i8";
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

			var guid = GetGuidForType(def);
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
			else if (IsDelegate(def))
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

		static Guid? GetGuidForType(TypeDefinition t)
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
	}
}