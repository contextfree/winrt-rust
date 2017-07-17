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
            foreach (var type in definitionsList.Values)
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

        /// <summary>
        /// Write output (all modules) to a single file. This is not used anymore
        /// </summary>
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

            const string IMPORTS = @"use ::prelude::*;";

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

        /// <summary>
        /// Write one output file per assembly.
        /// </summary>
        public void WriteModuleTreeMultiFile(DirectoryInfo directory)
        {
            directory.Create();
            using (var file = new StreamWriter(Path.Combine(directory.FullName, "mod.rs")))
            {
                file.WriteLine("// DO NOT MODIFY THIS MODULE NOR ANY OF ITS CHILDREN - THEY ARE AUTOMATICALLY GENERATED!");
                file.WriteLine(@"#![allow(non_camel_case_types, unused_imports)]");
                foreach (var child in rootModule.Children.Values)
                {
                    WriteModuleTreeMultiFile(child, directory, file);
                }
            }
        }

        private void WriteModuleTreeMultiFile(Module mod, DirectoryInfo directory, StreamWriter file, string path = null, AssemblyDefinition asm = null)
        {
            if (mod.IsEmpty) return;

            const string IMPORTS = @"use ::prelude::*;";

            string name = mod.Name.ToLower();
            string newPath = path == null ? mod.Name : (path + "." + mod.Name);

            bool isNewAssembly = (asm != mod.Assembly);

            if (isNewAssembly && mod.Assembly.Name.Name != "Windows.Foundation")
            {
                var featureCond = new FeatureConditions(new string[] { mod.Assembly.Name.Name }).GetAttribute().TrimEnd();
                file.Write(featureCond);
                file.Write(" ");
            }

            if (isNewAssembly || path == null || mod.ContainsMoreThanOneAssembly)
            {
                // write module to separate file
                file.WriteLine("pub mod " + name + "; // " + newPath);
                DirectoryInfo childDir;
                StreamWriter childFile;
                if (mod.ContainsMoreThanOneAssembly)
                {
                    childDir = directory.CreateSubdirectory(name);
                    childFile = new StreamWriter(Path.Combine(childDir.FullName, "mod.rs"));
                }
                else
                {
                    childDir = directory;
                    childFile = new StreamWriter(Path.Combine(childDir.FullName, name + ".rs"));
                }

                var text = mod.Text.ToString();
                if (!string.IsNullOrWhiteSpace(text))
                {
                    childFile.Write(IMPORTS);
                    childFile.WriteLine(text);
                }
                foreach (var child in mod.Children.Values)
                {
                    WriteModuleTreeMultiFile(child, childDir, childFile, newPath, mod.Assembly);
                }

                childFile.Close();
            }
            else
            {
                // write module inline
                file.WriteLine("pub mod " + name + " { // " + newPath);
                var text = mod.Text.ToString();
                if (!string.IsNullOrWhiteSpace(text))
                {
                    file.Write(IMPORTS);
                    file.WriteLine(text);
                }
                foreach (var child in mod.Children.Values)
                {
                    WriteModuleTreeMultiFile(child, directory, file, newPath, mod.Assembly);
                }
                file.WriteLine("} // " + newPath);
            }
        }
    }
}
