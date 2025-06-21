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
        _ffi = nogamepads_data.ControllerDataBuildRuntime(data.Use());
    }

    public GameMessage RecentMessage
    {
        get
        {
            if (IsUsed()) return GameMessage.End();
            var result = nogamepads_data.ControllerRuntimePop(_ffi);
            return result == null ? GameMessage.Error() : GameMessage.From(result);
        }
    }

    public void Close()
    {
        if (IsUsed()) return;
        nogamepads_data.ControllerRuntimeClose(_ffi);
    }

    public void Press(int key)
    {
        if (IsUsed()) return;
        nogamepads_data.ControllerRuntimePressAButton(_ffi, (byte)key);
    }

    public void Release(int key)
    {
        if (IsUsed()) return;
        nogamepads_data.ControllerRuntimeReleaseAButton(_ffi, (byte)key);
    }

    public void ChangeAxis(int key, float axis)
    {
        if (IsUsed()) return;
        nogamepads_data.ControllerRuntimeChangeAxis(_ffi, (byte)key, axis);
    }

    public void ChangeDirection(int key, float x, float y)
    {
        if (IsUsed()) return;
        nogamepads_data.ControllerRuntimeChangeDirection(_ffi, (byte)key, x, y);
    }

    public void SendText(string message)
    {
        if (IsUsed()) return;
        nogamepads_data.ControllerRuntimeSendTextMessage(_ffi, message);
    }

    public void Send(ControlMessage message)
    {
        if (IsUsed()) return;
        nogamepads_data.ControllerRuntimeSendMessage(_ffi, message.Convert());
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