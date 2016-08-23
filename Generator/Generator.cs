using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator
{
	public struct TypeCounter
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

	public class Generator
	{
		private TypeCounter counter = new TypeCounter { Enums = 0, Interfaces = 0, Classes = 0, Structs = 0 };

		private Dictionary<string, TypeDefinition> definitionsWorklist = new Dictionary<string, TypeDefinition>();

		private HashSet<string> definitionsDone = new HashSet<string>();

		private ParametricInterfaceManager pinterfaceManager = new ParametricInterfaceManager();

		public IEnumerable<TypeReference> BaseTypes { get; private set; }

		public AssemblyCollection Assemblies { get; private set; }

		private Module rootModule = new Module("");

		public Generator(AssemblyCollection assemblies)
		{
			Assemblies = assemblies;

			// TODO: enable all modules (disabled because it results in a huge file and very long compilation)
			var names = new string[] { "Windows.Foundation", "Windows.Devices", /*"Windows.Storage", "Windows.Media", "Windows.System", "Windows.Graphics"*/ };

			foreach (var asm in Assemblies.Assemblies.Where(a => names.Contains(a.Name.Name)))
			{
				foreach (var t in asm.MainModule.Types.Where(t => t.FullName != "<Module>"))
				{
					definitionsWorklist.Add(t.FullName, t);
				}
			}

			pinterfaceManager.AddBaseIReferenceInstantiations(Assemblies);
		}

		public void AddGenericInstantiation(GenericInstanceType ty)
		{
			pinterfaceManager.AddGenericInstantiation(ty);
		}

		public Module GetModule(string name)
		{
			return rootModule.FindChild(name);
		}

		public TypeDefinition GetTypeDefinition(string @namespace, string name)
		{
			return Assemblies.GetTypeDefinition(@namespace, name);
		}

		public void AddToWorklist(TypeDefinition def)
		{
			if (!definitionsWorklist.ContainsKey(def.FullName) && !definitionsDone.Contains(def.FullName))
			{
				definitionsWorklist.Add(def.FullName, def);
			}
		}

		public TypeCounter GenerateTypes(string outfile)
		{
			while (definitionsWorklist.Count > 0)
			{
				var type = definitionsWorklist.First().Value;
				definitionsWorklist.Remove(type.FullName);
				definitionsDone.Add(type.FullName);

				Assert(type.Attributes.HasFlag(TypeAttributes.WindowsRuntime));

				var module = GetModule(type.Namespace);

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
				else if (TypeHelpers.IsDelegate(type))
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

			return counter;
		}

		public int GenerateParametricInstantiations()
		{
			return pinterfaceManager.Generate(this);
		}

		private void WriteEnum(Module module, TypeDefinition t)
		{
			var isFlags = t.CustomAttributes.Any(a => a.AttributeType.Name == "FlagsAttribute");
			var underlyingType = t.Fields.Single(f => f.Name == "value__").FieldType;
			string name = TypeHelpers.GetTypeName(this, t, TypeUsage.Define);
			module.Append(@"
		RT_ENUM! { enum " + name + ": " + TypeHelpers.GetTypeName(this, underlyingType, TypeUsage.Raw) + @" {
			" + String.Join(", ", t.Fields.Where(f => f.Name != "value__").Select(f => NameHelpers.PreventKeywords(f.Name) + " (" + t.Name + "_" + f.Name + ") = " + f.Constant)) + @",
		}}");
		}

		private void WriteStruct(Module module, TypeDefinition t)
		{
			string name = TypeHelpers.GetTypeName(this, t, TypeUsage.Define);
			// TODO: derive(Eq) whenever possible?
			module.Append(@"
		RT_STRUCT! { struct " + name + @" {
			" + String.Join(", ", t.Fields.Select(f => NameHelpers.PreventKeywords(f.Name) + ": " + TypeHelpers.GetTypeName(this, f.FieldType, TypeUsage.Raw))) + (t.Fields.Any() ? "," : "") + @"
		}}");
		}

		private void WriteInterface(Module module, TypeDefinition t, bool isDelegate)
		{
			var guid = t.CustomAttributes.First(a => a.AttributeType.Name == "GuidAttribute");
			var exclusiveTo = t.CustomAttributes.SingleOrDefault(a => a.AttributeType.Name == "ExclusiveToAttribute");
			TypeDefinition exclusiveToType = null;
			if (exclusiveTo != null)
			{
				Assert(exclusiveTo.ConstructorArguments[0].Type.FullName == "System.Type");
				exclusiveToType = exclusiveTo.ConstructorArguments[0].Value as TypeDefinition;
			}

			bool isFactoryOrStatic = TypeHelpers.IsFactoryOrStatic(this, t, exclusiveToType);

			var name = TypeHelpers.GetTypeName(this, t, TypeUsage.Define);

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
			string prependStatic = isFactoryOrStatic ? "static " : "";

			if (!isDelegate)
			{
				module.Append(@"
		RT_INTERFACE!{" + prependStatic + "interface " + name + generic + "(" + name + "Vtbl): IInspectable(IInspectableVtbl) [IID_" + name + @"] {
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

		private string GetMethodWrapperDefinition(MethodDefinition m)
		{
			string rawName = GetRawMethodName(m);
			string name = GetMethodWrapperName(m, rawName);

			if (rawName == "GetMany" && m.DeclaringType.Namespace == "Windows.Foundation.Collections" &&
				(m.DeclaringType.Name == "IVectorView`1" || m.DeclaringType.Name == "IIterator`1" || m.DeclaringType.Name == "IVector`1"))
			{
				// This method has special semantics, since it takes an array and returns the number of elements that were filled
				// It uses the __RPC__out_ecount_part(capacity, *actual) annotation in the C headers
				// TODO ...
			}

			var input = new List<Tuple<string, TypeReference, InputKind>>();
			var output = new List<Tuple<string, TypeReference>>();

			foreach (var p in m.Parameters)
			{
				string pname = NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name));
				if (p.ParameterType.IsByReference)
				{
					Assert(p.IsOut);
					var realType = ((ByReferenceType)p.ParameterType).ElementType;
					output.Add(new Tuple<string, TypeReference>(pname, realType));
				}
				else
				{
					if (p.ParameterType.IsArray)
					{
						if (p.IsOut)
						{
							//TODO: this should probably be a mutable, write-only, empty slice -> what type?
							input.Add(new Tuple<string, TypeReference, InputKind>(pname + "Size", m.Module.Import(typeof(uint)), InputKind.Default));
							input.Add(new Tuple<string, TypeReference, InputKind>(pname, p.ParameterType, InputKind.Raw));
						}
						else
						{
							input.Add(new Tuple<string, TypeReference, InputKind>(pname, ((ArrayType)p.ParameterType).ElementType, InputKind.Slice));
						}
					}
					else
					{
						input.Add(new Tuple<string, TypeReference, InputKind>(pname, p.ParameterType, InputKind.Default));
					}
				}
			}

			if (m.ReturnType.FullName != "System.Void")
			{
				// this makes the actual return value the last in the tuple (if multiple)
				output.Add(new Tuple<string, TypeReference>("out", m.ReturnType));
			}

			string outType = String.Join(", ", output.Select(o => TypeHelpers.GetTypeName(this, o.Item2, TypeUsage.Out)));
			if (output.Count != 1)
			{
				outType = "(" + outType + ")"; // also works for count == 0 (empty tuple)
			}
			var inputParameters = new string[] { "&mut self" }.Concat(input.Select(i => i.Item1 + ": " + TypeHelpers.GetInputTypeName(this, i.Item2, i.Item3)));
			var rawParams = new List<string> { "self" };
			foreach (var p in m.Parameters)
			{
				var pname = NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name));
				if (p.ParameterType.IsByReference)
				{
					if (((ByReferenceType)p.ParameterType).ElementType.IsArray)
					{
						rawParams.Add("&mut " + pname + "Size");
					}

					// output parameter
					rawParams.Add("&mut " + pname);
				}
				else
				{
					// input parameter
					if (p.ParameterType.IsArray)
					{
						if (p.IsOut)
						{
							//TODO: this should probably be a mutable, write-only, empty slice -> what type?
							rawParams.Add(pname + "Size");
							rawParams.Add(TypeHelpers.UnwrapInputParameter(pname, p.ParameterType));
						}
						else
						{
							rawParams.Add(pname + ".len() as u32");
							rawParams.Add(pname + ".as_ptr() as *mut _");
						}
					}
					else
					{
						rawParams.Add(TypeHelpers.UnwrapInputParameter(pname, p.ParameterType));
					}
				}
			}

			if (m.ReturnType.FullName != "System.Void")
			{
				if (m.ReturnType.IsArray)
				{
					rawParams.Add("&mut outSize");
				}
				rawParams.Add("&mut out");
			}

			var outInit = String.Join(" ", output.SelectMany(o => TypeHelpers.CreateUninitializedOutputs(o.Item1, o.Item2)));
			if (outInit != "") outInit = "\r\n\t\t\t\t" + outInit;

			var outWrap = String.Join(", ", output.Select(o => TypeHelpers.WrapOutputParameter(o.Item1, o.Item2)));
			if (output.Count != 1)
			{
				outWrap = "(" + outWrap + ")"; // also works for count == 0 (empty tuple)
			}
			return "#[inline] pub unsafe fn " + name + "(" + String.Join(", ", inputParameters) + ") -> RtResult<" + outType + @"> {" + outInit + @"
				let hr = ((*self.lpVtbl)." + rawName + ")(" + String.Join(", ", rawParams) + ");" + @"
				if hr == ::w::S_OK { Ok(" + outWrap + @") } else { Err(hr) }
			}";
		}

		private string GetRawMethodDeclaration(MethodDefinition m)
		{
			var name = GetRawMethodName(m);
			return "fn " + name + "(" + String.Join(", ", GetMethodParameterDeclarations(m)) + ") -> ::w::HRESULT";
		}

		private IEnumerable<string> GetMethodParameterDeclarations(MethodDefinition m)
		{
			yield return "&mut self";
			foreach (var p in m.Parameters)
			{
				Assert(!p.IsReturnValue);
				int? lengthIs = null;
				var lengthIsAttribute = p.CustomAttributes.SingleOrDefault(a => a.AttributeType.Name == "LengthIsAttribute");
				if (lengthIsAttribute != null)
				{
					lengthIs = (int)lengthIsAttribute.ConstructorArguments[0].Value;
				}

				if (p.ParameterType.IsArray)
				{
					// need additional input size parameter (even if parameter is marked as [Out])
					yield return NameHelpers.FirstToLower(p.Name) + "Size: u32";
				}
				else if (p.ParameterType.IsByReference && (p.ParameterType as ByReferenceType).ElementType.IsArray)
				{
					Assert(!lengthIs.HasValue);
					// need additional output size parameter
					yield return NameHelpers.FirstToLower(p.Name) + "Size: *mut u32";
				}
				yield return NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name)) + ": " + TypeHelpers.GetTypeName(this, p.ParameterType, TypeUsage.Raw);
			}
			if (m.ReturnType.FullName != "System.Void")
			{
				if (m.ReturnType.IsArray)
				{
					yield return "outSize: *mut u32";
				}
				yield return "out: *mut " + TypeHelpers.GetTypeName(this, m.ReturnType, TypeUsage.Raw);
			}
		}

		private string GetRawMethodName(MethodDefinition m)
		{
			var rawName = m.Name;
			var overload = m.CustomAttributes.FirstOrDefault(a => a.AttributeType.Name == "OverloadAttribute");
			if (overload != null)
			{
				rawName = (string)overload.ConstructorArguments[0].Value;
			}
			return rawName;
		}

		private string GetMethodWrapperName(MethodDefinition m, string rawName)
		{
			string name = NameHelpers.PreventKeywords(NameHelpers.CamelToSnakeCase(rawName.Replace("put_", "set_")));
			if (rawName.Contains("_")) // name already contains '_' -> might result in a name clash after renaming
			{
				if (m.DeclaringType.Methods.Select(mm => new Tuple<MethodDefinition, string>(mm, GetRawMethodName(mm))).Where(mm => !mm.Item2.Contains("_")).Any(mm => GetMethodWrapperName(mm.Item1, mm.Item2) == name))
				{
					name += "_";
				}
			}
			return name;
		}

		private void WriteClass(Module module, TypeDefinition t)
		{
			var factory = TypeHelpers.GetFactoryType(t);
			var statics = TypeHelpers.GetStaticTypes(t);

			if (t.Interfaces.Count == 0 && !statics.Any())
			{
				Console.WriteLine("WARNING: Skipping " + t + " (class with no interfaces or statics)");
				return;
			}
			var classType = TypeHelpers.GetTypeName(this, t, TypeUsage.Define);
			bool needClassID = false;
			if (t.Interfaces.Count > 0)
			{
				var mainInterface = t.Interfaces[0];
				var aliasedType = TypeHelpers.GetTypeName(this, mainInterface, TypeUsage.Alias);

				if (factory == null)
				{
					module.Append(@"
		RT_CLASS!{class " + classType + ": " + aliasedType + "}");
				}
				else
				{
					needClassID = true;
					module.Append(@"
		RT_CLASS!{class " + classType + ": " + aliasedType + " [" + TypeHelpers.GetTypeName(this, factory, TypeUsage.Alias) + "] [CLSID_" + classType + "]}");
				}
			}

			foreach (var staticType in statics)
			{
				var staticName = TypeHelpers.GetTypeName(this, staticType, TypeUsage.Define);
				needClassID = true;
				module.Append(@"
		RT_ACTIVATABLE!{" + staticName + " [CLSID_" + classType + "]}");
			}

			if (needClassID)
			{
				module.Append(@"
		DEFINE_CLSID!(CLSID_" + classType + " = &[" + NameHelpers.StringToUTF16WithZero(t.FullName) + @"]);");
			}
		}

		public void WriteModuleTree(StreamWriter file)
		{
			file.WriteLine("// DO NOT MODIFY THIS FILE - IT IS AUTOMATICALLY GENERATED!");
			file.WriteLine(@"#![allow(non_camel_case_types, unused_imports)]");
			foreach (var child in rootModule.Children.Values)
			{
				WriteModuleTree(child, file);
			}
		}

		private void WriteModuleTree(Module mod, StreamWriter file, string path = null)
		{
			const string IMPORTS = @"use ::{ComInterface, HString, HStringArg, ComPtr, ComArray, ComIid, IUnknown};
use ::rt::{RtType, IInspectable, RtResult}; use ::rt::handler::IntoInterface;";

			string name = mod.Name.ToLower();
			string newPath = path == null ? mod.Name : (path + "." + mod.Name);
			file.WriteLine("pub mod " + name + " { // " + newPath);
			file.Write(IMPORTS);
			file.WriteLine(mod.Text.ToString());
			foreach (var child in mod.Children.Values)
			{
				WriteModuleTree(child, file, newPath);
			}
			file.WriteLine("} // " + newPath);
		}
	}
}
