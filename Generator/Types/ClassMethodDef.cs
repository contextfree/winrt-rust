using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator.Types
{
    /// <summary>
    /// Definition of a wrapped method in a class.
    /// Those are static methods that call get_activation_factory in their implementation.
    /// </summary>
    public class ClassMethodDef : ITypeRequestSource
    {
        public MethodDef WrappedMethod { get; private set; }
        public ClassDef ContainingClass { get; private set; }

        public string Name { get; private set; }

        public Module Module
        {
            get
            {
                return ContainingClass.Module;
            }
        }

        public Generator Generator
        {
            get
            {
                return WrappedMethod.DeclaringType.Generator;
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

        private string[] inputParameters;
        private string outType;

        public IEnumerable<TypeDef> ForeignAssemblyDependencies
        {
            get
            {
                return dependencies.Where(t => t.Module.Assembly != Module.Assembly && t.Module.Assembly.Name.Name != "Windows.Foundation");
            }
        }

        public ClassMethodDef(MethodDef method, ClassDef containingClass, bool isFactory)
        {
            WrappedMethod = method;
            ContainingClass = containingClass;
            Name = WrappedMethod.Details.WrappedName;

            AddDependency(method.DeclaringType);
            inputParameters = WrappedMethod.Details.MakeInputParameters(Generator, this);
            outType = WrappedMethod.Details.MakeOutType(Generator, this, isFactory);
        }

        public void FixupName(string suffix)
        {
            Name += suffix;
        }

        public string Emit()
        {
            var m = WrappedMethod;
            var dependsOnAssemblies = new List<string>(ForeignAssemblyDependencies.GroupBy(t => t.Module.Assembly.Name.Name).Select(g => g.Key));
            var features = new FeatureConditions(dependsOnAssemblies);

            return $@"{ features.GetAttribute() }#[inline] pub fn { Name }({ String.Join(", ", inputParameters) }) -> Result<{ outType }> {{
        <Self as RtActivatable<{ m.DeclaringType.Name }>>::get_activation_factory().deref().{ m.Details.WrappedName }({ String.Join(", ", m.Details.InputParameterNames) })
    }}";
        }

        public void AddDependency(TypeDef other)
        {
            Assert(Generator.AllowAddDependencies);
            dependencies.Add(other);
        }
    }
}
