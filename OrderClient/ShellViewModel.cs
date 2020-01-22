namespace OrderClient
{
	public class ShellViewModel : ViewModelBase
	{
		private ViewModelBase main;

		public ShellViewModel()
		{
			Main = new Features.Ordering.OrderOverviewViewModel();
		}

		public ViewModelBase Main {
			get => main;
			set => Set(ref main, value);
		}
	}
}
