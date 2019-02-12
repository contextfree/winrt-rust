using System;
using System.Linq;
using System.Text;
using System.Text.RegularExpressions;

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

        static Regex CaseConversionExceptionsRegex = new Regex(
            "(Base64|Float32|Float64|DBm|UInt|SInt|SFloat|Direct3D|VCard|OAuth|CData|ESim|MPeg|WMAudio|URLs|EFSpn|EFOns|EFOpl|EFPnn|ETag|IndexedDB|JavaScript|HResult|UIElement|IMediaSource|IAnimationObject|IBuffer)" +
            "([A-Z0-9_]|$)", RegexOptions.Compiled);

        public static string CamelToSnakeCase(string name)
        {
            var newName = new StringBuilder();
            bool noUnderscore = true;
            bool previousUpper = false;
            bool inNumeric = false;
            uint distPrevUpper = uint.MaxValue; // distance to previous uppercase letter

            int ci = 0;

            Func<int, bool> isLower = (offset) => ci + offset < name.Length && ci + offset >= 0 && char.IsLower(name[ci + offset]);
            Func<int, bool> isUpper = (offset) => ci + offset < name.Length && ci + offset >= 0 && char.IsUpper(name[ci + offset]);
            Func<int, bool> isDigit = (offset) => ci + offset < name.Length && ci + offset >= 0 && char.IsDigit(name[ci + offset]);

            var matches = CaseConversionExceptionsRegex.Matches(name).Cast<Match>().Select(m => m.Captures[0]).ToArray();
            Func<bool> withinMatch = () => matches.Any(m => ci > m.Index && ci < (m.Index + m.Length - 1));

            for (; ci < name.Length; ci++)
            {
                var c = name[ci];
                if (isUpper(0))
                {
                    distPrevUpper = 0;
                    if (!noUnderscore && (!previousUpper || isLower(1)) && !withinMatch())
                        newName.Append("_");
                    newName.Append(char.ToLowerInvariant(c));
                    previousUpper = true;
                    inNumeric = false;
                }
                else if (isDigit(0) && (isDigit(1) || (isUpper(1) && !isLower(2))))
                {
                    distPrevUpper += 1;
                    bool seen4LetterException = false;
                    if (distPrevUpper == 4)
                    {
                        var last4 = name.Substring(ci - 4, 4);
                        seen4LetterException = last4 == "Char" || last4 == "Argb" || last4 == "Wtls" || last4 == "Ieee";
                    }
                    bool seen2LetterException = false;
                    if (ci >= 2)
                    {
                        var last2 = name.Substring(ci - 2, 2);
                        seen2LetterException = last2 == "Is" || last2 == "3D";
                    }
                    if (!noUnderscore && !inNumeric && ((distPrevUpper > 3 && !seen4LetterException) || seen2LetterException) && !withinMatch())
                        newName.Append("_");
                    newName.Append(c);
                    previousUpper = true;
                    inNumeric = true;
                }
                else
                {
                    distPrevUpper += 1;
                    newName.Append(c);
                    previousUpper = false;
                    inNumeric = inNumeric && (isDigit(0) || isDigit(1)); // stay in numeric mode if there's just one letter in between
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

        public static Tuple<string, int> GetSortKeyIgnoringInterfacePrefix(string str)
        {
            if (str[0] == 'I' && char.IsUpper(str[1]))
            {
                return Tuple.Create(str.Substring(1), 0);
            }
            else
            {
                return Tuple.Create(str, 1);
            }
        }
    }
}
