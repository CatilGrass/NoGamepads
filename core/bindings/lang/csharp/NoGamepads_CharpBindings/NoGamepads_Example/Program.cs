using NoGamepads_Core.Data;
using NoGamepads_Core.Runtime;
using NoGamepads_Core.Services.Tcp;
using NoGamepads_Sharp;

namespace NoGamepads_Example;

internal class Program
{
    public static void Main(string[] args)
    {
        LoggerManagement.EnableLogger(2);
        
        Player player = new Player("CatilGrass", "123456");

        player.Hue = 250;
        player.Value = 1;
        player.Saturation = 0.5f;
        
        Console.WriteLine($"player \"{player.Id}\" hsv: {player.Hue}, {player.Saturation}, {player.Value}");
        
        ControllerData data = new ControllerData(player);
        
        ControllerRuntime runtime = new ControllerRuntime(data);
        
        new PadTcpClient(runtime)
            .SetAddressV4()
            .Connect();
    }
}