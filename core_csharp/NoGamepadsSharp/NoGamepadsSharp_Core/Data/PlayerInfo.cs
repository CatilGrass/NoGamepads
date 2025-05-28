namespace NoGamepadsSharp_Data.Data;

public struct PlayerInfo
{
    public PlayerAccount Account;
    public PlayerCustomize Customize;
    
    public struct PlayerCustomize
    {
        public String Nickname;
        public HsvColor Color;
    }
    
    public struct PlayerAccount
    {
        public String Id;
        public String Hash;
    }
}