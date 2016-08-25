using System;
using System.Collections.Generic;
using System.Linq;

using Mono.Cecil;

using Generator.Types;

namespace Generator
{
	public class ParametricInterfaceManager
	{
		private static Guid Namespace = new Guid("11F47AD5-7B73-42C0-ABAE-878B1E16ADEE");

		private static IEnumerable<Type> baseTypes = new List<Type>
		{
			//typeof(void),
			typeof(byte),
			typeof(sbyte),
			typeof(short),
			typeof(ushort),
			typeof(int),
			typeof(uint),
			typeof(long),
			typeof(ulong),
			typeof(float),
			typeof(double),
			typeof(char),
			typeof(bool),
			typeof(string),
			typeof(object),
			typeof(Guid),
		};

		private static IEnumerable<string> baseTypes2 = new List<string>
		{
			"DateTime",
			"TimeSpan",
			"Point",
			"Size",
			"Rect"
		};


		private Dictionary<string, GenericInstanceType> genericInstances = new Dictionary<string, GenericInstanceType>();

		public void AddBaseIReferenceInstances(Generator gen)
		{
			var tyIReference = gen.GetTypeDefinition("Windows.Foundation.IReference`1");
			var tyIReferenceArray = gen.GetTypeDefinition("Windows.Foundation.IReferenceArray`1");
			var assembly = tyIReference.Module.Assembly; // we need just any assembly

			var allBaseTypes = baseTypes.Select(t => assembly.MainModule.Import(t)).Concat(baseTypes2.Select(t => gen.GetTypeDefinition("Windows.Foundation." + t).Type));

			foreach (var baseType in allBaseTypes)
			{
				AddGenericInstance(tyIReference.MakeGenericInstanceType(baseType));
				AddGenericInstance(tyIReferenceArray.MakeGenericInstanceType(baseType));
			}
		}

		public void AddGenericInstance(GenericInstanceType ty)
		{
			if (genericInstances.ContainsKey(ty.FullName)) return;

			genericInstances.Add(ty.FullName, ty);
			// recursively add generic arguments
			foreach (var arg in ty.GenericArguments)
			{
				if (arg.IsGenericInstance)
				{
					AddGenericInstance((GenericInstanceType)arg);
				}
			}

			// build mapping from generic parameter names to instantiated types
			TypeDefinition def = ty.Resolve();
			var genericParameterMap = new Dictionary<string, TypeReference>();
			for (int i = 0; i < def.GenericParameters.Count; i++)
			{
				genericParameterMap.Add(def.GenericParameters[i].FullName, ty.GenericArguments[i]);
			}

			// recursively add implemented interfaces	
			foreach (var intf in def.Interfaces.Where(i => i.ContainsGenericParameter && i is GenericInstanceType).Cast<GenericInstanceType>())
			{
				var type = TypeHelpers.InstantiateType(intf, genericParameterMap);
				if (type is GenericInstanceType)
				{
					AddGenericInstance(type as GenericInstanceType);
				}
			}

			// recursively add other instances introduced by function parameters
			foreach (var m in def.Methods)
			{
				foreach (GenericInstanceType pty in m.Parameters
					.Select(p => p.ParameterType)
					.Concat(Enumerable.Repeat(m.ReturnType, 1))
					.Where(t => t.ContainsGenericParameter && t is GenericInstanceType)
					.Cast<GenericInstanceType>())
				{
					var type = TypeHelpers.InstantiateType(pty, genericParameterMap);
					if (type is GenericInstanceType)
					{
						AddGenericInstance(type as GenericInstanceType);
					}
				}
			}
		}

		public IEnumerable<ParametricInterfaceInstance> Collect(Generator gen)
		{
			return genericInstances.Values.Select(t => new ParametricInterfaceInstance(gen, t));
		}
	}
}
