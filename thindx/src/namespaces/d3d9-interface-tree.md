<!-- Breaks horribly in rust-analyzer's intellisense -->

[Unknown] - The root type from which all sane COM types derive<br>
&nbsp;  ├─ [Direct3D]\[[Ex](Direct3DEx)\] - Core factories for creating [Device]\[[Ex](Direct3DEx)\]s and [SwapChain]\[[Ex](SwapChainEx)\]s<span style="opacity: 25%"><br>
&nbsp;  ├─ [Device]\[[Ex](DeviceEx)\] - Create resources & dispatches rendering for an individual GPU<br>
&nbsp;  ├─ [SwapChain]\[[Ex](SwapChainEx)\] - Manages swapping buffers for individual "views" (monitors/windows)</span><br>
&nbsp;  ├─ [Resource]<br>
&nbsp;  │      ├─ [Surface] - 2D buffer of pixels<br>
&nbsp;  │      ├─ [BaseTexture] - A GPU-friendly collection of pixels<br>
&nbsp;  │      │      ├─ [CubeTexture] - 6-sided 2D texture<br>
&nbsp;  │      │      ├─ [Texture] - 2D texture<br>
&nbsp;  │      │      └─ [VolumeTexture] - Dense 3D texture<br>
&nbsp;  │      ├─ [IndexBuffer] - An [index buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) indexes verticies in a [VertexBuffer] when rendering.<br>
&nbsp;  │      └─ [VertexBuffer] - A [vertex buffer](https://docs.microsoft.com/en-us/windows/win32/direct3d9/rendering-from-vertex-and-index-buffers#scenario-2-drawing-two-triangles-with-indexing) typically contains points of a mesh to be rendered.<br>
&nbsp;  ├─ <strike>[Resource]</strike> You'd expect these to be resources, but they aren't - they derive from [Unknown].<br>
&nbsp;  │      ├─ [Volume] - 3D buffer of pixels<br>
&nbsp;  │      ├─ [PixelShader] - per-fragment GPU program<br>
&nbsp;  │      └─ [VertexShader] - per-vertex GPU program<br>
&nbsp;  ├─ [Query] - An asyncronous GPU query for [occlusion or other information](https://docs.microsoft.com/en-us/windows/win32/direct3d9/queries).<br>
&nbsp;  ├─ [StateBlock] - Used to [capture/save and restore](https://docs.microsoft.com/en-us/windows/win32/direct3d9/state-blocks-save-and-restore-state) changes to [Device] state.<br>
&nbsp;  └─ [VertexDeclaration] - Describes the layout of the contents of a [VertexBuffer]<br>
