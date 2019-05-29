using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;

namespace Generator.Types
{
    public class MethodDetailsCache
    {
        public string WrappedName;
        public string RawName;
        public string[] InputParameterNames;
        public Tuple<TypeReference, InputKind>[] InputParameterTypes;
        public Tuple<TypeReference, bool>[] OutTypes;
        public string[] GeneratedOutTypes;
        public string GetManyParameterName;
        public List<Tuple<string, TypeReference, bool>> Output;

        public string[] MakeInputParameters(Generator generator, ITypeRequestSource source)
        {
            return InputParameterNames.Zip(InputParameterTypes, (name, t) => name + ": " + TypeHelpers.GetInputTypeName(generator, source, t.Item1, t.Item2)).ToArray();
        }

        public string MakeOutType(Generator generator, ITypeRequestSource source, bool isFactoryMethod)
        {
            GeneratedOutTypes = OutTypes.Select(o => TypeHelpers.GetTypeName(generator, source, o.Item1, (o.Item2 || isFactoryMethod) ? TypeUsage.OutNonNull : TypeUsage.Out)).ToArray();
            string outType = String.Join(", ", GeneratedOutTypes);
            if (OutTypes.Count() != 1)
            {
                outType = "(" + outType + ")"; // also works for count == 0 (empty tuple)
            }
            return outType;
        }

        public string MakeWrapperBody(MethodDefinition def, bool isFactoryMethod)
        {
            var rawName = RawName;
            bool isGetMany = GetManyParameterName != null;
            var getManyPname = GetManyParameterName;
            var output = Output;

            var rawParams = new List<string> { "self.as_abi() as *const _ as *mut _" };
            foreach (var p in def.Parameters)
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
                                rawParams.Add(pname + ".len() as u32");
                                rawParams.Add(pname + ".as_mut_ptr() as *mut _");
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

            if (def.ReturnType.FullName != "System.Void")
            {
                if (def.ReturnType.IsArray)
                {
                    rawParams.Add("&mut outSize");
                }
                rawParams.Add("&mut out");
            }

            var outInit = String.Join(" ", output.SelectMany(o => TypeHelpers.CreateUninitializedOutputs(o.Item1, o.Item2)));
            if (outInit != "") outInit = "\r\n        " + outInit;

            var outWrap = String.Join(", ", output.Zip(GeneratedOutTypes, (a, b) => Tuple.Create(a.Item1, a.Item2, a.Item3, b)).Select(o => TypeHelpers.WrapOutputParameter(o.Item1, o.Item2, isFactoryMethod || o.Item3, o.Item4)));
            if (output.Count != 1)
            {
                outWrap = "(" + outWrap + ")"; // also works for count == 0 (empty tuple)
            }
            outWrap = "Ok(" + outWrap + ")";

            if (isGetMany)
            {
                outInit = $"\r\n        debug_assert!({ getManyPname }.capacity() > 0, \"capacity of `{ getManyPname }` must not be 0 (use Vec::with_capacity)\"); { getManyPname }.clear();{ outInit }";
                outWrap = $"{ getManyPname }.set_len(out as usize); Ok(())";
            }

            return outInit + $@"
        let hr = ((*self.as_abi().lpVtbl).{ rawName })({ String.Join(", ", rawParams) });
        if hr == S_OK {{ { outWrap } }} else {{ err(hr) }}";
        }
    }

    public class MethodDef : ITypeRequestSource
    {
        static string[] MutatingMethods =
        {
            "Windows.Foundation.Collections.IVector::SetAt",
            "Windows.Foundation.Collections.IVector::InsertAt",
            "Windows.Foundation.Collections.IVector::RemoveAt",
            "Windows.Foundation.Collections.IVector::Append",
            "Windows.Foundation.Collections.IVector::RemoveAtEnd",
            "Windows.Foundation.Collections.IVector::Clear",
            "Windows.Foundation.Collections.IVector::ReplaceAll",
            "Windows.Foundation.Collections.IMap::Insert",
            "Windows.Foundation.Collections.IMap::Remove",
            "Windows.Foundation.Collections.IMap::Clear",
        };

        public TypeDef DeclaringType { get; private set; }
        public MethodDefinition Method { get; private set; }
        public bool IsFactoryMethod
        {
            get
            {
                var kind = ((InterfaceDef)DeclaringType).InterfaceKind;
                if (kind == InterfaceKind.Unidentified) { throw new InvalidOperationException(); }
                return kind == InterfaceKind.Factory;
            }
        }

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

