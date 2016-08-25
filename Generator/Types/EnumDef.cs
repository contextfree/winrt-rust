using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;

using Mono.Cecil;

namespace Generator.Types
{
	public class EnumDef : TypeDef
	{
		public override TypeKind Kind
		{
			get
			{
				return TypeKind.Enum;
			}
		}

		public string UnderlyingTypeName { get; private set; }

		public EnumDef(TypeDefinition t) : base(t) { }

		public override void CollectDependencies()
		{
			//var isFlags = Type.CustomAttributes.Any(a => a.AttributeType.Name == "FlagsAttribute");
			var underlyingType = Type.Fields.Single(f => f.Name == "value__").FieldType;
			UnderlyingTypeName = TypeHelpers.GetTypeName(Generator, this, underlyingType, TypeUsage.Raw);
		}

		public override void Emit()
		{
			Module.Append(@"
		RT_ENUM! { enum " + DefinitionName + ": " + UnderlyingTypeName + @" {
			" + String.Join(", ", Type.Fields.Where(f => f.Name != "value__").Select(f => NameHelpers.PreventKeywords(f.Name) + " (" + Type.Name + "_" + f.Name + ") = " + f.Constant)) + @",
		}}");
		}
	}
}
