using NoGamepads_Core.Runtime;
using NoGamepads_Sharp;

namespace NoGamepads_Core.Services.Tcp;

public class PadTcpServer
{
    private readonly FfiTcpServerService _serviceFfi;
    private readonly GameRuntime _runtime;
    
    public PadTcpServer(GameRuntime runtime)
    {
        _runtime = runtime;
        _serviceFfi = nogamepads_data.TcpServerBuild(runtime.Borrow());
    }

    public PadTcpServer SetAddressV4(int a0 = 127, int a1 = 0, int a2 = 0, int a3 = 1, int port = 5989)
    {
        nogamepads_data.TcpServerBindAddressV4(_serviceFfi, (byte) a0, (byte) a1, (byte) a2, (byte) a3, ushort.Parse(port + ""));
        return this;
    }
    
    public PadTcpServer SetAddressV6(string address = "::1", int port = 5989)
    {
        nogamepads_data.TcpServerBindAddressV6(_serviceFfi, address, ushort.Parse(port + ""));
        return this;
    }

    public void BlockOn()
    {
        nogamepads_data.TcpServerListeningBlockOn(_serviceFfi);
        _runtime.Use();
    }
}