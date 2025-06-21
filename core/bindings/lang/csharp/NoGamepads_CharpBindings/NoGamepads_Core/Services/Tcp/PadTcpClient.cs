using NoGamepads_Core.Runtime;
using NoGamepads_Sharp;

namespace NoGamepads_Core.Services.Tcp;

public class PadTcpClient
{
    private readonly FfiTcpClientService _serviceFfi;
    
    public PadTcpClient(ControllerRuntime runtime)
    {
        _serviceFfi = nogamepads_data.TcpClientBuild(runtime.Borrow());
    }

    public PadTcpClient SetAddressV4(int a0 = 127, int a1 = 0, int a2 = 0, int a3 = 1, int port = 5989)
    {
        nogamepads_data.TcpClientBindAddressV4(_serviceFfi, (byte) a0, (byte) a1, (byte) a2, (byte) a3, (byte) port);
        return this;
    }
    
    public PadTcpClient SetAddressV6(string address = "::1", int port = 5989)
    {
        nogamepads_data.TcpClientBindAddressV6(_serviceFfi, address, (byte) port);
        return this;
    }

    public void Connect()
    {
        nogamepads_data.TcpClientConnect(_serviceFfi);
    }
}