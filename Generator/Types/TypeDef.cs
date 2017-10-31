using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;
using Mono.Cecil.Rocks;

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
            Assert(Generator.AllowAddDependencies);
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

            var other = (TypeDef)obj;
            return this.Type.FullName.Equals(other.Type.FullName) && this.Module.Assembly.FullName.Equals(other.Module.Assembly.FullName);
        }

        public override string ToString()
        {
            return this.Type.FullName;
        }

        public abstract void CollectDependencies();

        public abstract void Emit();

        public IEnumerable<TypeDefinition> GetFactoryTypes()
        {
            return Type.CustomAttributes
                .Where(a => a.AttributeType.Name == "ActivatableAttribute" && a.ConstructorArguments[0].Type.FullName == "System.Type")
                .Select(a => a.ConstructorArguments[0].Value as TypeDefinition);
        }

        public bool IsDefaultActivatable()
        {
            var count = Type.CustomAttributes.Count(a => a.AttributeType.Name == "ActivatableAttribute" && a.ConstructorArguments[0].Type.FullName != "System.Type");
            Assert(count <= 1); // make sure that our attribute filtering logic makes sense
            return count != 0;

        }

        public IEnumerable<TypeDefinition> GetStaticTypes()
        {
            return Type.CustomAttributes.Where(a => a.AttributeType.Name == "StaticAttribute").Select(a => (TypeDefinition)a.ConstructorArguments[0].Value);
        }

        /// <summary>
        /// Returns the GUID of this type, if one was defined with the `Guid` attribute in winmd.
        /// </summary>
        public Guid? GetGuid()
        {
            var guidAttr = Type.CustomAttributes.FirstOrDefault(attr => attr.AttributeType.Name == "GuidAttribute");
            Guid? guid = null;
            if (guidAttr != null)
            {
                var a = guidAttr.ConstructorArguments;
                guid = new Guid((uint)a[0].Value, (ushort)a[1].Value, (ushort)a[2].Value, (byte)a[3].Value, (byte)a[4].Value,
                    (byte)a[5].Value, (byte)a[6].Value, (byte)a[7].Value, (byte)a[8].Value, (byte)a[9].Value, (byte)a[10].Value);
            }

            return guid;
        }

        /// <summary>
        /// Gets the path of this type definition, relative to a module in which it should be named.
        /// Might also be an absolute path, if that is shorter than a relative one.
        /// </summary>
        public string GetPath(Module requestingModule)
        {
            string name = null;
            if (Module == requestingModule)
            {
                name = Name;
            }
            else
            {
                name = $"::rt::gen::{ Module.Path }::{ Name }";
                var relative = $"{ Module.GetRelativePath(requestingModule) }::{ Name }";
                if (relative.Length < name.Length)
                    name = relative;
            }
            return name;
        }

        public GenericInstanceType MakeGenericInstanceType(params TypeReference[] arguments)
        {
            return Type.MakeGenericInstanceType(arguments);
        }
    }
}
