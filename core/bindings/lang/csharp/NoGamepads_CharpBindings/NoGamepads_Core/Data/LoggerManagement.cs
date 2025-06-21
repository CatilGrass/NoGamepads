using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public static class LoggerManagement
{
    public static void EnableLogger(int level)
    {
        nogamepads_data.EnableLogger((byte) level);
    }
}