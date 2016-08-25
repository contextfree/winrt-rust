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

			isFactoryOrStatic = TypeHelpers.IsFactoryOrStatic(Generator, Type, exclusiveToType);

			rawMethodDeclarations = methods.Select(m => m.GetRawDeclaration()).ToList();
			wrapperMethodDeclarations = methods.Select(m => m.GetWrapperDefinition()).Where(m => m != null).ToList();
		}

		public override void Emit()
		{
			var t = Type;
			var guid = t.CustomAttributes.First(a => a.AttributeType.Name == "GuidAttribute");

			var name = DefinitionName;

			Module.Append(@"
		DEFINE_IID!(IID_" + name + ", " + String.Join(", ", guid.ConstructorArguments.Select(a => a.Value)) + ");");

			string generic = "";
			string genericWithBounds = "";
			if (t.HasGenericParameters)
			{
				if (t.GenericParameters.Count > 2) throw new NotImplementedException("Not yet supported by RT_INTERFACE macro");
				generic = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name)) + ">";
				genericWithBounds = "<" + String.Join(", ", t.GenericParameters.Select(p => p.Name + ": RtType")) + ">";
			}

			string prependStatic = isFactoryOrStatic ? "static " : "";

			if (!IsDelegate)
			{
				Module.Append(@"
		RT_INTERFACE!{" + prependStatic + "interface " + name + generic + "(" + name + "Vtbl): IInspectable(IInspectableVtbl) [IID_" + name + @"] {
			" + String.Join(",\r\n			", rawMethodDeclarations) + @"
		}}");
			}
			else
			{
				Module.Append(@"
		RT_DELEGATE!{delegate " + name + generic + "(" + name + "Vtbl, " + name + "Impl) [IID_" + name + @"] {
			" + String.Join(",\r\n			", rawMethodDeclarations) + @"
		}}");
			}

			Module.Append(@"
		impl" + genericWithBounds + " " + name + generic + @" {
			" + String.Join("\r\n			", wrapperMethodDeclarations) + @"
		}");
		}
	}
}
