using System;
using System.Collections.Generic;
using System.Linq;
using static System.Diagnostics.Debug;

using Mono.Cecil;
using Mono.Cecil.Rocks;

namespace Generator.Types
{
    public class TypeHelpers
    {
        public static bool IsDelegate(TypeDefinition t)
        {
            return t.IsClass && t.BaseType.FullName == "System.MulticastDelegate";
        }

        public static TypeReference InstantiateType(GenericInstanceType ty, Dictionary<string, TypeReference> map)
        {
            var genericArguments = ty.GenericArguments.Select(arg =>
            {
                if (arg.MetadataType == MetadataType.Var)
                {
                    return map[arg.FullName];
                }
                else if (arg is GenericInstanceType)
                {
                    return InstantiateType((GenericInstanceType)arg, map);
                }
                else
                {
                    return arg;
                }
            }).ToArray();
            return ty.ElementType.MakeGenericInstanceType(genericArguments);
        }

        // ---------------- The following should be split by types ---------------- //

        public static string GetTypeName(Generator gen, ITypeRequestSource source, TypeReference t, TypeUsage usage, bool hasConstModifier = false)
        {
            if (hasConstModifier && !t.IsByReference)
            {
                throw new NotSupportedException(); // we don't expect this case to happen
            }
            else if (t.IsByReference)
            {
                var ty = (ByReferenceType)t;
                if (hasConstModifier)
                {
                    if (usage == TypeUsage.Raw)
                        return $"*const { GetTypeName(gen, source, ty.ElementType, usage) }";
                    else if (usage == TypeUsage.In)
                        return $"&{ GetTypeName(gen, source, ty.ElementType, usage) }";
                    else
                        throw new NotSupportedException();
                }
                else
                {
                    if (usage == TypeUsage.Raw)
                        return $"*mut { GetTypeName(gen, source, ty.ElementType, usage) }";
                    else if (usage == TypeUsage.In) // currently never used
                        return $"&mut { GetTypeName(gen, source, ty.ElementType, usage) }";
                    else
                        throw new NotSupportedException();
                }
            }
            else if (t.IsGenericParameter)
            {
                switch (usage)
                {
                    case TypeUsage.Raw: return $"{ t.Name }::Abi";
                    case TypeUsage.In: return $"&{ t.Name }::In";
                    case TypeUsage.Out: return $"{ t.Name }::Out";
                    case TypeUsage.OutNonNull: return $"{ t.Name }::OutNonNull";
                    case TypeUsage.GenericArg: return t.Name;
                    default: throw new NotSupportedException();
                }
            }
            else if (t.IsArray)
            {
                var ty = (ArrayType)t;
                if (usage == TypeUsage.Out)
                {
                    return $"ComArray<{ GetTypeName(gen, source, ty.ElementType, TypeUsage.GenericArg) }>";
                }
                else
                {
                    return $"*mut { GetTypeName(gen, source, ty.ElementType, usage) }";
                }
            }
            else if (t.IsOptionalModifier)
            {
                var ty = (OptionalModifierType)t;
                if (ty.ModifierType.FullName == "System.Runtime.CompilerServices.IsConst")
                {
                    return GetTypeName(gen, source, ty.ElementType, usage, true);
                }
                else
                {
                    // according to ECMA Partition II documentation, we can usually just ignore optional modifiers
                    return GetTypeName(gen, source, ty.ElementType, usage, hasConstModifier);
                }
            }
            else if (t.IsRequiredModifier)
            {
                throw new NotImplementedException($"Unexpected reqmod { ((RequiredModifierType)t).ModifierType.FullName } is not implemented");
            }
            else
            {
                return GetElementTypeName(gen, source, t, usage);
            }
        }

