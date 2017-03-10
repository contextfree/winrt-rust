using System;
using System.Collections.Generic;
using System.Linq;

namespace Generator
{
	public class FeatureConditions
	{
		private List<string> conditions;

		public bool IsEmpty
		{
			get
			{
				return conditions.Count == 0;
			}
		}

		public FeatureConditions(IEnumerable<string> conditions)
		{
			this.conditions = new List<string>(conditions.Select(asm => "feature=\"" + asm.ToLower().Replace('.', '-') + "\""));
			this.conditions.Sort(); // we want deterministic code generation
		}

		public string GetAttribute()
		{
			if (conditions.Count == 0)
			{
				return "";
			}
			else if (conditions.Count == 1)
			{
				return "#[cfg(" + conditions.Single() + ")] ";
			}
			else
			{
				return "#[cfg(all(" + string.Join(",", conditions) + "))] ";
			}
		}

		public string GetInvertedAttribute()
		{
			if (conditions.Count == 0)
			{
				return "";
			}
			else if (conditions.Count == 1)
			{
				return "#[cfg(not(" + conditions.Single() + "))] ";
			}
			else
			{
				return "#[cfg(not(all(" + string.Join(",", conditions) + ")))] ";
			}
		}
	}
}
