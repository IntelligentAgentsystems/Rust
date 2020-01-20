using Avalonia;
using Avalonia.Controls;
using Avalonia.Markup.Xaml;

namespace OrderClient
{
	public class ShellView : Window
	{
		public ShellView()
		{
			this.InitializeComponent();
#if DEBUG
			this.AttachDevTools();
#endif
		}

		private void InitializeComponent()
		{
			AvaloniaXamlLoader.Load(this);
		}
	}
}
