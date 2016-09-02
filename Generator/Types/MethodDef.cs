using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator.Types
{
	public class MethodDef : ITypeRequestSource
	{
		public TypeDef DeclaringType { get; private set; }
		public MethodDefinition Method { get; private set; }

		public Module Module
		{
			get
			{
				return DeclaringType.Module;
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

		public IEnumerable<TypeDef> ForeignAssemblyDependencies
		{
			get
			{
				return dependencies.Where(t => t.Module.Assembly != this.DeclaringType.Module.Assembly && t.Module.Assembly.Name.Name != "Windows.Foundation");
			}
		}

		public MethodDef(TypeDef type, MethodDefinition method)
		{
			DeclaringType = type;
			Method = method;
		}

		public override string ToString()
		{
			return Method.Name;
		}

		public void AddDependency(TypeDef other)
		{
			Assert(DeclaringType.Generator.AllowAddDependencies);
			dependencies.Add(other);
			DeclaringType.AddDependency(other);
		}

		public string GetRawName()
		{
			var rawName = Method.Name;
			var overload = Method.CustomAttributes.FirstOrDefault(a => a.AttributeType.Name == "OverloadAttribute");
			if (overload != null)
			{
				rawName = (string)overload.ConstructorArguments[0].Value;
			}
			return rawName;
		}

		public string GetWrapperName(string rawName)
		{
			string name = NameHelpers.PreventKeywords(NameHelpers.CamelToSnakeCase(rawName.Replace("put_", "set_")));
			if (rawName.Contains("_")) // name already contains '_' -> might result in a name clash after renaming
			{
				if (DeclaringType.Methods.Select(mm => new Tuple<MethodDef, string>(mm, mm.GetRawName())).Where(mm => !mm.Item2.Contains("_")).Any(mm => mm.Item1.GetWrapperName(mm.Item2) == name))
				{
					name += "_";
				}
			}
			return name;
		}

		public string GetWrapperDefinition()
		{
			string rawName = GetRawName();
			string name = GetWrapperName(rawName);

			bool isGetMany = (rawName == "GetMany" && DeclaringType.Namespace == "Windows.Foundation.Collections" &&
							 (DeclaringType.Name == "IVectorView`1" || DeclaringType.Name == "IIterator`1" || DeclaringType.Name == "IVector`1"));
			string getManyPname = null;

			// These `GetMany` methods have special semantics, since it takes an array and returns the number of elements that were filled
			// It uses the __RPC__out_ecount_part(capacity, *actual) annotation in the C headers. For the wrapper we use a &mut Vec<> buffer.

			var input = new List<Tuple<string, TypeReference, InputKind>>();
			var output = new List<Tuple<string, TypeReference>>();

			foreach (var p in Method.Parameters)
			{
				string pname = NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name));
				if (p.ParameterType.IsByReference)
				{
					Assert(p.IsOut);
					var realType = ((ByReferenceType)p.ParameterType).ElementType;
					output.Add(new Tuple<string, TypeReference>(pname, realType));
				}
				else
				{
					if (p.ParameterType.IsArray)
					{
						if (p.IsOut)
						{
							if (isGetMany)
							{
								Assert(getManyPname == null); // there should only be one out-array parameter for GetMany
								getManyPname = pname;
								input.Add(new Tuple<string, TypeReference, InputKind>(pname, ((ArrayType)p.ParameterType).ElementType, InputKind.VecBuffer));
							}
							else
							{
								//TODO: this should probably be a mutable, write-only, empty slice -> what type?
								input.Add(new Tuple<string, TypeReference, InputKind>(pname + "Size", Method.Module.Import(typeof(uint)), InputKind.Default));
								input.Add(new Tuple<string, TypeReference, InputKind>(pname, p.ParameterType, InputKind.Raw));
							}
						}
						else
						{
							input.Add(new Tuple<string, TypeReference, InputKind>(pname, ((ArrayType)p.ParameterType).ElementType, InputKind.Slice));
						}
					}
					else
					{
						input.Add(new Tuple<string, TypeReference, InputKind>(pname, p.ParameterType, InputKind.Default));
					}
				}
			}

			if (Method.ReturnType.FullName != "System.Void")
			{
				// this makes the actual return value the last in the tuple (if multiple)
				output.Add(new Tuple<string, TypeReference>("out", Method.ReturnType));
			}

			string outType = String.Join(", ", output.Select(o => TypeHelpers.GetTypeName(DeclaringType.Generator, this, o.Item2, TypeUsage.Out)));
			if (output.Count != 1)
			{
				outType = "(" + outType + ")"; // also works for count == 0 (empty tuple)
			}
			var inputParameters = new string[] { "&mut self" }.Concat(input.Select(i => i.Item1 + ": " + TypeHelpers.GetInputTypeName(DeclaringType.Generator, this, i.Item2, i.Item3)));
			var rawParams = new List<string> { "self" };
			foreach (var p in Method.Parameters)
			{
				var pname = NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name));
				if (p.ParameterType.IsByReference)
				{
					if (((ByReferenceType)p.ParameterType).ElementType.IsArray)
					{
						rawParams.Add("&mut " + pname + "Size");
					}

					// output parameter
					rawParams.Add("&mut " + pname);
				}
				else
				{
					// input parameter
					if (p.ParameterType.IsArray)
					{
						if (p.IsOut)
						{
							if (isGetMany)
							{
								rawParams.Add(pname + ".capacity() as u32");
								rawParams.Add(pname + ".as_mut_ptr() as *mut T::Abi");
							}
							else
							{
								//TODO: this should probably be a mutable, write-only, empty slice -> what type?
								rawParams.Add(pname + "Size");
								rawParams.Add(TypeHelpers.UnwrapInputParameter(pname, p.ParameterType));
							}
						}
						else
						{
							rawParams.Add(pname + ".len() as u32");
							rawParams.Add(pname + ".as_ptr() as *mut _");
						}
					}
					else
					{
						rawParams.Add(TypeHelpers.UnwrapInputParameter(pname, p.ParameterType));
					}
				}
			}

			if (Method.ReturnType.FullName != "System.Void")
			{
				if (Method.ReturnType.IsArray)
				{
					rawParams.Add("&mut outSize");
				}
				rawParams.Add("&mut out");
			}

			var outInit = String.Join(" ", output.SelectMany(o => TypeHelpers.CreateUninitializedOutputs(o.Item1, o.Item2)));
			if (outInit != "") outInit = "\r\n\t\t\t\t" + outInit;

			var outWrap = String.Join(", ", output.Select(o => TypeHelpers.WrapOutputParameter(o.Item1, o.Item2)));
			if (output.Count != 1)
			{
				outWrap = "(" + outWrap + ")"; // also works for count == 0 (empty tuple)
			}
			outWrap = "Ok(" + outWrap + ")";

			if (isGetMany)
			{
				outType = "()";
				outInit = "\r\n\t\t\t\tdebug_assert!(" + getManyPname + ".capacity() > 0, \"capacity of `" + getManyPname + "` must not be 0 (use Vec::with_capacity)\"); " + getManyPname + ".clear();" + outInit;
				outWrap = getManyPname + ".set_len(out as usize); Ok(())";
			}

			string inline = isGetMany ? "" : "#[inline] ";

			return inline + "pub unsafe fn " + name + "(" + String.Join(", ", inputParameters) + ") -> RtResult<" + outType + @"> {" + outInit + @"
				let hr = ((*self.lpVtbl)." + rawName + ")(" + String.Join(", ", rawParams) + ");" + @"
				if hr == ::w::S_OK { " + outWrap + @" } else { Err(hr) }
			}";
		}

		public string GetRawDeclaration()
		{
			var name = GetRawName();
			return "fn " + name + "(" + String.Join(", ", GetParameterDeclarations()) + ") -> ::w::HRESULT";
		}

		public IEnumerable<string> GetParameterDeclarations()
		{
			yield return "&mut self";
			foreach (var p in Method.Parameters)
			{
				Assert(!p.IsReturnValue);
				int? lengthIs = null;
				var lengthIsAttribute = p.CustomAttributes.SingleOrDefault(a => a.AttributeType.Name == "LengthIsAttribute");
				if (lengthIsAttribute != null)
				{
					lengthIs = (int)lengthIsAttribute.ConstructorArguments[0].Value;
				}

				if (p.ParameterType.IsArray)
				{
					// need additional input size parameter (even if parameter is marked as [Out])
					yield return NameHelpers.FirstToLower(p.Name) + "Size: u32";
				}
				else if (p.ParameterType.IsByReference && (p.ParameterType as ByReferenceType).ElementType.IsArray)
				{
					Assert(!lengthIs.HasValue);
					// need additional output size parameter
					yield return NameHelpers.FirstToLower(p.Name) + "Size: *mut u32";
				}
				yield return NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name)) + ": " + TypeHelpers.GetTypeName(DeclaringType.Generator, this, p.ParameterType, TypeUsage.Raw);
			}
			if (Method.ReturnType.FullName != "System.Void")
			{
				if (Method.ReturnType.IsArray)
				{
					yield return "outSize: *mut u32";
				}
				yield return "out: *mut " + TypeHelpers.GetTypeName(DeclaringType.Generator, this, Method.ReturnType, TypeUsage.Raw);
			}
		}

		public FeatureConditions GetFeatureCondition()
		{
			var dependsOnAssemblies = new List<string>(ForeignAssemblyDependencies.GroupBy(t => t.Module.Assembly.Name.Name).Select(g => g.Key));
			return new FeatureConditions(dependsOnAssemblies);
		}
	}
}
