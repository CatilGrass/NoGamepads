using NoGamepads_Core.Data;
using NoGamepads_Sharp;

namespace NoGamepads_Core.Runtime;

public class GameRuntime : IRawData<FfiGameRuntime>
{
    private readonly FfiGameRuntime _ffi;

    public GameRuntime(GameData data)
    {
        _ffi = nogamepads_data.GameDataBuildRuntime(data.GetRawData());
    }
    
    public FfiGameRuntime GetRawData()
    {
        return _ffi;
    }
}