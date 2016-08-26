using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

using Mono.Cecil;

using Generator.Types;

namespace Generator
{
	public class Generator
	{
		private Dictionary<string, TypeDef> definitionsList = new Dictionary<string, TypeDef>();

		private ParametricInterfaceManager pinterfaceManager = new ParametricInterfaceManager();

		public IEnumerable<TypeReference> BaseTypes { get; private set; }

		public AssemblyCollection Assemblies { get; private set; }

		private Module rootModule = new Module(null, "");

		public bool AllowAddDependencies { get; private set; }

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
			AllowAddDependencies = true;

			foreach (var asm in Assemblies.Assemblies)
			{
				foreach (var t in asm.MainModule.Types.Where(t => t.FullName != "<Module>"))
				{
					var typedef = TypeDef.Create(this, t, asm);
					if (!typedef.CanBeSkipped)
					{
						definitionsList.Add(t.FullName, typedef);
					}
				}
			}

			pinterfaceManager.AddBaseIReferenceInstances(this);
		}

		public void AddGenericInstance(GenericInstanceType ty)
		{
			pinterfaceManager.AddGenericInstance(ty);
		}

		public Module GetModule(string name)
		{
			return rootModule.FindChild(name);
		}

		public TypeDef GetTypeDefinition(TypeReference t)
		{
			if (t.IsGenericInstance)
			{
				t = ((GenericInstanceType)t).ElementType;
			}
			return definitionsList[t.FullName];
		}

		public TypeDef GetTypeDefinition(string fullName)
		{
			return definitionsList[fullName];
		}

		public bool TryGetTypeDefinition(string fullName, out TypeDef value)
		{
			return definitionsList.TryGetValue(fullName, out value);
		}

		public void CollectDependencies()
		{
			foreach (var typedef in definitionsList.Values)
			{
				typedef.CollectDependencies();
			}

			AllowAddDependencies = false; // this prevents logic bugs where new dependencies are added after this phase
		}

		public void EmitTypes()
		{
			// TODO: get rid of this and enable all modules (disabled because it results in a huge file and very long compilation)
			var assemblyNames = new string[] { "Windows.Foundation", "Windows.Devices"/*, "Windows.Storage", "Windows.Media", "Windows.System", "Windows.Graphics"*/ };

			foreach (var type in definitionsList.Values.Where(t => assemblyNames.Contains(t.Module.Assembly.Name.Name)))
			{
				type.Emit();
			}
		}

		public int EmitParametricInstances()
		{
			var instances = pinterfaceManager.Collect(this).ToList();
			instances.Sort((x, y) => x.Name.CompareTo(y.Name));
			foreach (var inst in instances)
			{
				inst.Emit();
			}
			return instances.Count;
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

		private void WriteModuleTree(Module mod, StreamWriter file, string path = null, AssemblyDefinition asm = null)
		{
			if (mod.IsEmpty) return;

			const string IMPORTS = @"use ::{ComInterface, HString, HStringArg, ComPtr, ComArray, ComIid, IUnknown};
use ::rt::{RtType, IInspectable, RtResult}; use ::rt::handler::IntoInterface;";

			string name = mod.Name.ToLower();
			string newPath = path == null ? mod.Name : (path + "." + mod.Name);
			if (asm != mod.Assembly && mod.Assembly.Name.Name != "Windows.Foundation")
			{
				file.WriteLine(new FeatureConditions(new string[] { mod.Assembly.Name.Name }).GetAttribute().TrimEnd());
			}
			file.WriteLine("pub mod " + name + " { // " + newPath);
			var text = mod.Text.ToString();
			if (!string.IsNullOrWhiteSpace(text))
			{
				file.Write(IMPORTS);
				file.WriteLine(text);
			}
			foreach (var child in mod.Children.Values)
			{
				WriteModuleTree(child, file, newPath, mod.Assembly);
			}
			file.WriteLine("} // " + newPath);
		}
	}
}
