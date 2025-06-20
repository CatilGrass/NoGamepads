using NoGamepads_Core.Data;
using NoGamepads_Core.Data.Message;
using NoGamepads_Sharp;

namespace NoGamepads_Core.Runtime;

public class ControllerRuntime : IRustDataBorrow<FfiControllerRuntime>, IRustDataUse<FfiControllerRuntime>
{
    private readonly FfiControllerRuntime? _ffi;
    private bool _used;

    public ControllerRuntime(ControllerData data)
    {
        _ffi = nogamepads_data.ControllerDataBuildRuntime(data.Borrow());
    }

    public void Close()
    {
        nogamepads_data.ControllerRuntimeClose(_ffi);
    }

    public void Pressed(int key)
    {
        nogamepads_data.ControllerRuntimePressAButton(_ffi, (byte)key);
    }

    public void Released(int key)
    {
        nogamepads_data.ControllerRuntimeReleaseAButton(_ffi, (byte)key);
    }

    public void ChangeAxis(int key, float axis)
    {
        nogamepads_data.ControllerRuntimeChangeAxis(_ffi, (byte)key, axis);
    }

    public void ChangeDirection(int key, float x, float y)
    {
        nogamepads_data.ControllerRuntimeChangeDirection(_ffi, (byte)key, x, y);
    }

    public void SendTextMessage(string message)
    {
        nogamepads_data.ControllerRuntimeSendTextMessage(_ffi, message);
    }

    public void SendMessage(ControlMessage message)
    {
        nogamepads_data.ControllerRuntimeSendMessage(_ffi, message.Parse());
    }

    public GameMessage PopMessage()
    {
        var result = nogamepads_data.ControllerRuntimePop(_ffi);
        return result == null ? GameMessage.Error() : GameMessage.From(result);
    }

    public FfiControllerRuntime? Borrow()
    {
        return _ffi;
    }

    public bool IsUsed()
    {
        return _used;
    }

    public FfiControllerRuntime? Use()
    {
        var temp = IsUsed() ? null : _ffi;
        _used = true;
        return temp;
    }
}