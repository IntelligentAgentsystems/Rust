using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace OrderClient.Features.Ordering
{
	public class OrderOverviewView : UserControl
	{
		public OrderOverviewView()
		{
			this.InitializeComponent();
		}

		private void InitializeComponent()
		{
			AvaloniaXamlLoader.Load(this);
		}
	}
}
