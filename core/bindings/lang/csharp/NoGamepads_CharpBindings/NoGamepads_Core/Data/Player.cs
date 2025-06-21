using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public class Player : IRustDataBorrow<FfiPlayer>, IRustDataUse<FfiPlayer>
{
    private readonly FfiPlayer? _ffi;
    private bool _used;

    public Player(string account, string password)
    {
        _ffi = nogamepads_data.PlayerRegister(account, password);
    }
    
    public Player(FfiPlayer ffi)
    {
        _ffi = ffi;
    }

    public string NickName
    {
        get
        {
            unsafe
            {
                var get = _ffi?.Customize;
                return get == null || IsUsed() ? "" : PtrStringConverter.ToString(get.Nickname);
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
                var get = _ffi?.Account;
                return get == null || IsUsed() ? "" : PtrStringConverter.ToString(get.Id);
            }
        }
    }
    
    public string Hash
    {
        get
        {
            unsafe
            {
                var get = _ffi?.Account;
                return get == null || IsUsed() ? "" : PtrStringConverter.ToString(get.PlayerHash);
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
        if (IsUsed())
            return 0;
        return _ffi?.Customize?.ColorHue ?? 0;
    }
    
    private float GetValue()
    {
        if (IsUsed())
            return 1f;
        return (float) (_ffi?.Customize?.ColorValue ?? 1f);
    }
    
    private float GetSaturation()
    {
        if (IsUsed())
            return 1f;
        return (float) (_ffi?.Customize?.ColorSaturation ?? 1f);
    }

    public FfiPlayer? Borrow()
    {
        return _ffi;
    }
    
    public bool IsUsed()
    {
        return _used;
    }

    public FfiPlayer? Use()
    {
        var temp = IsUsed() ? null : _ffi;
        _used = true;
        return temp;
    }
}