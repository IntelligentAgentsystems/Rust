using System;
using System.Collections.Generic;
using System.Collections.ObjectModel;
using System.Text;

namespace OrderClient.Features.Ordering
{
	public class NewOrderViewModel : ViewModelBase
	{
		private string customer;
		private bool canEdit;

		public NewOrderViewModel()
		{
			CanEdit = true;
		}

		public string Customer {
			get => customer;
			set {
				if (Set(ref customer, value))
					Raise(nameof(IsValid));
			}
		}

		public bool CanEdit {
			get => canEdit;
			set => Set(ref canEdit, value);
		}

		public bool IsValid
			=> !string.IsNullOrWhiteSpace(Customer)
			&& Functions.Count > 0;

		public ObservableCollection<Fiab.PlotterFunction> Functions { get; } = new ObservableCollection<Fiab.PlotterFunction>();

		public void AddRed()
			=> Add(Fiab.PlotterFunction.DrawRed);
		public void AddGreen()
			=> Add(Fiab.PlotterFunction.DrawGreen);
		public void AddBlue()
			=> Add(Fiab.PlotterFunction.DrawBlue);
		public void AddYellow()
			=> Add(Fiab.PlotterFunction.DrawYellow);

		public void Remove(int idx)
		{
			if (idx >= 0 && idx < Functions.Count)
			{
				Functions.RemoveAt(idx);
				Raise(nameof(IsValid));
			}
		}

		private void Add(Fiab.PlotterFunction func)
		{
			Functions.Add(func);
			Raise(nameof(IsValid));
		}
	}
}
