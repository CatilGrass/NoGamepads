using NoGamepads_Core.Data;

Player player = new Player("Pzw", "123456")
{
    NickName = ""
};

Console.WriteLine("Player hash : " + player.Hash);
Console.WriteLine("Player name : " + player.NickName);

player.NickName = "OMG";

Console.WriteLine("Player name : " + player.NickName);

player.Hue = 92;
player.Value = 1;
player.Saturation = 1;

Console.WriteLine($"Color : H: {player.Hue}, S: {player.Saturation}, V: {player.Value}");