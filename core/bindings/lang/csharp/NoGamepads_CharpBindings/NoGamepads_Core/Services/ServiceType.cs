using NoGamepads_Sharp;

namespace NoGamepads_Core.Services;

public enum ServiceType
{
    Unknown,
    TcpConnection,
    BluetoothConnection,
    UsbConnection,
}

public static class ServiceTypeConverter
{
    public static FfiServiceType Convert(this ServiceType serviceType)
    {
        if (serviceType == ServiceType.TcpConnection)
            return FfiServiceType.TCPConnection;
        if (serviceType == ServiceType.BluetoothConnection)
            return FfiServiceType.BlueTooth;
        if (serviceType == ServiceType.UsbConnection)
            return FfiServiceType.USB;
        return FfiServiceType.Unknown;
    }
    
    public static ServiceType Convert(this FfiServiceType serviceType)
    {
        if (serviceType == FfiServiceType.TCPConnection)
            return ServiceType.TcpConnection;
        if (serviceType == FfiServiceType.BlueTooth)
            return ServiceType.BluetoothConnection;
        if (serviceType == FfiServiceType.USB)
            return ServiceType.UsbConnection;
        return ServiceType.Unknown;
    }
}