namespace UnrealEngine;

public interface IVector2D
{
    double X { get; set; }
    double Y { get; set; }
}

public interface IVector : IVector2D
{
    double Z { get; set; }
}

public interface IRotator
{
    float Pitch { get; set; }
    float Yaw { get; set; }
    float Roll { get; set; }
}
