using NoGamepads_Sharp;

namespace NoGamepads_Core.Data.Message;

public class ControlEvent
{
    public Player Player;
    public ControlMessage Message;
    
    public ControlEvent(FfiControlEvent ffi)
    {
        Player = new Player(ffi.Player);
        Message = ControlMessage.From(ffi.Message);
    }
}