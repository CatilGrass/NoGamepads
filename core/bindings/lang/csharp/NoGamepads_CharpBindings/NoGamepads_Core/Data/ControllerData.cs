using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class ControllerData : IRawData<FfiControllerData>
{
    private readonly FfiControllerData _ffi;

    public ControllerData(Player player)
    {
        _ffi = nogamepads_data.ControllerDataNew();
        nogamepads_data.ControllerDataBindPlayer(_ffi, player.GetRawData());
    }
    
    public FfiControllerData GetRawData()
    {
        return _ffi;
    }
}