using NoGamepads_Core.Data;
using NoGamepads_Core.Runtime;

namespace NoGamepads_Example;

internal class Program
{
    public static void Main(string[] args)
    {
        Player player = new Player("CatilGrass", "Unknown Password");

        player.Hue = 200;
        
        ControllerData data = new ControllerData(player);
        
        ControllerRuntime runtime = new ControllerRuntime(data);
        
        runtime.SendTextMessage("Hello World! ");
        Console.WriteLine(player.Hue + "!");
        
        // Console.WriteLine(player.Hash);
    }
}