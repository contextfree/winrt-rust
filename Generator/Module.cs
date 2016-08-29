using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

using Mono.Cecil;

namespace Generator
{
	public class Module
	{
		public bool IsEmpty
		{
			get
			{
				return (children.Count == 0 || children.Values.All(c => c.IsEmpty)) && Text.Length == 0;
			}
		}

		Dictionary<string, Module> children = new Dictionary<string, Module>();
		public Module Parent { get; private set; }

		public IReadOnlyDictionary<string, Module> Children
		{
			get
			{
				return children;
			}
		}

		public string Name { get; private set; }

		public string FullName
		{
			get
			{
				if (Parent == null || String.IsNullOrEmpty(Parent.FullName))

				{
					return Name;
				}
				else
				{
					return Parent.FullName + "." + Name;
				}
			}
		}

		public string Path
		{
			get
			{
				return FullName.ToLower().Replace(".", "::");
			}
		}

		public StringBuilder Text { get; private set; }

		public AssemblyDefinition Assembly { get; private set; }

		public Module(Module parent, string name)
		{
			Parent = parent;
			Name = name;
			Text = new StringBuilder();
		}

		public void AssignAssembly(AssemblyDefinition assembly)
		{
			if (Assembly == null)
			{
				Assembly = assembly;
			}
			else if (Assembly.Name.Name == assembly.Name.Name)
			{
				return; // already assigned
			}
			else
			{
				throw new NotSupportedException("This should never happen.");
			}

			if (Parent != null && FullName != assembly.Name.Name)
			{
				Parent.AssignAssembly(assembly);
			}
		}

		public void Append(string text)
		{
			Text.Append(text);
		}

		public Module FindChild(string path)
		{
			var split = path.Split(new char[] {'.'}, 2);
			var name = split[0];
			Module mod;
			if (!children.TryGetValue(name, out mod))
			{
				mod = new Module(this, name);
				children[name] = mod;
			}

			if (split.Length > 1)
			{
				return mod.FindChild(split[1]);
			}
			else
			{
				return mod;
			}
		}

		public string GetRelativePath(Module requestingModule)
		{
			string[] thisPath = FullName.ToLower().Split('.');
			string[] requestingPath = requestingModule.FullName.ToLower().Split('.');
			var maxCommonLength = Math.Min(thisPath.Length, requestingPath.Length);
			int i;
			for (i = 0; i < maxCommonLength; i++)
			{
				if (thisPath[i] != requestingPath[i])
					break;
			}

			var goUp = Enumerable.Repeat("super", requestingPath.Length - i); // how many times do we need to go up one level
			var goDown = thisPath.Skip(i);
			return string.Join("::", goUp.Concat(goDown));
		}

		public override string ToString()
		{
			return FullName;
		}
	}
}
