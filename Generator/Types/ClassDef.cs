using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator.Types
{
    public class ClassDef : TypeDef
    {
        public override TypeKind Kind
        {
            get
            {
                return TypeKind.Class;
            }
        }

        public override bool CanBeSkipped
        {
            get
            {
                return Type.Interfaces.Count == 0 && !statics.Any();
            }
        }

        private TypeDefinition[] statics;
        private string aliasedType;
        private string[] factories;
        private List<ClassMethodDef> methodWrappers = new List<ClassMethodDef>();

        public ClassDef(TypeDefinition t) : base(t)
        {
            // this is required early to know whether the definition can be skipped
            statics = GetStaticTypes().ToArray();
        }

        public override void CollectDependencies()
        {
            foreach (var staticType in statics)
            {
                AddDependency(Generator.GetTypeDefinition(staticType));
            }

            factories = GetFactoryTypes().Select(f => TypeHelpers.GetTypeName(Generator, this, f, TypeUsage.Alias)).ToArray();
            
            if (Type.Interfaces.Count > 0)
            {
                var defaultInterface = TypeHelpers.GetDefaultInterface(Type);
                aliasedType = TypeHelpers.GetTypeName(Generator, this, defaultInterface, TypeUsage.Alias);
            }

            var factoryMethods = GetFactoryTypes().SelectMany(f => Generator.GetTypeDefinition(f).Methods).ToArray();
            var staticMethods = statics.SelectMany(s => Generator.GetTypeDefinition(s).Methods).ToArray();
            
            foreach (var m in factoryMethods)
            {
                methodWrappers.Add(new ClassMethodDef(m, this));
            }
            foreach (var m in staticMethods)
            {
                methodWrappers.Add(new ClassMethodDef(m, this));
            }

            // fix name clashes in method wrappers (caused by overloads from different interfaces)
            foreach (var nameToFix in methodWrappers.GroupBy(m => m.Name).Where(g => g.Count() > 1))
            {
                // we expect that there is a single base name (modulo suffix numbers) for all clashing interfaces
                string baseName = nameToFix.Select(n => n.WrappedMethod.DeclaringType.Name.TrimEnd('1', '2', '3', '4', '5', '6', '7', '8', '9', '0')).Distinct().Single();
                foreach (var m in nameToFix)
                {
                    // so far this is always "2"
                    var nameSuffix = m.WrappedMethod.DeclaringType.Name.Replace(baseName, "");
                    m.FixupName(nameSuffix);
                }
            }
        }

        public override void Emit()
        {
            var classType = DefinitionName;
            bool needClassID = false;

            if (Type.Interfaces.Count > 0)
            {
                var dependsOnAssemblies = new List<string>(ForeignAssemblyDependencies.GroupBy(t => t.Module.Assembly.Name.Name).Select(g => g.Key));
                var features = new FeatureConditions(dependsOnAssemblies);

                Module.Append($@"
{ features.GetAttribute() }RT_CLASS!{{class { DefinitionName }: { aliasedType }}}");
                if (!features.IsEmpty)
                {
                    // if the aliased type is from a different assembly that is not included, just use IInspectable instead
                    // otherwise types depending on this class would transitively depend on the aliased type
                    Module.Append($@"
{ features.GetInvertedAttribute() }RT_CLASS!{{class { DefinitionName }: IInspectable}}");
                }

                foreach (var factory in factories)
                {
                    needClassID = true;
                    Module.Append($@"
impl RtActivatable<{ factory }> for { classType } {{}}");
                }
            }
            else
            {
                Assert(!factories.Any());
                Module.Append($@"
RT_CLASS!{{static class { DefinitionName }}}");
            }

            foreach (var staticType in statics)
            {
                var staticName = Generator.GetTypeDefinition(staticType).DefinitionName;
                needClassID = true;
                Module.Append($@"
impl RtActivatable<{ staticName }> for { classType } {{}}");
            }

            if (IsDefaultActivatable())
            {
                needClassID = true;
                Module.Append($@"
impl RtActivatable<IActivationFactory> for { classType } {{}}");
            }

            if (methodWrappers.Any())
            {
                Module.Append($@"
impl { DefinitionName } {{
    { String.Join("\r\n    ", methodWrappers.Select(m => m.Emit())) }
}}");
            }

            if (needClassID)
            {
                Module.Append($@"
DEFINE_CLSID!({ classType }(&[{ NameHelpers.StringToUTF16WithZero(Type.FullName) }]) [CLSID_{ classType }]);");
            }
        }
    }
}
