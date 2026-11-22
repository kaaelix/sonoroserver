using System;
using System.Threading;
using System.Threading.Tasks;

namespace AkiLogicService;

class Program
{
    static async Task Main(string[] args)
    {
        Console.WriteLine("==================================================");
        Console.WriteLine("  AkiLogicService (.NET 9 + Shared SQLite Engine) ");
        Console.WriteLine("==================================================");

        SqliteStore.EnsureInitialized();
        var (pCount, fCount) = SqliteStore.GetDatabaseStats();
        Console.WriteLine($"[AkiLogicService] SQLite stats: players={pCount}, state_files={fCount}");
        Console.WriteLine($"[AkiLogicService] Explicit C# Handlers: {IpcServer.RichHandlerCount}, domain_engine={DomainEngineHandler.LoadedRouteCount}");

        var cts = new CancellationTokenSource();
        Console.CancelKeyPress += (s, e) =>
        {
            e.Cancel = true;
            cts.Cancel();
        };

        var socketPath = Environment.GetEnvironmentVariable("AKI_IPC_SOCKET_PATH")
            ?? "/tmp/aki_logic.sock";
        var tcpPort = int.TryParse(Environment.GetEnvironmentVariable("AKI_IPC_TCP_PORT"), out var configuredPort)
            ? configuredPort
            : 19999;
        var server = new IpcServer(socketPath, tcpPort);
        try
        {
            await server.StartAsync(cts.Token);
        }
        catch (OperationCanceledException)
        {
            Console.WriteLine("[AkiLogicService] Shutting down gracefully...");
        }
        finally
        {
            server.Stop();
        }
    }
}
