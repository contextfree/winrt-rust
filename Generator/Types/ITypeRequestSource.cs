namespace Generator.Types
{
	public interface ITypeRequestSource
	{
		Module Module { get; }
		void AddDependency(TypeDef other);
	}
}
