using NoGamepads_Core.Data;
using NoGamepads_Core.Data.Message;
using NoGamepads_Core.Runtime;
using NoGamepads_Core.Services.Tcp;

namespace NoGamepads_Example;

internal class Program
{
    public static void Main(string[] args)
    {
        LoggerManagement.EnableLogger();
        
        Player player = new Player("CatilGrass", "123456");

        player.Hue = 250;
        player.Value = 1;
        player.Saturation = 0.5f;
        
        Console.WriteLine($"player \"{player.Id}\" hsv: {player.Hue}, {player.Saturation}, {player.Value}");
        
        ControllerData data = new ControllerData(player);
        
        ControllerRuntime runtime = new ControllerRuntime(data);
        
        var threadService = new Thread(() =>
        {
            new PadTcpClient(runtime)
                .SetAddressV4()
                .Connect();
        });

        var threadRuntime = new Thread(() =>
        {
            while (true)
            {
                var recent = runtime.RecentMessage;
                
                if (recent.MessageTag == GameMessage.Tag.End)
                    break;
                
                if (recent.MessageTag != GameMessage.Tag.Error)
                    Console.WriteLine(recent);
            }
            Console.WriteLine("\nPress any key to continue...");
        });
        
        threadService.Start();
        threadRuntime.Start();
    }
}