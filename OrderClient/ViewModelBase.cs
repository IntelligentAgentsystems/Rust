using ReactiveUI;
using System.Collections.Generic;
using System.Runtime.CompilerServices;

namespace OrderClient
{
	public class ViewModelBase : ReactiveObject
	{
		protected bool Set<T>(ref T field, T value, [CallerMemberName] string propertyName = null)
		{
			bool changing = !EqualityComparer<T>.Default.Equals(field, value); ;
			this.RaiseAndSetIfChanged(ref field, value, propertyName);
			return changing;
		}

		protected void Raise(string propertyName)
		{
			this.RaisePropertyChanged(propertyName);
		}
	}
}
