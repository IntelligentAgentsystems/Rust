using ReactiveUI;
using System;
using System.Collections.Generic;
using System.Text;

namespace OrderClient
{
	public class ShellViewModel : ViewModelBase
	{
		private ViewModelBase main;

		public ShellViewModel()
		{
			Main = new Features.ConnectionTestViewModel();
		}

		public ViewModelBase Main {
			get => main;
			set => this.RaiseAndSetIfChanged(ref main, value);
		}
	}
}
