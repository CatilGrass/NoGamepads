using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class Player
{
    private readonly FfiPlayer _ffi;

    public Player(string account, string password)
    {
        _ffi = nogamepads_data.PlayerRegister(account, password);
    }

    public string NickName
    {
        get
        {
            unsafe
            {
                var get = _ffi.Customize;
                return get == null ? "" : PtrStringConverter.ToString(get.Nickname);
            }
        }
        
        set => nogamepads_data.PlayerSetNickname(_ffi, value);
    }
    
    public string Id
    {
        get
        {
            unsafe
            {
                return PtrStringConverter.ToString(_ffi.Account.Id);
            }
        }
    }
    
    public string Hash
    {
        get
        {
            unsafe
            {
                return PtrStringConverter.ToString(_ffi.Account.PlayerHash);
            }
        }
    }

    public int Hue
    {
        get => GetHue();
        set => SetHsv(value, Value, Saturation);
    }
    
    public float Value
    {
        get => GetValue();
        set => SetHsv(Hue, value, Saturation);
    }
    
    public float Saturation
    {
        get => GetSaturation();
        set => SetHsv(Hue, Value, value);
    }
    
    private void SetHsv(int h, float s, float v)
    {
        nogamepads_data.PlayerSetHsv(_ffi, h, s, v);
    }

    private int GetHue()
    {
        return _ffi.Customize?.ColorHue ?? 0;
    }
    
    private float GetValue()
    {
        return (float) (_ffi.Customize?.ColorValue ?? 1f);
    }
    
    private float GetSaturation()
    {
        return (float) (_ffi.Customize?.ColorSaturation ?? 1f);
    }

    public FfiPlayer GetRawPlayer()
    {
        return _ffi;
    }
}