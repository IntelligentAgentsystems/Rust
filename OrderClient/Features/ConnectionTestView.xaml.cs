using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace OrderClient.Features
{
	public class ConnectionTestView : UserControl
	{
		public ConnectionTestView()
		{
			this.InitializeComponent();
		}

		private void InitializeComponent()
		{
			AvaloniaXamlLoader.Load(this);
		}
	}
}
