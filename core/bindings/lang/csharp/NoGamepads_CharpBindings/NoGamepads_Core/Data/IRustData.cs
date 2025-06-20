namespace NoGamepads_Core.Data;

public interface IRustDataBorrow<out T>
{
    T? Borrow();
}

public interface IRustDataUse<out T>
{
    bool IsUsed();
    
    T? Use();
}