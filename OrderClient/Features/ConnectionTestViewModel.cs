using System;
using System.Collections.Generic;
using System.Text;
using System.Threading.Tasks;
using Google.Protobuf.WellKnownTypes;
using Grpc.Net.Client;
using ReactiveUI;

namespace OrderClient.Features
{
	public class ConnectionTestViewModel : ViewModelBase
	{
		private string host;
		private string output;

		public ConnectionTestViewModel()
		{
			Host = "localhost:5010";
		}

		public string Host {
			get => host;
			set => this.RaiseAndSetIfChanged(ref host, value);
		}

		public string Output {
			get => output;
			set => this.RaiseAndSetIfChanged(ref output, value);
		}

		public async Task TestConnection()
		{
			Output = "";

			try
			{
				using var channel = GrpcChannel.ForAddress($"http://{Host}");
				var client = new Fiab.OrderService.OrderServiceClient(channel);

				var reply = await client.OrderAsync(new Empty());
				Output = "Success!";
			}
			catch (Exception ex)
			{
				Output = $"Error! {ex.GetType().Name}: {ex.Message}\n{ex}";
			}
		}
	}
}
