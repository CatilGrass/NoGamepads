using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class GameData : IRawData<FfiGameData>
{
    private readonly FfiGameData _ffi = nogamepads_data.GameDataNew();

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
        nogamepads_data.GameDataLoadArchive(_ffi, gameArchiveData.GetRawData());
    }
    
    public FfiGameData GetRawData()
    {
        return _ffi;
    }
}