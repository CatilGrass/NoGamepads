using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public static class PlayerList
{
    public static unsafe List<Player> ToPlayerList(this FfiPlayerList ffiPlayerList)
    {
        var playerList = new List<Player>();

        var native = (FfiPlayerList.__Internal*) ffiPlayerList.__Instance;
        
        var playersPtr = (FfiPlayer.__Internal*) native -> players.ToPointer();
        ulong count = native->len;
        
        for (ulong i = 0; i < count; i++)
        {
            var current = playersPtr + (int)i;
            IntPtr playerPtr = new IntPtr(current);
            
            FfiPlayer ffiPlayer = FfiPlayer.__CreateInstance(playerPtr);
            playerList.Add(new Player(ffiPlayer));
        }

        return playerList;
    }
}