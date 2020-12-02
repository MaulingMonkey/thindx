// An example super basic shader library

cbuffer ExampleCBuffer {
    float1 scale;
}

export float4 scale4(float4 v) { return v * scale; }

export float4 xyz1(float3 v) { return float4(v, 1.0); }
