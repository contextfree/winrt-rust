using System;
using System.Linq;
using System.Text;

namespace Generator
{
	public class NameHelpers
	{
		public static string PreventKeywords(string name)
		{
			if (name == "type" || name == "Self" || name == "box" || name == "move") // TODO: add more keywords
			{
				name += "_";
			}

			return name;
		}

		public static string FirstToLower(string name)
		{
			return name[0].ToString().ToLower() + name.Substring(1);
		}

		public static string CamelToSnakeCase(string name)
		{
			var newName = new StringBuilder();
			bool noUnderscore = true;
			bool previousUpper = false;
			foreach (var c in name)
			{
				if (char.IsUpper(c))
				{
					if (!noUnderscore && !previousUpper) newName.Append("_");
					newName.Append(char.ToLowerInvariant(c));
					previousUpper = true;
				}
				else
				{
					newName.Append(c);
					previousUpper = false;
				}

				noUnderscore = false;
				if (c == '_') noUnderscore = true;
			}
			return newName.ToString();
		}

		// Returns the string as an UTF16 encoded, null-terminated sequence of u16 values
		public static string StringToUTF16WithZero(string str)
		{
			return String.Join(",", str.Select(c => ((ushort)c).ToString()).Concat(new string[] { "0" }));
		}
	}
}
