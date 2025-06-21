namespace NoGamepads_Core.Data;

public struct ButtonStatus
{
    private bool _value;
    
    public bool Found;
    public bool Pressed => _value;
    public bool Released => !_value;

    public static ButtonStatus Press()
    {
        return new ButtonStatus
        {
            Found = true,
            _value = true
        };
    }
    
    public static ButtonStatus Release()
    {
        return new ButtonStatus
        {
            Found = true,
            _value = false
        };
    }
    
    public static ButtonStatus NotFound()
    {
        return new ButtonStatus
        {
            Found = false,
            _value = false
        };
    }
}

public struct AxisStatus
{
    public bool Found;
    public float X;

    public static AxisStatus Axis(float value)
    {
        return new AxisStatus
        {
            Found = true,
            X = value
        };
    }
    
    public static AxisStatus NotFound()
    {
        return new AxisStatus
        {
            Found = false,
            X = 0
        };
    }
}

public struct DirectionStatus
{
    public bool Found;
    public float X;
    public float Y;

    public static DirectionStatus Direction(float x, float y)
    {
        return new DirectionStatus
        {
            Found = true,
            X = x,
            Y = y
        };
    }
    
    public static DirectionStatus NotFound()
    {
        return new DirectionStatus
        {
            Found = false,
            X = 0,
            Y = 0
        };
    }
}