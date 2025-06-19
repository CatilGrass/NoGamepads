using NoGamepads_Sharp;

namespace NoGamepads_Core.Data.Message;

public struct GameMessage
{
    public enum Tag
    {
        EventTrigger = 0,
        Message = 1,
        LetExit = 2, 
        Error = 3, 
        End = 4
    }

    public Tag MessageTag => _tag;

    private Tag _tag;
    private string _content;
    private int _key;
    private ExitReason _exitReason;

    public static GameMessage EventTrigger(int key)
    {
        return new GameMessage
        {
            _tag = Tag.EventTrigger,
            _key = key
        };
    }
    
    public static GameMessage Message(string messageContent)
    {
        return new GameMessage
        {
            _tag = Tag.Message,
            _content = messageContent
        };
    }
    
    public static GameMessage LetExit(ExitReason exitReason)
    {
        return new GameMessage
        {
            _tag = Tag.LetExit,
            _exitReason = exitReason
        };
    }
    
    public static GameMessage Error()
    {
        return new GameMessage
        {
            _tag = Tag.Error
        };
    }
    
    public static GameMessage End()
    {
        return new GameMessage
        {
            _tag = Tag.End
        };
    }

    public FfiGameMessage Parse()
    {
        FfiGameMessage gameMessage = new FfiGameMessage();
        int index = (int) MessageTag;
        FfiGameMessageTag tag = (FfiGameMessageTag) index;
        gameMessage.Tag = tag;
        switch (tag)
        {
            case FfiGameMessageTag.GameEventTrigger:
                gameMessage.Data = gameMessage.Data with { Key = (byte) _key };
                break;
            case FfiGameMessageTag.GameMsg:
                unsafe { gameMessage.Data = gameMessage.Data with { Message = PtrStringConverter.ToPtr(_content) }; }
                break;
            case FfiGameMessageTag.GameLetExit:
                gameMessage.Data = gameMessage.Data with { ExitReason = _exitReason.Convert()};
                break;
        }
        return gameMessage;
    }

    public static GameMessage From(FfiGameMessage message)
    {
        switch (message.Tag)
        {
            case FfiGameMessageTag.GameEventTrigger:
                return EventTrigger(message.Data.Key);
            case FfiGameMessageTag.GameMsg:
                unsafe { return Message(PtrStringConverter.ToString(message.Data.Message)); } 
            case FfiGameMessageTag.GameLetExit:
                return LetExit(message.Data.ExitReason.Convert());
            case FfiGameMessageTag.GameError:
                return Error();
            case FfiGameMessageTag.GameEnd:
                return End();
        }
        return Error();
    }
}