using NoGamepads_Sharp;

namespace NoGamepads_Core.Data.Message;

public enum ExitReason
{
    Exit = 0,
    GameOverReason = 1,
    ServerClosedReason = 2,
    YouAreKickedReason = 3,
    YouAreBannedReason = 4,
    ErrorReason = 5
}

public static class ExitReasonConvertor
{
    public static ExitReason Convert(this FfiExitReason reason)
    {
        int index = (int) reason;
        return (ExitReason) index;
    } 
    
    public static FfiExitReason Convert(this ExitReason reason)
    {
        int index = (int) reason;
        return (FfiExitReason) index;
    } 
}