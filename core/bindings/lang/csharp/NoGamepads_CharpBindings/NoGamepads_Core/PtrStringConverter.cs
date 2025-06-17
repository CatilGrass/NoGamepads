using System.Runtime.InteropServices;
using System.Text;

namespace NoGamepads_Core;

public static class PtrStringConverter
{
    public static unsafe string ToString(sbyte* ptr)
    {
        string? str = Marshal.PtrToStringUTF8((IntPtr)ptr);
        return str ?? "";
    }
    
    public static unsafe sbyte* ToPtr(string str)
    {
        byte[] bytes = Encoding.ASCII.GetBytes(str + '\0');
        IntPtr ptr = Marshal.AllocHGlobal(bytes.Length);
        Marshal.Copy(bytes, 0, ptr, bytes.Length);
        sbyte* sbytePtr = (sbyte*)ptr.ToPointer();
        Marshal.FreeHGlobal(ptr);
        return sbytePtr;
    }
}