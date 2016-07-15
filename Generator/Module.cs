using System;
using System.Collections.Generic;
using System.Text;

namespace Generator
{
	public class Module
	{
		Dictionary<string, Module> children = new Dictionary<string, Module>();

		public IReadOnlyDictionary<string, Module> Children
		{
			get
			{
				return children;
			}
		}

		public string Name { get; private set; }
		public StringBuilder Text { get; private set; }

		public Module(string name)
		{
			Name = name;
			Text = new StringBuilder();
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
				mod = new Module(name);
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
	}
}
