using NoGamepads_Core.Data;
using NoGamepads_Core.Data.Message;
using NoGamepads_Core.Services;
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

    #region Status Management

    public bool LookStatus
    {
        get => nogamepads_data.GameRuntimeGetLockStatus(_ffi);
        set
        {
            if (value)
            {
                nogamepads_data.GameRuntimeLock(_ffi);
            }
            else
            {
                nogamepads_data.GameRuntimeUnlock(_ffi);
            }
        }
    }

    public void CloseGame()
    {
        nogamepads_data.GameRuntimeClose(_ffi);
    }

    #endregion
    
    #region Player Management

    public void Kick(Player player, ServiceType type = ServiceType.Unknown)
    {
        if (IsUsed()) return;
        var serviceType = type == ServiceType.Unknown ? GetServiceType(player) : type;
        nogamepads_data.GameRuntimeKickPlayer(_ffi, player.Borrow(), serviceType.Convert());
    }
    
    public void LetExit(Player player, ServiceType type = ServiceType.Unknown, ExitReason reason = ExitReason.Exit)
    {
        if (IsUsed()) return;
        var serviceType = type == ServiceType.Unknown ? GetServiceType(player) : type;
        nogamepads_data.GameRuntimeLetExit(_ffi, player.Borrow(), serviceType.Convert(), reason.Convert());
    }

    public void Ban(Player player, ServiceType type = ServiceType.Unknown)
    {
        if (IsUsed()) return;
        var serviceType = type == ServiceType.Unknown ? GetServiceType(player) : type;
        nogamepads_data.GameRuntimeBanPlayer(_ffi, player.Borrow(), serviceType.Convert());
    }

    public void Pardon(Player player)
    {
        if (IsUsed()) return;
        nogamepads_data.GameRuntimePardonPlayer(_ffi, player.Borrow());
    }

    public ServiceType GetServiceType(Player player)
    {
        if (IsUsed()) return ServiceType.Unknown;
        return nogamepads_data.GameRuntimeGetServiceType(_ffi, player.Borrow()).Convert();
    }

    public bool IsOnline(Player player)
    {
        if (IsUsed()) return false;
        var result = nogamepads_data.GameRuntimeIsPlayerOnline(_ffi, player.Borrow());
        if (result.Found)
        {
            return result.Result;
        }
        return false;
    } 
    
    public bool IsBanned(Player player)
    {
        if (IsUsed()) return false;
        var result = nogamepads_data.GameRuntimeIsPlayerBanned(_ffi, player.Borrow());
        if (result.Found)
        {
            return result.Result;
        }
        return false;
    } 

    public List<Player>? OnlinePlayers => IsUsed() ? null : nogamepads_data.GameRuntimeGetOnlineList(_ffi).ToPlayerList();
    
    public List<Player>? BannedPlayers => IsUsed() ? null : nogamepads_data.GameRuntimeGetBannedList(_ffi).ToPlayerList();

    #endregion

    #region Message Management

    public ControlEvent? RecentEvent => IsUsed() ? null : new(nogamepads_data.GameRuntimePopControlEvent(_ffi));

    public void SendText(string message, Player player, ServiceType type = ServiceType.Unknown)
    {
        if (IsUsed()) return;
        var serviceType = type == ServiceType.Unknown ? GetServiceType(player) : type;
        nogamepads_data.GameRuntimeSendTextMessage(_ffi, player.Borrow(), serviceType.Convert(), message);
    }

    public void Send(GameMessage message, Player player, ServiceType type = ServiceType.Unknown)
    {
        if (IsUsed()) return;
        var serviceType = type == ServiceType.Unknown ? GetServiceType(player) : type;
        nogamepads_data.GameRuntimeSendMessageTo(_ffi, player.Borrow(), message.Convert(), serviceType.Convert());
    }

    #endregion

    #region Control Management

    public ButtonStatus Button(int key, Player player)
    {
        if (IsUsed()) return ButtonStatus.NotFound();
        var result = nogamepads_data.GameRuntimeGetButtonStatus(_ffi, player.Borrow(), (byte)key);
        if (result == null || !result.Found)
        {
            return ButtonStatus.NotFound();
        }
        return result.Pressed ? ButtonStatus.Press() : ButtonStatus.Release();
    }
    
    public AxisStatus Axis(int key, Player player)
    {
        if (IsUsed()) return AxisStatus.NotFound();
        var result = nogamepads_data.GameRuntimeGetAxis(_ffi, player.Borrow(), (byte)key);
        if (result == null || !result.Found)
        {
            return AxisStatus.NotFound();
        }
        return AxisStatus.Axis((float)result.Axis);
    }
    
    public DirectionStatus Direction(int key, Player player)
    {
        if (IsUsed()) return DirectionStatus.NotFound();
        var result = nogamepads_data.GameRuntimeGetDirection(_ffi, player.Borrow(), (byte)key);
        if (result == null || !result.Found)
        {
            return DirectionStatus.NotFound();
        }
        return DirectionStatus.Direction((float) result.X, (float) result.Y);
    }

    #endregion
    
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