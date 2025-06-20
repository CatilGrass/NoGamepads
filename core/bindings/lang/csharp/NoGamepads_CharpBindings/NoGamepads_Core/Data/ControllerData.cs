using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class ControllerData : IRustDataBorrow<FfiControllerData>, IRustDataUse<FfiControllerData>
{
    private readonly FfiControllerData? _ffi;
    private bool _used;

    public ControllerData(Player player)
    {
        _ffi = nogamepads_data.ControllerDataNew();
        nogamepads_data.ControllerDataBindPlayer(_ffi, player.Use());
    }

    public FfiControllerData? Borrow()
    {
        return _ffi;
    }

    public bool IsUsed()
    {
        return _used;
    }

    public FfiControllerData? Use()
    {
        var temp = IsUsed() ? null : _ffi;
        _used = true;
        return temp;
    }
}