using System;
using System.Collections.Generic;
using System.Linq;

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
				var mainInterface = Type.Interfaces[0];
				aliasedType = TypeHelpers.GetTypeName(Generator, this, mainInterface, TypeUsage.Alias);
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

				Module.Append(@"
		" + features.GetAttribute() + "RT_CLASS!{class " + DefinitionName + ": " + aliasedType + "}");
				if (!features.IsEmpty)
				{
					// if the aliased type is from a different assembly that is not included, just use IInspectable instead
					Module.Append(@"
		" + features.GetInvertedAttribute() + "RT_CLASS!{class " + DefinitionName + ": IInspectable}");
				}

				foreach (var factory in factories)
				{
					needClassID = true;
					Module.Append(@"
		impl ::RtActivatable<" + factory + "> for " + classType + " {}");
				}
			}
			else
			{
				Module.Append(@"
		RT_CLASS!{static class " + DefinitionName + "}");
			}

			foreach (var staticType in statics)
			{
				var staticName = Generator.GetTypeDefinition(staticType).DefinitionName;
				needClassID = true;
				Module.Append(@"
		impl ::RtActivatable<" + staticName + "> for " + classType + " {}");
			}

			if (IsDefaultActivatable())
			{
				needClassID = true;
				Module.Append(@"
		impl ::RtActivatable<IActivationFactory> for " + classType + " {}");
			}

			if (needClassID)
			{
				Module.Append(@"
		DEFINE_CLSID!(" + classType + "(&[" + NameHelpers.StringToUTF16WithZero(Type.FullName) + @"]) [CLSID_" + classType + "]);");
			}
		}
	}
}
