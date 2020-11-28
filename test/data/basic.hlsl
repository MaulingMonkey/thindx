// An example super basic shader set

struct Vertex {
    float4 position : POSITION0;
    float4 color    : COLOR0;
};

struct VsToPs {
    float4 color    : COLOR0;
    float4 position : SV_POSITION;
};

struct Fragment {
    float4 color    : SV_TARGET0;
};

VsToPs vs_main(Vertex v) {
    VsToPs o;
    o.color     = v.color;
    o.position  = v.position;
    return o;
}

Fragment ps_main(VsToPs i) {
    Fragment o;
    o.color     = i.color;
    return o;
}
