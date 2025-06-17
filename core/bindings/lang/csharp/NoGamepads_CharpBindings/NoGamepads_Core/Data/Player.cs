using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class Player
{
    private FfiPlayer _ffi = nogamepads_data.PlayerRegister("Fuck", "AAAAA");

    public String name
    {
        get
        {
            unsafe
            {
                return PtrStringConverter.ToString(_ffi.Account.PlayerHash);
            }
        }
    }
}