using NoGamepads_Sharp;

namespace NoGamepads_Core.Data;

public static class LoggerManagement
{
    public static void EnableLogger(int level = 0)
    {
        nogamepads_data.EnableLogger((byte) level);
    }
}