        private static string GetElementTypeName(Generator gen, ITypeRequestSource source, TypeReference t, TypeUsage usage)
        {
            if (t.FullName == "System.String")
            {
                switch (usage)
                {
                    case TypeUsage.Raw: return "HSTRING";
                    case TypeUsage.In: return "&HStringArg";
                    case TypeUsage.Out: return "HString";
                    case TypeUsage.GenericArg: return "HString";
                    default: throw new NotSupportedException();
                }
            }
            else if (t.FullName == "System.Object")
            {
                switch (usage)
                {
                    case TypeUsage.Raw: return "*mut IInspectable";
                    case TypeUsage.GenericArg: return "IInspectable";
                    case TypeUsage.In: return "&IInspectable";
                    case TypeUsage.Out: return "Option<ComPtr<IInspectable>>";
                    default: throw new InvalidOperationException();
                }
            }
            else if (t.FullName == "System.Guid")
            {
                return "Guid";
            }
            else if (t.IsPrimitive)
            {
                switch (t.FullName)
                {
                    case "System.Boolean":
                        return "bool";
                    case "System.SByte":
                        return "i8";
                    case "System.Int16":
                        return "i16";
                    case "System.Int32":
                        return "i32";
                    case "System.Int64":
                        return "i64";
                    case "System.Byte":
                        return "u8";
                    case "System.UInt16":
                        return "u16";
                    case "System.UInt32":
                        return "u32";
                    case "System.UInt64":
                        return "u64";
                    case "System.Single":
                        return "f32";
                    case "System.Double":
                        return "f64";
                    case "System.Char":
                        return "Char";
                    default:
                        throw new NotImplementedException("Primitive type: " + t.FullName);
                }
            }
            else
            {
                var def = gen.GetTypeDefinition(t);
                source.AddDependency(def);

                string name = def.GetPath(source.Module);

                int i = name.IndexOf('`');
                if (i >= 0)
                {
                    name = name.Substring(0, i);
                }

                if (t.IsGenericInstance)
                {
                    var ty = (GenericInstanceType)t;
                    if (!ty.ContainsGenericParameter)
                    {
                        gen.AddGenericInstance(ty);
                    }
                    name += $"<{ String.Join(", ", ty.GenericArguments.Select(a => GetTypeName(gen, source, a, TypeUsage.GenericArg))) }>";
                }

                if (!t.IsValueType)
                {
                    if (usage == TypeUsage.In)
                    {
                        name = $"&{ name }";
                    }
                    else if (usage == TypeUsage.GenericArg)
                    {
                        // leave name unchanged
                    }
                    else if (usage == TypeUsage.Raw)
                    {
                        name = $"*mut { name }";
                    }
                    else if (usage == TypeUsage.Out)
                    {
                        name = $"Option<ComPtr<{ name }>>";
                    }
                    else if (usage == TypeUsage.OutNonNull)
                    {
                        name = $"ComPtr<{ name }>";
                    }
                }

                return name;
            }
        }

        public static string GetInputTypeName(Generator gen, ITypeRequestSource source, TypeReference t, InputKind kind)
        {
            switch (kind)
            {
                case InputKind.Default: return GetTypeName(gen, source, t, TypeUsage.In);
                case InputKind.Raw: return GetTypeName(gen, source, t, TypeUsage.Raw);
                case InputKind.Slice: return $"&[{ GetTypeName(gen, source, t, TypeUsage.In) }]";
                case InputKind.MutSlice:
                    Assert(t.IsValueType);
                    return $"&mut [{ GetTypeName(gen, source, t, TypeUsage.In) }]";
                case InputKind.VecBuffer: return $"&mut Vec<{ GetTypeName(gen, source, t, TypeUsage.OutNonNull) }>";
                default: throw new InvalidOperationException();
            }
        }

        public static string UnwrapInputParameter(string name, TypeReference t, bool hasConstModifier = false)
        {
            if (t.IsGenericParameter)
            {
                return $"{ t.Name }::unwrap({ name })";
            }
            if (t.IsByReference)
            {
                // value types passed by reference
                return $"{ name } as *const _ as *mut _";
            }
            else if (t.IsArray)
            {
                return name; // TODO
            }
            else if (t.FullName == "System.String")
            {
                return $"{ name }.get()";
            }
            else if (t.FullName == "System.Object")
            {
                return $"{ name } as *const _ as *mut _";
            }
            else if (t.FullName == "System.Guid")
            {
                return name;
            }
            else if (t.IsPrimitive)
            {
                switch (t.FullName)
                {
                    case "System.Boolean":
                    case "System.Byte":
                    case "System.Int16":
                    case "System.Int32":
                    case "System.Int64":
                    case "System.UInt16":
                    case "System.UInt32":
                    case "System.UInt64":
                    case "System.Single":
                    case "System.Double":
                    case "System.Char":
                        return name;
                    default:
                        throw new NotImplementedException("Primitive type: " + t.FullName);
                }
            }
            else if (t.IsValueType)
            {
                return name;
            }
            else if (t.IsOptionalModifier)
            {
                var ty = (OptionalModifierType)t;
                if (ty.ModifierType.FullName == "System.Runtime.CompilerServices.IsConst")
                {
                    return UnwrapInputParameter(name, ty.ElementType, true);
                }
                else
                {
                    // according to ECMA Partition II documentation, we can usually just ignore optional modifiers
                    return UnwrapInputParameter(name, ty.ElementType, hasConstModifier);
                }
            }
            else if (t.IsRequiredModifier)
            {
                throw new NotImplementedException($"Unexpected reqmod { ((RequiredModifierType)t).ModifierType.FullName } is not implemented");
            }
            else // reference type
            {
                return $"{ name } as *const _ as *mut _";
            }
        }

