using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class GameArchiveData : IRustDataBorrow<FfiGameRuntimeArchive>, IRustDataUse<FfiGameRuntimeArchive>
{
    private readonly FfiGameRuntimeArchive? _ffi = nogamepads_data.GameArchiveDataNew();
    private bool _used;

    public void AddBannedPlayer(Player player)
    {
        nogamepads_data.GameArchiveDataAddBanPlayer(_ffi, player.Use());
    }

    public FfiGameRuntimeArchive? Borrow()
    {
        return _ffi;
    }

    public bool IsUsed()
    {
        return _used;
    }

    public FfiGameRuntimeArchive? Use()
    {
        var temp = IsUsed() ? null : _ffi;
        _used = true;
        return temp;
    }
}