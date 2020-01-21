using Grpc.Net.Client;
using System;
using System.Threading;
using System.Threading.Tasks;

namespace OrderClient.Features.Ordering
{
	public class OrderOverviewViewModel : ViewModelBase
	{
		private string output;
		private string host;
		private bool orderInProgress;

		public OrderOverviewViewModel()
		{
			Host = "localhost:5010";

			NewOrder = new NewOrderViewModel();
			NewOrder.PropertyChanged += (s, e) =>
			{
				if (e.PropertyName == nameof(NewOrderViewModel.IsValid))
					Raise(nameof(CanStartOrder));
			};
		}

		public NewOrderViewModel NewOrder { get; }

		public bool CanStartOrder 
			=> !orderInProgress 
			&& (NewOrder?.IsValid ?? false)
			&& !string.IsNullOrWhiteSpace(Host);

		public string Host {
			get => host;
			set {
				if (Set(ref host, value))
					Raise(nameof(CanStartOrder));
			}
		}

		public string Output {
			get => output;
			set => Set(ref output, value);
		}

		public async Task StartOrder()
		{
			Output = "";
			orderInProgress = true;
			NewOrder.CanEdit = false;

			Raise(nameof(CanStartOrder));

			try
			{
				using var channel = GrpcChannel.ForAddress($"http://{Host}");
				var client = new Fiab.OrderService.OrderServiceClient(channel);

				var request = new Fiab.OrderRequest();
				request.Customer = NewOrder.Customer;
				request.Functions.AddRange(NewOrder.Functions);

				using (var call = client.Order(request))
				{
					while (await call.ResponseStream.MoveNext(default(CancellationToken)))
					{
						var status = call.ResponseStream.Current;
						switch (status.State)
						{
							case Fiab.OrderStatusUpdate.Types.State.Started:
								Output += "Started\n";
								break;
							case Fiab.OrderStatusUpdate.Types.State.InProgress:
								Output += $"In Progress to {status.NextFunc}\n";
								break;
							case Fiab.OrderStatusUpdate.Types.State.Done:
								Output += "Done\n";
								break;
							case Fiab.OrderStatusUpdate.Types.State.PathInUseTooOften:
								Output += "PathInUseTooOften\n";
								break;
							case Fiab.OrderStatusUpdate.Types.State.NoPathFound:
								Output += "NoPathFound\n";
								break;
							case Fiab.OrderStatusUpdate.Types.State.PlottingFailed:
								Output += "PlottingFailed\n";
								break;
							case Fiab.OrderStatusUpdate.Types.State.TransportFailed:
								Output += "TransportFailed\n";
								break;
							default:
								break;
						}
					}
				}
				Output += "Done!";
			}
			catch (Exception ex)
			{
				Output = $"Error! {ex.GetType().Name}: {ex.Message}\n{ex}";
			}
			finally
			{
				orderInProgress = false;
				NewOrder.CanEdit = true;

				Raise(nameof(CanStartOrder));
			}
		}
	}
}
