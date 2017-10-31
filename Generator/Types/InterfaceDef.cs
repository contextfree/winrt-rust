using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator.Types
{
    public class InterfaceDef : TypeDef
    {
        public override TypeKind Kind
        {
            get
            {
                return IsDelegate ? TypeKind.Delegate : TypeKind.Interface;
            }
        }

        public bool IsDelegate { get; private set; }

        private bool isFactoryOrStatic;
        private List<MethodDef> methods;
        public override IEnumerable<MethodDef> Methods
        {
            get
            {
                return methods;
            }
        }
        private List<string> rawMethodDeclarations;
        private List<string> wrapperMethodDeclarations;
        

        public InterfaceDef(TypeDefinition t) : base(t)
        {
            IsDelegate = t.IsClass && t.BaseType.FullName == "System.MulticastDelegate";

            methods = t.Methods.Where(m => m.Name != ".ctor").Select(m => new MethodDef(this, m)).ToList();
        }

        public override void CollectDependencies()
        {
            var exclusiveTo = Type.CustomAttributes.SingleOrDefault(a => a.AttributeType.Name == "ExclusiveToAttribute");
            TypeDefinition exclusiveToType = null;
            if (exclusiveTo != null)
            {
                Assert(exclusiveTo.ConstructorArguments[0].Type.FullName == "System.Type");
                exclusiveToType = exclusiveTo.ConstructorArguments[0].Value as TypeDefinition;
            }

            isFactoryOrStatic = IsFactoryOrStatic(Generator, this, exclusiveToType);

            rawMethodDeclarations = methods.Select(m => m.GetRawDeclaration()).ToList();
            wrapperMethodDeclarations = methods.Select(m => m.GetWrapperDefinition()).ToList();
        }

        public override void Emit()
        {
            var t = Type;
            var guid = t.CustomAttributes.First(a => a.AttributeType.Name == "GuidAttribute");

            var definitionFeatureConditions = methods.Select(m => m.GetFeatureCondition()).ToList();

            var name = DefinitionName;

            Module.Append($@"
DEFINE_IID!(IID_{ name }, { String.Join(", ", guid.ConstructorArguments.Select(a => a.Value)) });");

            string generic = "";
            string genericWithBounds = "";
            if (t.HasGenericParameters)
            {
                if (t.GenericParameters.Count > 2) throw new NotImplementedException("Not yet supported by RT_INTERFACE macro");
                generic = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name)) + ">";
                genericWithBounds = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name + ": RtType")) + ">";
            }

            string prependStatic = isFactoryOrStatic ? "static " : "";

            var rawMethodDeclarationsWithFeatures = new List<string>();
            var lastFeatureAttr = definitionFeatureConditions.LastOrDefault()?.GetAttribute();

            // walk through method declaration backwards in order to emit dummy definitions in between valid ones
            for (int i = rawMethodDeclarations.Count - 1; i >= 0; i--)
            {
                var feature = definitionFeatureConditions[i];
                var featureAttr = feature.GetAttribute();
                var decl = rawMethodDeclarations[i];
                if (feature.IsEmpty || featureAttr == lastFeatureAttr)
                {
                    rawMethodDeclarationsWithFeatures.Insert(0, featureAttr + decl);
                }
                else
                {
                    rawMethodDeclarationsWithFeatures.Insert(0, featureAttr + decl);
                    rawMethodDeclarationsWithFeatures.Insert(0, feature.GetInvertedAttribute() + "fn __Dummy" + i + "(&self) -> ()");
                }
            }

            if (!IsDelegate)
            {
                Module.Append($@"
RT_INTERFACE!{{{ prependStatic }interface { name }{ generic }({ name }Vtbl): IInspectable(IInspectableVtbl) [IID_{ name }] {{
    { String.Join(",\r\n    ", rawMethodDeclarationsWithFeatures) }
}}}}");
            }
            else
            {
                Module.Append($@"
RT_DELEGATE!{{delegate { name }{ generic }({ name }Vtbl, { name }Impl) [IID_{ name }] {{
    { String.Join(",\r\n    ", rawMethodDeclarationsWithFeatures) }
}}}}");
            }

            if (wrapperMethodDeclarations.Any())
            {
                Module.Append($@"
impl{ genericWithBounds } { name }{ generic } {{
    { String.Join("\r\n    ", wrapperMethodDeclarations.Zip(definitionFeatureConditions, (wrapper, feature) => feature.GetAttribute() + wrapper)) }
}}");
            }
        }

        private bool IsFactoryOrStatic(Generator gen, TypeDef t, TypeDefinition exclusiveToType)
        {
            var trimmedName = t.Name.TrimEnd('1', '2', '3', '4', '5', '6', '7', '8', '9');
            var guessedFromName = trimmedName.EndsWith("Factory") || trimmedName.EndsWith("Statics");
            var candidates = new HashSet<TypeDef>();
            if (exclusiveToType != null)
            {
                candidates.Add(gen.GetTypeDefinition(exclusiveToType));
            }

            if (guessedFromName)
            {
                var targetTypeName = trimmedName.Substring(0, trimmedName.Length - 7); // "Factory" and "Statics" both have length 7
                TypeDef resolved;
                if (gen.TryGetTypeDefinition(t.Namespace + "." + targetTypeName, out resolved))
                {
                    candidates.Add(resolved);
                }
                if (targetTypeName.StartsWith("I"))
                {
                    if (gen.TryGetTypeDefinition(t.Namespace + "." + targetTypeName.Substring(1), out resolved))
                    {
                        candidates.Add(resolved);
                    }
                }
            }

            return candidates.Any(c => c.GetFactoryTypes().Contains(t.Type) || c.GetStaticTypes().Contains(t.Type));
        }
    }
}
