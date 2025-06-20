using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class GameData : IRustDataBorrow<FfiGameData>, IRustDataUse<FfiGameData>
{
    private readonly FfiGameData? _ffi = nogamepads_data.GameDataNew();
    private bool _used;

    public string Name
    {
        set => nogamepads_data.GameDataSetNameInfo(_ffi, value);
    }
    
    public string Version
    {
        set => nogamepads_data.GameDataSetVersionInfo(_ffi, value);
    }
    
    public void EditInfo(string infoName, string value)
    {
        nogamepads_data.GameDataAddInfo(_ffi, infoName, value);
    }
    
    public void LoadArchive(GameArchiveData gameArchiveData)
    {
        nogamepads_data.GameDataLoadArchive(_ffi, gameArchiveData.Use());
    }

    public FfiGameData? Borrow()
    {
        return _ffi;
    }

    public bool IsUsed()
    {
        return _used;
    }

    public FfiGameData? Use()
    {
        var temp = IsUsed() ? null : _ffi;
        _used = true;
        return temp;
    }
}