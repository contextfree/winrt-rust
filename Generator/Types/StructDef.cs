using System;
using System.Collections.Generic;
using System.Linq;

using Mono.Cecil;

namespace Generator.Types
{
	public class StructDef : TypeDef
	{
		public override TypeKind Kind
		{
			get
			{
				return TypeKind.Struct;
			}
		}

		private List<string> fields;

		public StructDef(TypeDefinition t) : base(t) { }

		public override void CollectDependencies()
		{
			fields = Type.Fields.Select(f => NameHelpers.PreventKeywords(f.Name) + ": " + TypeHelpers.GetTypeName(Generator, this, f.FieldType, TypeUsage.Raw)).ToList();
		}

		public override void Emit()
		{
			// TODO: derive(Eq) whenever possible?
			Module.Append(@"
RT_STRUCT! { struct " + DefinitionName + @" {
	" + string.Join(", ", fields) + (fields.Any() ? "," : "") + @"
}}");
		}

		
	}
}
