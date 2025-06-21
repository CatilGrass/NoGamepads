using NoGamepads_Sharp;

namespace NoGamepads_Core.Services;

public enum ServiceType
{
    Unknown = 0,
    TcpConnection = 1,
    BluetoothConnection = 2,
    UsbConnection = 3,
}

public static class ServiceTypeConverter
{
    public static FfiServiceType Convert(this ServiceType serviceType)
    {
        var index = (int)serviceType;
        return (FfiServiceType)index;
    }
    
    public static ServiceType Convert(this FfiServiceType serviceType)
    {
        var index = (int)serviceType;
        return (ServiceType)index;
    }
}