        private MethodDetailsCache details;
        public MethodDetailsCache Details
        {
            get
            {
                if (details == null)
                {
                    details = InitializeDetailsCache();
                }
                return details;
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
            string name = NameHelpers.PreventKeywords(NameHelpers.CamelToSnakeCase(rawName.StartsWith("put_") ? "set_" + rawName.Substring(4) : rawName));
            if (rawName.Contains("_")) // name already contains '_' -> might result in a name clash after renaming, e.g. caused by original names `get_Name` (property getter) and `GetName` (method)
            {
                if (DeclaringType.Methods.Select(mm => Tuple.Create(mm, mm.GetRawName())).Where(mm => !mm.Item2.Contains("_")).Any(mm => mm.Item1.GetWrapperName(mm.Item2) == name))
                {
                    name += "_";
                }
            }
            return name;
        }

        public string GetWrapperDefinition()
        {
            var inputParameters = Details.MakeInputParameters(DeclaringType.Generator, this);
            var outType = Details.MakeOutType(DeclaringType.Generator, this, IsFactoryMethod);

            string selfArg;
            if (MutatingMethods.Contains(DeclaringType.Namespace + "." + DeclaringType.DefinitionName + "::" + Method.Name))
            {
                selfArg = "&mut self";
            }
            else
            {
                selfArg = "&self";
            }

            return $@"#[inline] pub fn { Details.WrappedName }({ String.Join(", ", new string[] { selfArg }.Concat(inputParameters)) }) -> Result<{ outType }> {{ unsafe {{ { Details.MakeWrapperBody(Method, IsFactoryMethod) }
    }}}}";
        }

        private MethodDetailsCache InitializeDetailsCache()
        {
            string rawName = GetRawName();
            string name = GetWrapperName(rawName);

            bool isGetMany = (rawName == "GetMany" && DeclaringType.Namespace == "Windows.Foundation.Collections" &&
                             (DeclaringType.Name == "IVectorView`1" || DeclaringType.Name == "IIterator`1" || DeclaringType.Name == "IVector`1"));
            string getManyPname = null;

            // These `GetMany` methods have special semantics, since it takes an array and returns the number of elements that were filled
            // It uses the __RPC__out_ecount_part(capacity, *actual) annotation in the C headers. For the wrapper we use a &mut Vec<> buffer.

            var input = new List<Tuple<string, TypeReference, InputKind>>();
            var output = new List<Tuple<string, TypeReference, bool>>();

            foreach (var p in Method.Parameters)
            {
                string pname = NameHelpers.PreventKeywords(NameHelpers.FirstToLower(p.Name));
                if (p.ParameterType.IsByReference)
                {
                    Assert(p.IsOut);
                    var realType = ((ByReferenceType)p.ParameterType).ElementType;
                    output.Add(Tuple.Create(pname, realType, false));
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
                                input.Add(Tuple.Create(pname, ((ArrayType)p.ParameterType).ElementType, InputKind.VecBuffer));
                            }
                            else
                            {
                                input.Add(Tuple.Create(pname, ((ArrayType)p.ParameterType).ElementType, InputKind.MutSlice));
                            }
                        }
                        else
                        {
                            input.Add(Tuple.Create(pname, ((ArrayType)p.ParameterType).ElementType, InputKind.Slice));
                        }
                    }
                    else
                    {
                        input.Add(Tuple.Create(pname, p.ParameterType, InputKind.Default));
                    }
                }
            }

            if (Method.ReturnType.FullName != "System.Void")
            {
                // this makes the actual return value the last in the tuple (if multiple)
                output.Add(Tuple.Create("out", Method.ReturnType, TypeHelpers.IsReturnTypeNonNull(Method.ReturnType, DeclaringType.Generator)));
            }

            // TODO: second tuple element should be true for some method's return value
            var outTypes = output.Select(o => Tuple.Create(o.Item2, o.Item3)).ToArray();

            if (isGetMany)
            {
                outTypes = new Tuple<TypeReference, bool>[] { }; // GetMany has no return value
            }



            return new MethodDetailsCache
            {
                WrappedName = name,
                RawName = rawName,
                InputParameterNames = input.Select(i => i.Item1).ToArray(),
                InputParameterTypes = input.Select(i => Tuple.Create(i.Item2, i.Item3)).ToArray(),
                OutTypes = outTypes.ToArray(),
                GetManyParameterName = isGetMany ? getManyPname : null,
                Output = output
            };
        }

        public string GetRawDeclaration()
        {
            var name = GetRawName();
            return $"fn { name }({ String.Join(", ", GetParameterDeclarations()) }) -> HRESULT";
        }

        public IEnumerable<string> GetParameterDeclarations()
        {
            yield return "&self";
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
