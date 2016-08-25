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

		private List<TypeDefinition> statics;
		private string aliasedType;
		private string factory;

		public ClassDef(TypeDefinition t) : base(t)
		{
			// this is required early to know whether the definition can be skipped
			statics = GetStaticTypes().ToList(); 
		}

		public override void CollectDependencies()
		{
			foreach (var staticType in statics)
			{
				AddDependency(Generator.GetTypeDefinition(staticType));
			}

			var factoryType = GetFactoryType();
			if (factoryType != null)
			{
				factory = TypeHelpers.GetTypeName(Generator, this, factoryType, TypeUsage.Alias);
			}
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

				if (string.IsNullOrEmpty(factory))
				{
					Module.Append(@"
		" + features.GetAttribute() + "RT_CLASS!{class " + DefinitionName + ": " + aliasedType + "}");
					if (!features.IsEmpty)
					{
						// if the aliased type is from a different assembly, just use IInspectable instead
						Module.Append(@"
		" + features.GetInvertedAttribute() + "RT_CLASS!{class " + DefinitionName + ": IInspectable}");
					}
				}
				else
				{
					if (!features.IsEmpty)
					{
						throw new NotImplementedException("This case is currently not supported.");
					}
					needClassID = true;
					Module.Append(@"
		RT_CLASS!{class " + DefinitionName + ": " + aliasedType + " [" + factory + "] [CLSID_" + classType + "]}");
				}
			}

			foreach (var staticType in statics)
			{
				var staticName = Generator.GetTypeDefinition(staticType).DefinitionName;
				needClassID = true;
				Module.Append(@"
		RT_ACTIVATABLE!{" + staticName + " [CLSID_" + classType + "]}");
			}

			if (needClassID)
			{
				Module.Append(@"
		DEFINE_CLSID!(CLSID_" + classType + " = &[" + NameHelpers.StringToUTF16WithZero(Type.FullName) + @"]);");
			}
		}
	}
}
