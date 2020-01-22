using Grpc.Net.Client;
using System;
using System.Threading;
using System.Threading.Tasks;

namespace OrderClient.Features
{
	public class ConnectionTestViewModel : ViewModelBase
	{
		private string host;
		private string output;
		private bool isReady;

		public ConnectionTestViewModel()
		{
			Host = "localhost:5010";
			IsReady = true;
		}

		public string Host {
			get => host;
			set => Set(ref host, value);
		}

		public string Output {
			get => output;
			set => Set(ref output, value);
		}

		public bool IsReady {
			get => isReady;
			set => Set(ref isReady, value);
		}

		public async Task TestConnection()
		{
			Output = "";
			IsReady = false;

			try
			{
				using var channel = GrpcChannel.ForAddress($"http://{Host}");
				var client = new Fiab.OrderService.OrderServiceClient(channel);

				var request = new Fiab.OrderRequest();
				request.Customer = "Martin";
				request.Functions.Add(Fiab.PlotterFunction.DrawBlue);

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
				IsReady = true;
			}
		}
	}
}
