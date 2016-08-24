using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator
{
	public class Generator
	{
		private Dictionary<string, TypeDef> definitionsList = new Dictionary<string, TypeDef>();

		// TODO: remove these
		private Dictionary<string, TypeDefinition> definitionsWorklist = new Dictionary<string, TypeDefinition>();
		private HashSet<string> definitionsDone = new HashSet<string>();

		private ParametricInterfaceManager pinterfaceManager = new ParametricInterfaceManager();

		public IEnumerable<TypeReference> BaseTypes { get; private set; }

		public AssemblyCollection Assemblies { get; private set; }

		private Module rootModule = new Module(null, "");

		public IEnumerable<TypeDef> AllTypes
		{
			get
			{
				return definitionsList.Values;
			}
		}

		public Generator(AssemblyCollection assemblies)
		{
			Assemblies = assemblies;

			// TODO: enable all modules (disabled because it results in a huge file and very long compilation)
			var names = new string[] { "Windows.Foundation", "Windows.Devices", /*"Windows.Storage", "Windows.Media", "Windows.System", "Windows.Graphics"*/ };

			foreach (var asm in Assemblies.Assemblies/*.Where(a => names.Contains(a.Name.Name))*/)
			{
				foreach (var t in asm.MainModule.Types.Where(t => t.FullName != "<Module>"))
				{
					definitionsWorklist.Add(t.FullName, t);
					definitionsList.Add(t.FullName, new TypeDef(this, t, asm));
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

		public void AddToWorklist(TypeDefinition def)
		{
			// TODO: remove this method, as it is not necessary to add anything to worklist later when assemblies are not filtered in the beginning
			if (!definitionsWorklist.ContainsKey(def.FullName) && !definitionsDone.Contains(def.FullName))
			{
				definitionsWorklist.Add(def.FullName, def);
			}
		}

		public TypeDef GetTypeDefinition(TypeReference t)
		{
			if (t.IsGenericInstance)
			{
				t = ((GenericInstanceType)t).ElementType;
			}
			return definitionsList[t.FullName];
		}

		public void GenerateTypes()
		{
			while (definitionsWorklist.Count > 0)
			{
				var type = definitionsWorklist.First().Value;
				definitionsWorklist.Remove(type.FullName);
				definitionsDone.Add(type.FullName);

				Assert(type.Attributes.HasFlag(TypeAttributes.WindowsRuntime));

				var typedef = definitionsList[type.FullName];
				switch (typedef.Kind)
				{
					case TypeKind.Enum:
						WriteEnum(typedef);
						break;
					case TypeKind.Interface:
						WriteInterface(typedef, false);
						break;
					case TypeKind.Struct:
						WriteStruct(typedef);
						break;
					case TypeKind.Delegate:
						WriteInterface(typedef, true);
						break;
					case TypeKind.Class:
						WriteClass(typedef);
						break;
				}
			}
		}

		public int GenerateParametricInstantiations()
		{
			return pinterfaceManager.Generate(this);
		}

		private void WriteEnum(TypeDef def)
		{
			var t = def.Type;
			var isFlags = t.CustomAttributes.Any(a => a.AttributeType.Name == "FlagsAttribute");
			var underlyingType = t.Fields.Single(f => f.Name == "value__").FieldType;
			string name = TypeHelpers.GetTypeName(this, def, t, TypeUsage.Define);
			def.Module.Append(@"
		RT_ENUM! { enum " + name + ": " + TypeHelpers.GetTypeName(this, def, underlyingType, TypeUsage.Raw) + @" {
			" + String.Join(", ", t.Fields.Where(f => f.Name != "value__").Select(f => NameHelpers.PreventKeywords(f.Name) + " (" + t.Name + "_" + f.Name + ") = " + f.Constant)) + @",
		}}");
		}

		private void WriteStruct(TypeDef def)
		{
			var t = def.Type;
			string name = TypeHelpers.GetTypeName(this, def, t, TypeUsage.Define);
			// TODO: derive(Eq) whenever possible?
			def.Module.Append(@"
		RT_STRUCT! { struct " + name + @" {
			" + String.Join(", ", t.Fields.Select(f => NameHelpers.PreventKeywords(f.Name) + ": " + TypeHelpers.GetTypeName(this, def, f.FieldType, TypeUsage.Raw))) + (t.Fields.Any() ? "," : "") + @"
		}}");
		}

		private void WriteInterface(TypeDef def, bool isDelegate)
		{
			var t = def.Type;
			var guid = t.CustomAttributes.First(a => a.AttributeType.Name == "GuidAttribute");
			var exclusiveTo = t.CustomAttributes.SingleOrDefault(a => a.AttributeType.Name == "ExclusiveToAttribute");
			TypeDefinition exclusiveToType = null;
			if (exclusiveTo != null)
			{
				Assert(exclusiveTo.ConstructorArguments[0].Type.FullName == "System.Type");
				exclusiveToType = exclusiveTo.ConstructorArguments[0].Value as TypeDefinition;
			}

			bool isFactoryOrStatic = TypeHelpers.IsFactoryOrStatic(this, t, exclusiveToType);

			var name = TypeHelpers.GetTypeName(this, def, t, TypeUsage.Define);

			def.Module.Append(@"
		DEFINE_IID!(IID_" + name + ", " + String.Join(", ", guid.ConstructorArguments.Select(a => a.Value)) + ");");

			string generic = "";
			string genericWithBounds = "";
			if (t.HasGenericParameters)
			{
				if (t.GenericParameters.Count > 2) throw new NotImplementedException("Not yet supported by RT_INTERFACE macro");
				generic = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name)) + ">";
				genericWithBounds = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name + ": RtType")) + ">";
			}

			var methods = def.Methods;
			string prependStatic = isFactoryOrStatic ? "static " : "";

			if (!isDelegate)
			{
				def.Module.Append(@"
		RT_INTERFACE!{" + prependStatic + "interface " + name + generic + "(" + name + "Vtbl): IInspectable(IInspectableVtbl) [IID_" + name + @"] {
			" + String.Join(",\r\n			", methods.Select(m => GetRawMethodDeclaration(m))) + @"
		}}");
			}
			else
			{
				def.Module.Append(@"
		RT_DELEGATE!{delegate " + name + generic + "(" + name + "Vtbl, " + name + "Impl) [IID_" + name + @"] {
			" + String.Join(",\r\n			", methods.Select(m => GetRawMethodDeclaration(m))) + @"
		}}");
			}

			def.Module.Append(@"
		impl" + genericWithBounds + " " + name + generic + @" {
			" + String.Join("\r\n			", methods.Select(m => GetMethodWrapperDefinition(m)).Where(m => m != null)) + @"
		}");
		}

		private string GetMethodWrapperDefinition(MethodDef m)
		{
			string rawName = m.GetRawName();
			string name = m.GetWrapperName(rawName);
			
			if (rawName == "GetMany" && m.DeclaringType.Namespace == "Windows.Foundation.Collections" &&
				(m.DeclaringType.Name == "IVectorView`1" || m.DeclaringType.Name == "IIterator`1" || m.DeclaringType.Name == "IVector`1"))
			{
				// This method has special semantics, since it takes an array and returns the number of elements that were filled
				// It uses the __RPC__out_ecount_part(capacity, *actual) annotation in the C headers
				// TODO ...
			}

			var input = new List<Tuple<string, TypeReference, InputKind>>();
			var output = new List<Tuple<string, TypeReference>>();

			foreach (var p in m.Method.Parameters)
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
							input.Add(new Tuple<string, TypeReference, InputKind>(pname + "Size", m.Method.Module.Import(typeof(uint)), InputKind.Default));
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

			if (m.Method.ReturnType.FullName != "System.Void")
			{
				// this makes the actual return value the last in the tuple (if multiple)
				output.Add(new Tuple<string, TypeReference>("out", m.Method.ReturnType));
			}

			string outType = String.Join(", ", output.Select(o => TypeHelpers.GetTypeName(this, m, o.Item2, TypeUsage.Out)));
			if (output.Count != 1)
			{
				outType = "(" + outType + ")"; // also works for count == 0 (empty tuple)
			}
			var inputParameters = new string[] { "&mut self" }.Concat(input.Select(i => i.Item1 + ": " + TypeHelpers.GetInputTypeName(this, m, i.Item2, i.Item3)));
			var rawParams = new List<string> { "self" };
			foreach (var p in m.Method.Parameters)
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

			if (m.Method.ReturnType.FullName != "System.Void")
			{
				if (m.Method.ReturnType.IsArray)
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

		private string GetRawMethodDeclaration(MethodDef m)
		{
			var name = m.GetRawName();
			return "fn " + name + "(" + String.Join(", ", GetMethodParameterDeclarations(m)) + ") -> ::w::HRESULT";
		}

		private IEnumerable<string> GetMethodParameterDeclarations(MethodDef m)
		{
			yield return "&mut self";
			foreach (var p in m.Method.Parameters)
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
				yield return NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name)) + ": " + TypeHelpers.GetTypeName(this, m, p.ParameterType, TypeUsage.Raw);
			}
			if (m.Method.ReturnType.FullName != "System.Void")
			{
				if (m.Method.ReturnType.IsArray)
				{
					yield return "outSize: *mut u32";
				}
				yield return "out: *mut " + TypeHelpers.GetTypeName(this, m, m.Method.ReturnType, TypeUsage.Raw);
			}
		}

		private void WriteClass(TypeDef def)
		{
			var t = def.Type;
			var factory = TypeHelpers.GetFactoryType(t);
			var statics = TypeHelpers.GetStaticTypes(t);

			if (t.Interfaces.Count == 0 && !statics.Any())
			{
				Console.WriteLine("WARNING: Skipping " + t + " (class with no interfaces or statics)");
				return;
			}
			var classType = TypeHelpers.GetTypeName(this, def, t, TypeUsage.Define);
			bool needClassID = false;
			if (t.Interfaces.Count > 0)
			{
				var mainInterface = t.Interfaces[0];
				var aliasedType = TypeHelpers.GetTypeName(this, def, mainInterface, TypeUsage.Alias);

				if (factory == null)
				{
					def.Module.Append(@"
		RT_CLASS!{class " + classType + ": " + aliasedType + "}");
				}
				else
				{
					needClassID = true;
					def.Module.Append(@"
		RT_CLASS!{class " + classType + ": " + aliasedType + " [" + TypeHelpers.GetTypeName(this, def, factory, TypeUsage.Alias) + "] [CLSID_" + classType + "]}");
				}
			}

			foreach (var staticType in statics)
			{
				var staticName = TypeHelpers.GetTypeName(this, def, staticType, TypeUsage.Define);
				needClassID = true;
				def.Module.Append(@"
		RT_ACTIVATABLE!{" + staticName + " [CLSID_" + classType + "]}");
			}

			if (needClassID)
			{
				def.Module.Append(@"
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
