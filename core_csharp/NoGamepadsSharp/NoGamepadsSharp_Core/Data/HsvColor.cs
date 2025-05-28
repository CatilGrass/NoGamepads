namespace NoGamepadsSharp_Data.Data;

public struct HsvColor
{
    private int _h;
    private float _s;
    private float _v;

    public HsvColor(int h, float s, float v)
    {
        _h = h;
        _s = s;
        _v = v;
    }

    public HsvColor(float r, float g, float b)
    {
        Converter.ToHsv(r, g, b, out int h, out float s, out float v);
        _h = h;
        _s = s;
        _v = v;
    }
    
    public int hue
    {
        get => _h;
        set => _h = int.Clamp(value, 0, 360);
    }

    public float saturation
    {
        get => _s;
        set => _s = float.Clamp(value, 0, 1);
    }

    public float value
    {
        get => _v;
        set => _v = float.Clamp(value, 0, 1);
    }

    public float red
    {
        get
        {
            Converter.ToRgb(_h, _s, _v, out float f, out _, out _);
            return f;
        }
        set
        {
            Converter.ToRgb(_h, _s, _v, out float r, out float g, out float b);
            r = float.Clamp(value, 0, 1);
            Converter.ToHsv(r, g, b, out int h, out float s, out float v);
            _h = h;
            _s = s;
            _v = v;
        }
    }
    
    public float green
    {
        get
        {
            Converter.ToRgb(_h, _s, _v, out _, out float f, out _);
            return f;
        }
        set
        {
            Converter.ToRgb(_h, _s, _v, out float r, out float g, out float b);
            g = float.Clamp(value, 0, 1);
            Converter.ToHsv(r, g, b, out int h, out float s, out float v);
            _h = h;
            _s = s;
            _v = v;
        }
    }
    
    public float blue
    {
        get
        {
            Converter.ToRgb(_h, _s, _v, out _, out _, out float f);
            return f;
        }
        set
        {
            Converter.ToRgb(_h, _s, _v, out float r, out float g, out float b);
            b = float.Clamp(value, 0, 1);
            Converter.ToHsv(r, g, b, out int h, out float s, out float v);
            _h = h;
            _s = s;
            _v = v;
        }
    }

    public class Converter
    {
        public static void ToRgb(int h, float s, float v, out float r, out float g, out float b)
        {
            h = (h % 360 + 360) % 360;
        
            if (s <= 0.0f)
            {
                r = g = b = v;
                return;
            }

            float hh = h / 60.0f;
            int i = (int)hh;
            float ff = hh - i;
            float p = v * (1.0f - s);
            float q = v * (1.0f - s * ff);
            float t = v * (1.0f - s * (1.0f - ff));

            switch (i)
            {
                case 0: r = v; g = t; b = p; break;
                case 1: r = q; g = v; b = p; break;
                case 2: r = p; g = v; b = t; break;
                case 3: r = p; g = q; b = v; break;
                case 4: r = t; g = p; b = v; break;
                default: r = v; g = p; b = q; break;
            }
        }

        public static void ToHsv(float r, float g, float b, out int h, out float s, out float v)
        {
            float min = Math.Min(Math.Min(r, g), b);
            float max = Math.Max(Math.Max(r, g), b);
            float delta = max - min;

            v = max;
        
            if (delta <= 1e-6f)
            {
                h = 0;
                s = 0.0f;
                return;
            }

            s = delta / max;
            float hh;

            if (Math.Abs(r - max) < 0.001f)
                hh = (g - b) / delta;
            else if (Math.Abs(g - max) < 0.001f)
                hh = (b - r) / delta + 2.0f;
            else
                hh = (r - g) / delta + 4.0f;

            hh *= 60.0f;
            if (hh < 0.0f) hh += 360.0f;
        
            h = (int)Math.Round(hh);
            h = (h % 360 + 360) % 360;
        }
    }
}