using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class GameArchiveData : IRawData<FfiGameRuntimeArchive>
{
    private readonly FfiGameRuntimeArchive _ffi = nogamepads_data.GameArchiveDataNew();

    public void AddBannedPlayer(Player player)
    {
        nogamepads_data.GameArchiveDataAddBanPlayer(_ffi, player.GetRawData());
    }

    public FfiGameRuntimeArchive GetRawData()
    {
        return _ffi;
    }
}