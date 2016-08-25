using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

using Generator.Types;

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
					definitionsList.Add(t.FullName, TypeDef.Create(this, t, asm));
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

		public void CollectDependencies()
		{
			foreach (var typedef in definitionsList.Values)
			{
				if (!typedef.CanBeSkipped)
					typedef.CollectDependencies();
			}
		}

		public void EmitTypes()
		{
			while (definitionsWorklist.Count > 0)
			{
				var type = definitionsWorklist.First().Value;
				definitionsWorklist.Remove(type.FullName);
				definitionsDone.Add(type.FullName);

				Assert(type.Attributes.HasFlag(TypeAttributes.WindowsRuntime));

				var typedef = definitionsList[type.FullName];
				if (!typedef.CanBeSkipped)
					typedef.Emit();
			}
		}

		public int EmitParametricInstantiations()
		{
			return pinterfaceManager.Generate(this);
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
