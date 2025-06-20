using NoGamepads_Core.Data;
using NoGamepads_Sharp;

namespace NoGamepads_Core.Runtime;

public class GameRuntime : IRustDataBorrow<FfiGameRuntime>, IRustDataUse<FfiGameRuntime>
{
    private readonly FfiGameRuntime? _ffi;
    private bool _used;

    public GameRuntime(GameData data)
    {
        _ffi = nogamepads_data.GameDataBuildRuntime(data.Borrow());
    }
    
    public FfiGameRuntime? Borrow()
    {
        return _ffi;
    }

    public bool IsUsed()
    {
        return _used;
    }

    public FfiGameRuntime? Use()
    {
        var temp = IsUsed() ? null : _ffi;
        _used = true;
        return temp;
    }
}