        public static IEnumerable<string> CreateUninitializedOutputs(string name, TypeReference t)
        {
            if (t.IsArray)
            {
                yield return $"let mut { name }Size = 0;";
            }
            yield return $"let mut { name } = { CreateUninitializedOutput(t) };";
        }

        public static string CreateUninitializedOutput(TypeReference t)
        {
            if (t.IsGenericParameter)
            {
                return $"{ t.Name }::uninitialized()";
            }
            if (t.IsByReference)
            {
                throw new NotSupportedException();
            }
            else if (t.IsArray)
            {
                return "null_mut()"; // TODO?
            }
            else if (t.FullName == "System.String")
            {
                return "null_mut()";
            }
            else if (t.FullName == "System.Object")
            {
                return "null_mut()";
            }
            else if (t.FullName == "System.Guid")
            {
                return "zeroed()";
            }
            else if (t.IsPrimitive)
            {
                switch (t.FullName)
                {
                    case "System.Boolean":
                    case "System.Byte":
                    case "System.Int16":
                    case "System.Int32":
                    case "System.Int64":
                    case "System.UInt16":
                    case "System.UInt32":
                    case "System.UInt64":
                    case "System.Single":
                    case "System.Double":
                    case "System.Char":
                        return "zeroed()";
                    default:
                        throw new NotImplementedException("Primitive type: " + t.FullName);
                }
            }
            else if (t.IsValueType)
            {
                return "zeroed()";
            }
            else // reference type
            {
                return "null_mut()";
            }
        }

        public static string WrapOutputParameter(string name, TypeReference t, bool isNonNull)
        {
            string wrapFn = isNonNull ? "ComPtr::wrap" : "ComPtr::wrap_optional";

            if (t.IsGenericParameter)
            {
                return $"{ t.Name }::wrap({ name })";
            }
            if (t.IsByReference)
            {
                throw new NotSupportedException();
            }
            else if (t.IsArray)
            {
                return $"ComArray::from_raw({ name }Size, { name })";
            }
            else if (t.FullName == "System.String")
            {
                return $"HString::wrap({ name })";
            }
            else if (t.FullName == "System.Object")
            {
                return $"{ wrapFn }({ name })";
            }
            else if (t.FullName == "System.Guid")
            {
                return name;
            }
            else if (t.IsPrimitive)
            {
                switch (t.FullName)
                {
                    case "System.Boolean":
                    case "System.Byte":
                    case "System.Int16":
                    case "System.Int32":
                    case "System.Int64":
                    case "System.UInt16":
                    case "System.UInt32":
                    case "System.UInt64":
                    case "System.Single":
                    case "System.Double":
                    case "System.Char":
                        return name;
                    default:
                        throw new NotImplementedException("Primitive type: " + t.FullName);
                }
            }
            else if (t.IsValueType)
            {
                return name;
            }
            else // reference type
            {
                return $"{ wrapFn }({ name })";
            }
        }

        public static TypeReference GetDefaultInterface(TypeDefinition type)
        {
            return type.Interfaces.Single(i => i.CustomAttributes.Any(a => a.AttributeType.Name == "DefaultAttribute")).InterfaceType;
        }

        public static bool IsReturnTypeNonNull(TypeReference t, Generator gen)
        {
            var nonNullReturnTypes = new string[]
            {
                "Windows.Foundation.IAsyncAction",
                "Windows.Foundation.IAsyncActionWithProgress`1",
                "Windows.Foundation.IAsyncOperation`1",
                "Windows.Foundation.IAsyncOperationWithProgress`2"
            };

            if (nonNullReturnTypes.Contains(t.GetElementType().FullName))
                return true;

            if (t.IsGenericParameter || t.IsArray || t.IsValueType || t.IsPrimitive ||
                t.FullName == "System.String" || t.FullName == "System.Object" || t.FullName == "System.Guid")
            {
                return false;
            }
            else
            {
                // we mark class types as non-null if their default interface is non-null
                var def = gen.GetTypeDefinition(t);
                if (def.Kind == TypeKind.Class)
                {
                    var defaultInterface = TypeHelpers.GetDefaultInterface(def.Type);
                    return nonNullReturnTypes.Contains(defaultInterface.GetElementType().FullName);
                }
                else
                    return false;
            }
        }
    }

    public enum TypeUsage
    {
        In,
        Out,
        OutNonNull,
        Raw,
        Alias,
        GenericArg,
    }

    public enum InputKind
    {
        Default,
        Raw,
        Slice,
        MutSlice,
        VecBuffer
    }
}
