using NoGamepads_Sharp;

namespace NoGamepads_Core.Data.Message;

public struct ControlMessage
{
    public enum Tag
    {
        Message = 0,
        Pressed = 1, 
        Released = 2,
        Axis = 3,
        Direction = 4,
        Exit = 5, 
        Error = 6,
        End = 7
    }

    public Tag MessageTag => _tag;

    private Tag _tag;
    private string _content;
    private int _key;
    private float _x; // Axis or DirectionX
    private float _y;

    public static ControlMessage Message(string messageContent)
    {
        return new ControlMessage
        {
            _tag = Tag.Message,
            _content = messageContent
        };
    }
    
    public static ControlMessage Pressed(int key)
    {
        return new ControlMessage
        {
            _tag = Tag.Pressed,
            _key = key
        };
    }
    
    public static ControlMessage Released(int key)
    {
        return new ControlMessage
        {
            _tag = Tag.Released,
            _key = key
        };
    }

    public static ControlMessage Axis(int key, float x)
    {
        return new ControlMessage
        {
            _tag = Tag.Axis,
            _key = key,
            _x = x
        };
    }
    
    public static ControlMessage Direction(int key, float x, float y)
    {
        return new ControlMessage
        {
            _tag = Tag.Direction,
            _key = key,
            _x = x,
            _y = y
        };
    }
    
    public static ControlMessage Exit()
    {
        return new ControlMessage
        {
            _tag = Tag.Exit,
        };
    }
    
    public static ControlMessage Error()
    {
        return new ControlMessage
        {
            _tag = Tag.Error,
        };
    }
    
    public static ControlMessage End()
    {
        return new ControlMessage
        {
            _tag = Tag.End,
        };
    }

    public FfiControlMessage Parse()
    {
        FfiControlMessage controlMessage = new FfiControlMessage();
        int index = (int) MessageTag;
        FfiControlMessageTag tag = (FfiControlMessageTag) index;
        controlMessage.Tag = tag;
        switch (tag)
        {
            case FfiControlMessageTag.CtrlMsg:
                unsafe { controlMessage.Data = controlMessage.Data with { Message = PtrStringConverter.ToPtr(_content) }; }
                break;
            case FfiControlMessageTag.CtrlPressed:
                controlMessage.Data = controlMessage.Data with { Key = (byte) _key };
                break;
            case FfiControlMessageTag.CtrlReleased:
                controlMessage.Data = controlMessage.Data with { Key = (byte) _key };
                break;
            case FfiControlMessageTag.CtrlAxis:
                FfiKeyAndAxis keyAndAxis = new FfiKeyAndAxis();
                keyAndAxis.Key = (byte) _key;
                keyAndAxis.Axis = _x;
                controlMessage.Data = controlMessage.Data with { KeyAndAxis = keyAndAxis };
                break;
            case FfiControlMessageTag.CtrlDir:
                FfiKeyAndDirection keyAndDirection = new FfiKeyAndDirection();
                keyAndDirection.Key = (byte) _key;
                keyAndDirection.X = _x;
                keyAndDirection.Y = _y;
                controlMessage.Data = controlMessage.Data with { KeyAndDirection = keyAndDirection };
                break;
        }
        return controlMessage;
    }
    
    public static ControlMessage From(FfiControlMessage message)
    {
        switch (message.Tag)
        {
            case FfiControlMessageTag.CtrlMsg:
                unsafe { return Message(PtrStringConverter.ToString(message.Data.Message)); } 
            case FfiControlMessageTag.CtrlPressed:
                return Pressed(message.Data.Key);
            case FfiControlMessageTag.CtrlReleased:
                return Released(message.Data.Key);
            case FfiControlMessageTag.CtrlAxis:
                FfiKeyAndAxis keyAndAxis = message.Data.KeyAndAxis;
                return Axis(keyAndAxis.Key, (float) keyAndAxis.Axis);
            case FfiControlMessageTag.CtrlDir:
                FfiKeyAndDirection keyAndDirection = message.Data.KeyAndDirection;
                return Direction(keyAndDirection.Key, (float) keyAndDirection.X, (float) keyAndDirection.Y);
            case FfiControlMessageTag.CtrlExit:
                return Exit();
            case FfiControlMessageTag.CtrlError:
                return Error();
            case FfiControlMessageTag.CtrlEnd:
                return End();
        }
        return Error();
    }
}