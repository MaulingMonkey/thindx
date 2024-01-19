#![allow(dead_code)] // TODO: remove

use crate::*;
use crate::d3d9::*;

use abibool::bool32;

use bytemuck::{Zeroable, Pod};

use winapi::shared::d3d9::IDirect3DDevice9;
use winapi::shared::d3d9types::*;
use winapi::shared::windef::RECT;
use winapi::um::unknwnbase::IUnknown;
use winapi::um::wingdi::*;

use winresult::*;

use std::ptr::*;

pub(crate) const MAX_BUFFER_ALLOC : u32 = 0xFFFF_0000;

// TODO: support for Device s in doc comment examples (via dev crate?)
// TODO: fuzz / torture-test Device operations in randomized combinations for odd interactions


/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
/// Core interface used for general rendering, resource creation, etc.
///
/// # Table of Contents
/// | Topic                                         | Overview |
/// | --------------------------------------------- | -------- |
/// | [Common](#common)                             | ...
/// | [Drawing](#drawing)                           | Draw primitives
/// | [Buffers](#buffers)                           | Bind/Create/Update [IndexBuffer]s and [VertexBuffer]s
/// | [Queries](#queries)                           | Create/Check Occlusion and other [Query]s
/// | [Shaders](#shaders)                           | Bind/Create [PixelShader]s and [VertexShader]s
/// | [StateBlocks](#stateblocks)                   | Create/Capture/Replay Direct3D states via [StateBlock]s
/// | [Surfaces](#surfaces)                         | Bind/Create [Surface]s for back buffers, render targets, depth stencil, etc.
/// | [SwapChains](#swapchains)                     | Create [SwapChain]s / [SwapChainEx]s for multi-window rendering
/// | [Textures](#textures)                         | Bind/Create/Update [Texture]s, [CubeTexture]s, and [VolumeTexture]s
/// | [VertexDeclarations](#vertexdeclarations)     | Bind/Create [VertexDeclaration]s for describing [VertexBuffer] layouts
/// | [Miscellanious](#miscellanious)               | Metadata, etc.
/// | [Lighting](#lighting-16-bit)                  | Configure (and query) [Light]ing
/// | [Viewports](#viewports)                       | Configure (and query) the [Viewport]
#[derive(Clone)] #[repr(transparent)]
pub struct Device(pub(crate) mcom::Rc<IDirect3DDevice9>);

unsafe impl AsSafe<IUnknown         > for Device { fn as_safe(&self) -> &IUnknown           { &**self.0 } }
unsafe impl AsSafe<IDirect3DDevice9 > for Device { fn as_safe(&self) -> &IDirect3DDevice9   { &*self.0 } }



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
/// IDirect3DDevice9 extension methods
///
/// ### Methods
/// | thindx                                                                    | learn.microsoft.com            | Description |
/// | ------------------------------------------------------------------------- | ----------------------------- | ----------- |
/// | [begin_scene](Self::begin_scene)                                          | [BeginScene]                  | Begins a scene.
/// | [begin_state_block](Self::begin_state_block)                              | [BeginStateBlock]             | Signals Direct3D to begin recording a device-state block.
/// | [clear](Self::clear)                                                      | [Clear]                       | Clears one or more surfaces such as a render target, multiple render targets, a stencil buffer, and a depth buffer.
/// | [color_fill](Self::color_fill)                                            | [ColorFill]                   | Allows an application to fill a rectangular area of a [Pool::Default] surface with a specified color.
/// | [create_additional_swap_chain](Self::create_additional_swap_chain)        | [CreateAdditionalSwapChain]   | Creates an additional swap chain for rendering multiple views.
/// | [create_cube_texture](Self::create_cube_texture)                          | [CreateCubeTexture]           | Creates a cube texture resource.
/// | [create_depth_stencil_surface](Self::create_depth_stencil_surface)        | [CreateDepthStencilSurface]   | Creates a depth-stencil resource.
/// | [create_index_buffer](Self::create_index_buffer)                          | [CreateIndexBuffer]           | Creates an index buffer.
/// | [create_offscreen_plain_surface](Self::create_offscreen_plain_surface)    | [CreateOffscreenPlainSurface] | Create an off-screen surface.
/// | [create_pixel_shader](Self::create_pixel_shader)                          | [CreatePixelShader]           | Creates a pixel shader.
/// | [create_query](Self::create_query)                                        | [CreateQuery]                 | Creates a status query.
/// | [create_render_target](Self::create_render_target)                        | [CreateRenderTarget]          | Creates a render-target surface.
/// | [create_state_block](Self::create_state_block)                            | [CreateStateBlock]            | Creates a new state block that contains the values for all device states, vertex-related states, or pixel-related states.
/// | [create_texture](Self::create_texture)                                    | [CreateTexture]               | Creates a texture resource.
/// | [create_vertex_buffer](Self::create_vertex_buffer)                        | [CreateVertexBuffer]          | Creates a vertex buffer.
/// | [create_vertex_declaration](Self::create_vertex_declaration)              | [CreateVertexDeclaration]     | Create a vertex shader declaration from the device and the vertex elements.
/// | [create_vertex_shader](Self::create_vertex_shader)                        | [CreateVertexShader]          | Creates a vertex shader.
/// | [create_volume_texture](Self::create_volume_texture)                      | [CreateVolumeTexture]         | Creates a volume texture resource.
/// | [delete_patch](Self::delete_patch)                                        | [DeletePatch]                 | Frees a cached high-order patch.
/// | [draw_indexed_primitive](Self::draw_indexed_primitive)                    | [DrawIndexedPrimitive]        | Based on indexing, renders the specified geometric primitive into an array of vertices.
/// | [draw_indexed_primitive_up](Self::draw_indexed_primitive_up)              | [DrawIndexedPrimitiveUP]      | Renders the specified geometric primitive with data specified by a user memory pointer.
/// | [draw_primitive](Self::draw_primitive)                                    | [DrawPrimitive]               | Renders a sequence of nonindexed, geometric primitives of the specified type from the current set of data input streams.
/// | [draw_primitive_up](Self::draw_primitive_up)                              | [DrawPrimitiveUP]             | Renders data specified by a user memory pointer as a sequence of geometric primitives of the specified type.
/// | [draw_rect_patch](Self::draw_rect_patch)                                  | [DrawRectPatch]               | Draws a rectangular patch using the currently set streams.
/// | [draw_tri_patch](Self::draw_tri_patch)                                    | [DrawTriPatch]                | Draws a triangular patch using the currently set streams.
/// | [end_scene](Self::end_scene)                                              | [EndScene]                    | Ends a scene that was begun by calling IDirect3DDevice9::BeginScene.
/// | [end_state_block](Self::end_state_block)                                  | [EndStateBlock]               | Signals Direct3D to stop recording a device-state block and retrieve a pointer to the state block interface.
/// | [evict_managed_resources](Self::evict_managed_resources)                  | [EvictManagedResources]       | Evicts all managed resources, including both Direct3D and driver-managed resources.
/// | [get_available_texture_mem](Self::get_available_texture_mem)              | [GetAvailableTextureMem]      | Returns an estimate of the amount of available texture memory.
/// | [get_back_buffer](Self::get_back_buffer)                                  | [GetBackBuffer]               | Retrieves a back buffer from the device's swap chain.
/// | [get_clip_plane](Self::get_clip_plane)                                    | [GetClipPlane]                | Retrieves the coefficients of a user-defined clipping plane for the device.
/// | [get_clip_status](Self::get_clip_status)                                  | [GetClipStatus]               | Retrieves the clip status.
/// | [get_creation_parameters](Self::get_creation_parameters)                  | [GetCreationParameters]       | Retrieves the creation parameters of the device.
/// | [get_current_texture_palette](Self::get_current_texture_palette)          | [GetCurrentTexturePalette]    | Retrieves the current texture palette.
/// | [get_depth_stencil_surface](Self::get_depth_stencil_surface)              | [GetDepthStencilSurface]      | Gets the depth-stencil surface owned by the Direct3DDevice object.
/// | [get_device_caps](Self::get_device_caps)                                  | [GetDeviceCaps]               | Retrieves the capabilities of the rendering device.
/// | [get_direct3d](Self::get_direct3d)                                        | [GetDirect3D]                 | Returns an interface to the instance of the Direct3D object that created the device.
/// | [get_display_mode](Self::get_display_mode)                                | [GetDisplayMode]              | Retrieves the display mode's spatial resolution, color resolution, and refresh frequency.
/// | [get_front_buffer_data](Self::get_front_buffer_data)                      | [GetFrontBufferData]          | Generates a copy of the device's front buffer and places that copy in a system memory buffer provided by the application.
/// | [get_fvf](Self::get_fvf)                                                  | [GetFVF]                      | Gets the fixed vertex function declaration.
/// | [get_gamma_ramp](Self::get_gamma_ramp)                                    | [GetGammaRamp]                | Retrieves the gamma correction ramp for the swap chain.
/// | [get_indices](Self::get_indices)                                          | [GetIndices]                  | Retrieves index data.
/// | [get_light](Self::get_light)                                              | [GetLight]                    | Retrieves a set of lighting properties that this device uses.
/// | [get_light_32](Self::get_light_32)                                        | [GetLight]                    | Retrieves a set of lighting properties that this device uses.
/// | [get_light_enable](Self::get_light_enable)                                | [GetLightEnable]              | Retrieves the activity status - enabled or disabled - for a set of lighting parameters within a device.
/// | [get_light_enable_32](Self::get_light_enable_32)                          | [GetLightEnable]              | Retrieves the activity status - enabled or disabled - for a set of lighting parameters within a device.
/// | [get_material](Self::get_material)                                        | [GetMaterial]                 | Retrieves the current material properties for the device.
/// | [get_npatch_mode](Self::get_npatch_mode)                                  | [GetNPatchMode]               | Gets the N-patch mode segments.
/// | [get_number_of_swap_chains](Self::get_number_of_swap_chains)              | [GetNumberOfSwapChains]       | Gets the number of implicit swap chains.
/// | [get_palette_entries_unchecked](Self::get_palette_entries_unchecked)      | [GetPaletteEntries]           | Retrieves palette entries.
/// | [get_pixel_shader](Self::get_pixel_shader)                                | [GetPixelShader]              | Retrieves the currently set pixel shader.
/// | [get_pixel_shader_constant_b](Self::get_pixel_shader_constant_b)          | [GetPixelShaderConstantB]     | Gets a Boolean shader constant.
/// | [get_pixel_shader_constant_f](Self::get_pixel_shader_constant_f)          | [GetPixelShaderConstantF]     | Gets a floating-point shader constant.
/// | [get_pixel_shader_constant_i](Self::get_pixel_shader_constant_i)          | [GetPixelShaderConstantI]     | Gets an integer shader constant.
/// | [get_raster_status](Self::get_raster_status)                              | [GetRasterStatus]             | Returns information describing the raster of the monitor on which the swap chain is presented.
/// | [get_render_state](Self::get_render_state)                                | [GetRenderState]              | Retrieves a render-state value for a device.
/// | [get_render_target](Self::get_render_target)                              | [GetRenderTarget]             | Retrieves a render-target surface.
/// | [get_render_target_data](Self::get_render_target_data)                    | [GetRenderTargetData]         | Copies the render-target data from device memory to system memory.
/// | [get_sampler_state_untyped](Self::get_sampler_state_untyped)              | [GetSamplerState]             | Gets the sampler state value.
/// | [get_scissor_rect](Self::get_scissor_rect)                                | [GetScissorRect]              | Gets the scissor rectangle.
/// | [get_software_vertex_processing](Self::get_software_vertex_processing)    | [GetSoftwareVertexProcessing] | Gets the vertex processing (hardware or software) mode.
/// | [get_stream_source](Self::get_stream_source)                              | [GetStreamSource]             | Retrieves a vertex buffer bound to the specified data stream.
/// | [get_stream_source_freq](Self::get_stream_source_freq)                    | [GetStreamSourceFreq]         | Gets the stream source frequency divider value.
/// | [get_swap_chain](Self::get_swap_chain)                                    | [GetSwapChain]                | Gets a pointer to a swap chain.
/// | [get_texture](Self::get_texture)                                          | [GetTexture]                  | Retrieves a texture assigned to a stage for a device.
/// | [get_texture_stage_state](Self::get_texture_stage_state)                  | [GetTextureStageState]        | Retrieves a state value for an assigned texture.
/// | [get_transform](Self::get_transform)                                      | [GetTransform]                | Retrieves a matrix describing a transformation state.
/// | [get_vertex_declaration](Self::get_vertex_declaration)                    | [GetVertexDeclaration]        | Gets a vertex shader declaration.
/// | [get_vertex_shader](Self::get_vertex_shader)                              | [GetVertexShader]             | Retrieves the currently set vertex shader.
/// | [get_vertex_shader_constant_b](Self::get_vertex_shader_constant_b)        | [GetVertexShaderConstantB]    | Gets a Boolean vertex shader constant.
/// | [get_vertex_shader_constant_f](Self::get_vertex_shader_constant_f)        | [GetVertexShaderConstantF]    | Gets a floating-point vertex shader constant.
/// | [get_vertex_shader_constant_i](Self::get_vertex_shader_constant_i)        | [GetVertexShaderConstantI]    | Gets an integer vertex shader constant.
/// | [get_viewport](Self::get_viewport)                                        | [GetViewport]                 | Retrieves the viewport parameters currently set for the device.
/// | [light_enable](Self::light_enable)                                        | [LightEnable]                 | Enables or disables a set of lighting parameters within a device.
/// | [multiply_transform](Self::multiply_transform)                            | [MultiplyTransform]           | Multiplies a device's world, view, or projection matrices by a specified matrix.
/// | [present](Self::present)                                                  | [Present]                     | Presents the contents of the next buffer in the sequence of back buffers owned by the device.
/// | [process_vertices](Self::process_vertices)                                | [ProcessVertices]             | Applies the vertex processing defined by the vertex shader to the set of input data streams, generating a single stream of interleaved vertex data to the destination vertex buffer.
/// | [reset](Self::reset)                                                      | [Reset]                       | Resets the type, size, and format of the swap chain.
/// | [set_clip_plane](Self::set_clip_plane)                                    | [SetClipPlane]                | Sets the coefficients of a user-defined clipping plane for the device.
/// | [set_clip_status](Self::set_clip_status)                                  | [SetClipStatus]               | Sets the clip status.
/// | [set_current_texture_palette_unchecked](Self::set_current_texture_palette_unchecked) | [SetCurrentTexturePalette] | Sets the current texture palette.
/// | [set_cursor_position](Self::set_cursor_position)                          | [SetCursorPosition]           | Sets the cursor position and update options.
/// | [set_cursor_properties](Self::set_cursor_properties)                      | [SetCursorProperties]         | Sets properties for the cursor.
/// | [set_depth_stencil_surface](Self::set_depth_stencil_surface)              | [SetDepthStencilSurface]      | Sets the depth stencil surface.
/// | [set_dialog_box_mode](Self::set_dialog_box_mode)                          | [SetDialogBoxMode]            | This method allows the use of GDI dialog boxes in full-screen mode applications.
/// | [set_fvf](Self::set_fvf)                                                  | [SetFVF]                      | Sets the current vertex stream declaration.
/// | [set_gamma_ramp](Self::set_gamma_ramp)                                    | [SetGammaRamp]                | Sets the gamma correction ramp for the implicit swap chain. This method will affect the entire screen (not just the active window if you are running in windowed mode).
/// | [set_indices](Self::set_indices)                                          | [SetIndices]                  | Sets index data.
/// | [set_light](Self::set_light)                                              | [SetLight]                    | Assigns a set of lighting properties for this device.
/// | [set_light_32_unchecked](Self::set_light_32_unchecked)                    | [SetLight]                    | Like [set_light](Self::set_light), but may overflow/crash if `index` >= `0x1000_0000 - 8` even as recently as Windows 10
/// | [set_material](Self::set_material)                                        | [SetMaterial]                 | Sets the material properties for the device.
/// | [set_npatch_mode](Self::set_npatch_mode)                                  | [SetNPatchMode]               | Enable or disable N-patches.
/// | [set_palette_entries](Self::set_palette_entries)                          | [SetPaletteEntries]           | Sets palette entries.
/// | [set_pixel_shader](Self::set_pixel_shader)                                | [SetPixelShader]              | Sets the current pixel shader to a previously created pixel shader.
/// | [set_pixel_shader_constant_b](Self::set_pixel_shader_constant_b)          | [SetPixelShaderConstantB]     | Sets a Boolean shader constant.
/// | [set_pixel_shader_constant_f](Self::set_pixel_shader_constant_f)          | [SetPixelShaderConstantF]     | Sets a floating-point shader constant.
/// | [set_pixel_shader_constant_i](Self::set_pixel_shader_constant_i)          | [SetPixelShaderConstantI]     | Sets an integer shader constant.
/// | [set_render_state](Self::set_render_state)                                | [SetRenderState]              | Sets a single device render-state parameter.
/// | [set_render_target](Self::set_render_target)                              | [SetRenderTarget]             | Sets a new color buffer for the device.
/// | [set_sampler_state](Self::set_sampler_state)                              | [SetSamplerState]             | Sets the sampler state value.
/// | [set_scissor_rect](Self::set_scissor_rect)                                | [SetScissorRect]              | Sets the scissor rectangle.
/// | [set_software_vertex_processing](Self::set_software_vertex_processing)    | [SetSoftwareVertexProcessing] | Use this method to switch between software and hardware vertex processing.
/// | [set_stream_source](Self::set_stream_source)                              | [SetStreamSource]             | Binds a vertex buffer to a device data stream. For more information, see Setting the Stream Source (Direct3D 9).
/// | [set_stream_source_freq](Self::set_stream_source_freq)                    | [SetStreamSourceFreq]         | Sets the stream source frequency divider value. This may be used to draw several instances of geometry.
/// | [set_texture](Self::set_texture)                                          | [SetTexture]                  | Assigns a texture to a stage for a device.
/// | [set_texture_stage_state](Self::set_texture_stage_state)                  | [SetTextureStageState]        | Sets the state value for the currently assigned texture.
/// | [set_transform](Self::set_transform)                                      | [SetTransform]                | Sets a single device transformation-related state.
/// | [set_vertex_declaration](Self::set_vertex_declaration)                    | [SetVertexDeclaration]        | Sets a Vertex Declaration (Direct3D 9).
/// | [set_vertex_shader](Self::set_vertex_shader)                              | [SetVertexShader]             | Sets the vertex shader.
/// | [set_vertex_shader_constant_b](Self::set_vertex_shader_constant_b)        | [SetVertexShaderConstantB]    | Sets a Boolean vertex shader constant.
/// | [set_vertex_shader_constant_f](Self::set_vertex_shader_constant_f)        | [SetVertexShaderConstantF]    | Sets a floating-point vertex shader constant.
/// | [set_vertex_shader_constant_i](Self::set_vertex_shader_constant_i)        | [SetVertexShaderConstantI]    | Sets an integer vertex shader constant.
/// | [set_viewport](Self::set_viewport)                                        | [SetViewport]                 | Sets the viewport parameters for the device.
/// | [show_cursor](Self::show_cursor)                                          | [ShowCursor]                  | Displays or hides the cursor.
/// | [stretch_rect](Self::stretch_rect)                                        | [StretchRect]                 | Copy the contents of the source rectangle to the destination rectangle. The source rectangle can be stretched and filtered by the copy. This function is often used to change the aspect ratio of a video stream.
/// | [test_cooperative_level](Self::test_cooperative_level)                    | [TestCooperativeLevel]        | Reports the current cooperative-level status of the Direct3D device for a windowed or full-screen application.
/// | [update_surface](Self::update_surface)                                    | [UpdateSurface]               | Copies rectangular subsets of pixels from one surface to another.
/// | [update_texture](Self::update_texture)                                    | [UpdateTexture]               | Updates the dirty portions of a texture.
/// | [validate_device](Self::validate_device)                                  | [ValidateDevice]              | Reports the device's ability to render the current texture-blending operations and arguments in a single pass.
/// | &nbsp;
/// | <span style="opacity: 25%">N/A</span>                                     | [BeginScene], [EndScene]      | Scoped scenes?
/// | <span style="opacity: 25%">N/A</span>                                     | [BeginStateBlock], [EndStateBlock]    | Scoped state blocks?
///
/// [BeginScene]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginscene
/// [BeginStateBlock]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginstateblock
/// [Clear]:                        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-clear
/// [ColorFill]:                    https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-colorfill
/// [CreateAdditionalSwapChain]:    https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createadditionalswapchain
/// [CreateCubeTexture]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture
/// [CreateDepthStencilSurface]:    https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createdepthstencilsurface
/// [CreateIndexBuffer]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer
/// [CreateOffscreenPlainSurface]:  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createoffscreenplainsurface
/// [CreatePixelShader]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createpixelshader
/// [CreateQuery]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createquery
/// [CreateRenderTarget]:           https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createrendertarget
/// [CreateStateBlock]:             https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createstateblock
/// [CreateTexture]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture
/// [CreateVertexBuffer]:           https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer
/// [CreateVertexDeclaration]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexdeclaration
/// [CreateVertexShader]:           https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexshader
/// [CreateVolumeTexture]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvolumetexture
/// [DeletePatch]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-deletepatch
/// [DrawIndexedPrimitive]:         https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitive
/// [DrawIndexedPrimitiveUP]:       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitiveup
/// [DrawPrimitive]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitive
/// [DrawPrimitiveUP]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitiveup
/// [DrawRectPatch]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawrectpatch
/// [DrawTriPatch]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawtripatch
/// [EndScene]:                     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endscene
/// [EndStateBlock]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endstateblock
/// [EvictManagedResources]:        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-evictmanagedresources
/// [GetAvailableTextureMem]:       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getavailabletexturemem
/// [GetBackBuffer]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getbackbuffer
/// [GetClipPlane]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipplane
/// [GetClipStatus]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipstatus
/// [GetCreationParameters]:        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getcreationparameters
/// [GetCurrentTexturePalette]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getcurrenttexturepalette
/// [GetDepthStencilSurface]:       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdepthstencilsurface
/// [GetDeviceCaps]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdevicecaps
/// [GetDirect3D]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdirect3d
/// [GetDisplayMode]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdisplaymode
/// [GetFrontBufferData]:           https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getfrontbufferdata
/// [GetFVF]:                       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getfvf
/// [GetGammaRamp]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getgammaramp
/// [GetIndices]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getindices
/// [GetLight]:                     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getlight
/// [GetLightEnable]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getlightenable
/// [GetMaterial]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getmaterial
/// [GetNPatchMode]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getnpatchmode
/// [GetNumberOfSwapChains]:        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getnumberofswapchains
/// [GetPaletteEntries]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpaletteentries
/// [GetPixelShader]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshader
/// [GetPixelShaderConstantB]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshaderconstantb
/// [GetPixelShaderConstantF]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshaderconstantf
/// [GetPixelShaderConstantI]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshaderconstanti
/// [GetRasterStatus]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrasterstatus
/// [GetRenderState]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrenderstate
/// [GetRenderTarget]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertarget
/// [GetRenderTargetData]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertargetdata
/// [GetSamplerState]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsamplerstate
/// [GetScissorRect]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getscissorrect
/// [GetSoftwareVertexProcessing]:  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsoftwarevertexprocessing
/// [GetStreamSource]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getstreamsource
/// [GetStreamSourceFreq]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getstreamsourcefreq
/// [GetSwapChain]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getswapchain
/// [GetTexture]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-gettexture
/// [GetTextureStageState]:         https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-gettexturestagestate
/// [GetTransform]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-gettransform
/// [GetVertexDeclaration]:         https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexdeclaration
/// [GetVertexShader]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshader
/// [GetVertexShaderConstantB]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshaderconstantb
/// [GetVertexShaderConstantF]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshaderconstantf
/// [GetVertexShaderConstantI]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshaderconstanti
/// [GetViewport]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getviewport
/// [LightEnable]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-lightenable
/// [MultiplyTransform]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-multiplytransform
/// [Present]:                      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-present
/// [ProcessVertices]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-processvertices
/// [Reset]:                        https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-reset
/// [SetClipPlane]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setclipplane
/// [SetClipStatus]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setclipstatus
/// [SetCurrentTexturePalette]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcurrenttexturepalette
/// [SetCursorPosition]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorposition
/// [SetCursorProperties]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorproperties
/// [SetDepthStencilSurface]:       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setdepthstencilsurface
/// [SetDialogBoxMode]:             https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setdialogboxmode
/// [SetFVF]:                       https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setfvf
/// [SetGammaRamp]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setgammaramp
/// [SetIndices]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setindices
/// [SetLight]:                     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setlight
/// [SetMaterial]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setmaterial
/// [SetNPatchMode]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setnpatchmode
/// [SetPaletteEntries]:            https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpaletteentries
/// [SetPixelShader]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshader
/// [SetPixelShaderConstantB]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshaderconstantb
/// [SetPixelShaderConstantF]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshaderconstantf
/// [SetPixelShaderConstantI]:      https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshaderconstanti
/// [SetRenderState]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setrenderstate
/// [SetRenderTarget]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setrendertarget
/// [SetSamplerState]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsamplerstate
/// [SetScissorRect]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setscissorrect
/// [SetSoftwareVertexProcessing]:  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsoftwarevertexprocessing
/// [SetStreamSource]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsource
/// [SetStreamSourceFreq]:          https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsourcefreq
/// [SetTexture]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-settexture
/// [SetTextureStageState]:         https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-settexturestagestate
/// [SetTransform]:                 https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-settransform
/// [SetVertexDeclaration]:         https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexdeclaration
/// [SetVertexShader]:              https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshader
/// [SetVertexShaderConstantB]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshaderconstantb
/// [SetVertexShaderConstantF]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshaderconstantf
/// [SetVertexShaderConstantI]:     https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshaderconstanti
/// [SetViewport]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setviewport
/// [ShowCursor]:                   https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-showcursor
/// [StretchRect]:                  https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-stretchrect
/// [TestCooperativeLevel]:         https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-testcooperativelevel
/// [UpdateSurface]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-updatesurface
/// [UpdateTexture]:                https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-updatetexture
/// [ValidateDevice]:               https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-validatedevice
///
pub trait IDirect3DDevice9Ext : AsSafe<IDirect3DDevice9> + Sized {
    // TODO: fn scene(&self) with sane error handling / drop behavior?

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginscene)\]
    /// IDirect3DDevice9::BeginScene
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if the device was already within a scene (e.g. [begin_scene] was called twice without an intervening [end_scene])
    /// *   `Ok(())`                if the device was not already within a scene, and now is
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.begin_scene().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.begin_scene());
    /// // ...issue draw calls and stuff...
    /// device.end_scene().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.end_scene());
    /// ```
    ///
    /// [begin_scene]:          Self::begin_scene
    /// [end_scene]:            Self::end_scene
    fn begin_scene(&self) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::begin_scene => IDirect3DDevice9::BeginScene);
        fn_check_hr!(unsafe { self.as_winapi().BeginScene() })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginstateblock)\]
    /// IDirect3DDevice9::BeginStateBlock
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if the device was already within a state block
    /// *   `Ok(())`                otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.begin_state_block().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.begin_state_block());
    /// // ...issue draw calls and stuff...
    /// let block : StateBlock = device.end_state_block().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().map(|_| ()));
    /// ```
    fn begin_state_block(&self) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::begin_state_block => IDirect3DDevice9::BeginStateBlock);
        fn_check_hr!(unsafe { self.as_winapi().BeginStateBlock() })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-clear)\]
    /// IDirect3DDevice9::Clear
    ///
    /// Clears one or more surfaces such as a render target, multiple render targets, a stencil buffer, and a depth buffer.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `rects.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]   if all non-`rects` parameters were `None`
    /// *   [D3DERR::INVALIDCALL]   if `color`   was `Some(...)` without a render target being bound
    /// *   [D3DERR::INVALIDCALL]   if `depth`   was `Some(...)` without a depth buffer being bound
    /// *   [D3DERR::INVALIDCALL]   if `stencil` was `Some(...)` without a stencil buffer being bound
    /// *   `Ok(())`                otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.clear(None, Some(d3d::Color::argb(0xFF112233)), None, None).unwrap();
    /// ```
    fn clear(&self, rects: Option<&[Rect]>, color: Option<Color>, depth: Option<f32>, stencil: Option<u32>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::clear => IDirect3DDevice9::Clear);
        // TODO: more clear docs
        // TODO: conversion traits for params?

        let n     = rects.map_or(Ok(0), |rects| fn_param_try_len32!(rects))?;
        let rects = rects.map_or(null(), |r| r.as_ptr().cast());

        let flags =
            ((color  .is_some() as u32) * D3DCLEAR_TARGET ) |
            ((depth  .is_some() as u32) * D3DCLEAR_ZBUFFER) |
            ((stencil.is_some() as u32) * D3DCLEAR_STENCIL);

        let color   = color.unwrap_or_default().into();
        let depth   = depth.unwrap_or(0.0);
        let stencil = stencil.unwrap_or(0);

        fn_check_hr!(unsafe { self.as_winapi().Clear(n, rects, flags, color, depth, stencil) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-colorfill)\]
    /// IDirect3DDevice9::ColorFill
    ///
    /// Allows an application to fill a rectangular area of a [Pool::Default] surface with a specified color.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]     if `surface` isn't from [Pool::Default] ?
    /// *   [D3DERR::INVALIDCALL]     if `surface` isn't a supported format ?
    /// *   [D3DERR::INVALIDCALL]     if `rect` exceeds the bounds of the surface
    /// *   `Ok(())`                  on success
    fn color_fill(&self, surface: &Surface, rect: Option<Rect>, color: impl Into<Color>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::color_fill => IDirect3DDevice9::ColorFill);
        let rect = rect.map(RECT::from);
        let rect = rect.as_ref().map_or(null(), |r| r);
        fn_check_hr!(unsafe { self.as_winapi().ColorFill(surface.as_raw(), rect, color.into().into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createadditionalswapchain)\]
    /// IDirect3DDevice9::CreateAdditionalSwapChain
    ///
    /// Creates an additional swap chain for rendering multiple views.
    ///
    /// there is always at least one swap chain (the implicit swap chain) for each device because Direct3D 9 has one swap chain as a property of the device.
    /// Note that any given device can support only one full-screen swap chain.
    /// [Format::UNKNOWN] can be specified for the windowed mode back buffer format when calling [IDirect3D9Ext::create_device], [IDirect3DDevice9Ext::reset] and [IDirect3DDevice9Ext::create_additional_swap_chain].
    /// This means the application does not have to query the current desktop format before calling [create_device] for windowed mode.
    /// For full-screen mode, the back buffer format must be specified.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   The caller's codebase is responsible for ensuring any [HWND]s inside [PresentParameters] outlive the resulting [SwapChain]s that depend on them.
    ///     See [IDirect3D9Ext::create_device] for details and guidance about dealing with this lifetime issue.
    ///
    /// ### Returns
    /// *   [D3DERR::NOTAVAILABLE]
    /// *   [D3DERR::DEVICELOST]
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([SwapChain])
    ///
    /// ### See Also
    /// *   [Presenting Multiple Views in Windowed Mode (Direct3D 9)](https://learn.microsoft.com/en-us/windows/desktop/direct3d9/presenting-multiple-views-in-windowed-mode)
    ///
    /// [create_device]:            #method.create_device
    unsafe fn create_additional_swap_chain(&self, presentation_parameters: &mut PresentParameters<'static>) -> Result<SwapChain, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_additional_swap_chain => IDirect3DDevice9::CreateAdditionalSwapChain);
        let mut swap_chain = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateAdditionalSwapChain(presentation_parameters.as_mut(), &mut swap_chain) })?;
        Ok(unsafe { SwapChain::from_raw(swap_chain) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture)\]
    /// IDirect3DDevice9::CreateCubeTexture
    ///
    /// Creates a cube texture resource.
    ///
    /// ### Arguments
    /// *   `edge_length`       The size in pixels of every edge of the cubemap at mip 0.  E.g. an edge length of 128 means 6 x 128 x 128 pixel cubemap faces.
    /// *   `levels`            The number of mipmap levels, or `0` to have Direct3D generate sublevels down to 1x1.
    /// *   `usage`             See [Usage and Resource Combinations](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dusage#usage-and-resource-combinations)
    /// *   `format`            The format to be used by this texture to store pixels or pixel blocks.
    /// *   `pool`              Specifies how to manage the memory of the cube texture.
    /// *   `shared_handle`     Reserved, specify `()`.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([CubeTexture])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 6 x 128x128 ARGB cubemap with automatic mipmaps
    /// let texture = device.create_cube_texture(128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_cube_texture(1 << 15, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).map(|_|()));
    /// ```
    fn create_cube_texture(&self, edge_length: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<CubeTexture, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_cube_texture => IDirect3DDevice9::CreateCubeTexture);
        let _ = shared_handle;
        let mut texture = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateCubeTexture(edge_length, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut texture, null_mut()) })?;
        Ok(unsafe { CubeTexture::from_raw(texture) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture)\]
    /// IDirect3DDevice9::CreateCubeTexture
    ///
    /// Creates a cube texture resource.
    ///
    /// ### Arguments
    /// *   `size`              The size in pixels of every edge of the cubemap at mip 0.  E.g. an edge length of 128 means 6 x 128 x 128 pixel cubemap faces.
    /// *   `mips`              The pixel data to initialize the cubemap with.
    /// *   `usage`             See [Usage and Resource Combinations](https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dusage#usage-and-resource-combinations)
    /// *   `format`            The format to be used by this texture to store pixels or pixel blocks, and to use to interpret `mips`.
    /// *   `pool`              Specifies how to manage the memory of the cube texture.
    /// *   `shared_handle`     Reserved, specify `()`.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([CubeTexture])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a cubemap where each face is a 4x4 manually mipmapped ARGB texture
    /// let data = [0u8; 4*4*4];
    /// // Normally you wouldn't overlap mip data like this, but for solid colors its fine:
    /// let m0 = TextureMipRef { data: &data[..4 * 4*4 ], stride: 4*4 };
    /// let m1 = TextureMipRef { data: &data[..4 * 2*2 ], stride: 4*2 };
    /// let m2 = TextureMipRef { data: &data[..4 * 1*1 ], stride: 4*1 };
    /// let mips = [
    ///     // Normally you wouldn't have identical cubemap faces either
    ///     CubeTextureMipRef { pos_x: m0, neg_x: m0, pos_y: m0, neg_y: m0, pos_z: m0, neg_z: m0 },
    ///     CubeTextureMipRef { pos_x: m1, neg_x: m1, pos_y: m1, neg_y: m1, pos_z: m1, neg_z: m1 },
    ///     CubeTextureMipRef { pos_x: m2, neg_x: m2, pos_y: m2, neg_y: m2, pos_z: m2, neg_z: m2 },
    /// ];
    /// let texture = device.create_cube_texture_from(4, &mips, Usage::None,    FixedTextureFormat::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let texture = device.create_cube_texture_from(4, &mips, Usage::Dynamic, FixedTextureFormat::A8R8G8B8, Pool::Default, ()).unwrap();
    /// ```
    fn create_cube_texture_from(&self, size: u32, mips: &[CubeTextureMipRef], usage: impl Into<Usage>, format: &FixedTextureFormat, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<CubeTexture, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_cube_texture_from => IDirect3DDevice9::CreateCubeTexture);
        // TODO: consider THINERR::* constants instead?
        if size == 0        { return Err(fn_param_error!(size, D3DERR::INVALIDCALL)); }
        if mips.is_empty()  { return Err(fn_param_error!(mips, D3DERR::INVALIDCALL)); } // 0 levels = autogenerate mips, which is different from no levels

        let levels          = fn_param_try_len32!(mips)?;
        let usage           = usage.into();
        let pool            = pool.into();
        let texture         = self.create_cube_texture(size, levels, usage, format.format, pool, shared_handle)?;
        let block_bits      = u32::from(format.bits_per_block);
        let block_width     = u32::from(format.block_size.0);
        let block_height    = u32::from(format.block_size.1);
        let is_dynamic      = 0 != (usage.into_inner() & d3d::Usage::Dynamic.into_inner());

        let mut mip_pixels_size = size;
        for (mip_level, mip_ref) in mips.iter().enumerate() {
            assert!(mip_pixels_size != 0, "too many mips"); // should this bail out instead perhaps?

            let mip_level           = mip_level as u32; // safe: mip_level < mips.len() == levels <= u32::MAX
            let mip_blocks_width    = (mip_pixels_size + block_width  - 1) / block_width;
            let mip_blocks_height   = (mip_pixels_size + block_height - 1) / block_height;
            let block_row_bytes     = ((mip_blocks_width * block_bits + 7) / 8) as usize;

            for (face,                          mip_ref         ) in [
                (d3d::CubeMapFace::PositiveX,   &mip_ref.pos_x  ),
                (d3d::CubeMapFace::NegativeX,   &mip_ref.neg_x  ),
                (d3d::CubeMapFace::PositiveY,   &mip_ref.pos_y  ),
                (d3d::CubeMapFace::NegativeY,   &mip_ref.neg_y  ),
                (d3d::CubeMapFace::PositiveZ,   &mip_ref.pos_z  ),
                (d3d::CubeMapFace::NegativeZ,   &mip_ref.neg_z  ),
            ].iter().copied() {
                let lock_type           = if !is_dynamic { Lock::NoOverwrite } else if mip_level == 0 && face == d3d::CubeMapFace::PositiveX { Lock::Discard } else { Lock::None };
                let lock                = unsafe { texture.lock_rect_unchecked(face, mip_level, .., lock_type) }?;
                let dst_origin          = lock.pBits as *mut u8;
                let dst_pitch           = lock.Pitch as u32;
                debug_assert!(dst_pitch as usize >= block_row_bytes);

                for block_row in 0 .. mip_blocks_height {
                    // while `[..block_row_bytes]` looks redundant, its bounds check is necessary for soundness!
                    let src = mip_ref.data[block_row as usize * mip_ref.stride ..][..block_row_bytes].as_ptr();

                    // In Direct3D 8+, Pitch is bytes per *row of blocks* (ref: https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-rect)
                    let dst = unsafe { dst_origin.add((block_row * dst_pitch) as usize) };

                    unsafe { std::ptr::copy_nonoverlapping(src, dst, block_row_bytes) };
                }

                texture.unlock_rect(face, mip_level)?;
            }

            mip_pixels_size >>= 1;
        }

        Ok(texture)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createdepthstencilsurface)\]
    /// IDirect3DDevice9::CreateDepthStencilSurface
    ///
    /// Creates a depth-stencil resource.
    fn create_depth_stencil_surface(&self, width: u32, height: u32, format: Format, multi_sample: MultiSample, multi_sample_quality: u32, discard: bool, _shared_handle: impl SharedHandleParam) -> Result<Surface, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_depth_stencil_surface => IDirect3DDevice9::CreateDepthStencilSurface);
        let mut surface = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateDepthStencilSurface(width, height, format.into(), multi_sample.into(), multi_sample_quality, discard.into(), &mut surface, null_mut()) })?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer)\]
    /// IDirect3DDevice9::CreateIndexBuffer
    ///
    /// Creates an index buffer.
    ///
    /// ### Arguments
    /// *   `length`            Size of the index buffer, **in bytes**.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `format`            Typically [Format::Index16] or [Format::Index32] (type of index buffer)
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://learn.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one index (2 for [Format::Index16], 4 for [Format::Index32])
    /// *   [D3DERR::INVALIDCALL]       if `usage`, `format`, or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::INVALIDDATA]
    /// *   [E::OUTOFMEMORY]
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([IndexBuffer])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_index_buffer(3 * 2, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// ```
    fn create_index_buffer(&self, length: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<IndexBuffer, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_index_buffer => IDirect3DDevice9::CreateIndexBuffer);
        // !0 will fail OUTOFMEMORY
        // !0/2 spammed will fail OUTOFVIDEOMEMORY
        // !0-4 spammed will "succeed", hinting at an arithmetic overflow within d3d or the driver
        if length > MAX_BUFFER_ALLOC { return Err(fn_param_error!(length, THINERR::ALLOC_OVERFLOW)); }

        let _ = shared_handle;
        let mut buffer = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateIndexBuffer(length, usage.into().into(), format.into().into(), pool.into().into(), &mut buffer, null_mut()) })?;
        Ok(unsafe { IndexBuffer::from_raw(buffer) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer)\]
    /// IDirect3DDevice9::CreateIndexBuffer
    ///
    /// Creates an index buffer.
    ///
    /// ### Arguments
    /// *   `data`              &\[u16\] or &\[u32\]
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://learn.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one index (2 for [Format::Index16], 4 for [Format::Index32])
    /// *   [D3DERR::INVALIDCALL]       if `usage`, `format`, or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::INVALIDDATA]
    /// *   [E::OUTOFMEMORY]
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([IndexBuffer])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let inds : &[u16] = &[0, 1, 2, 0, 2, 3];
    /// let tri = device.create_index_buffer_from(inds, Usage::None, Pool::Managed, ()).unwrap();
    /// ```
    fn create_index_buffer_from<I: Index>(&self, data: &[I], usage: impl Into<Usage>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<IndexBuffer, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_index_buffer_from => IDirect3DDevice9::CreateIndexBuffer);
        let bytes = std::mem::size_of_val(data);
        let bytes32 : u32 = bytes.try_into().map_err(|_| fn_param_error!(data, THINERR::ALLOC_OVERFLOW))?;
        let usage = usage.into();
        let lock = if usage.into_inner() & Usage::Dynamic.into_inner() != 0 { Lock::NoOverwrite } else { Lock::None };
        let ib = self.create_index_buffer(bytes32, usage, I::format(), pool, shared_handle)?;
        unsafe {
            let dst = ib.lock_unchecked(0, 0, lock)?;
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, dst.cast(), bytes);
            ib.unlock()?;
        }
        Ok(ib)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createoffscreenplainsurface)\]
    /// IDirect3DDevice9::CreateOffscreenPlainSurface
    ///
    /// Create an off-screen surface.
    fn create_offscreen_plain_surface(&self, width: u32, height: u32, format: Format, pool: Pool, _shared_handle: impl SharedHandleParam) -> Result<Surface, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_offscreen_plain_surface => IDirect3DDevice9::CreateOffscreenPlainSurface);
        let mut surface = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateOffscreenPlainSurface(width, height, format.into(), pool.into(), &mut surface, null_mut()) })?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createpixelshader)\]
    /// IDirect3DDevice9::CreatePixelShader
    ///
    /// Creates a pixel shader.
    ///
    /// ### ⚠️ Safety ⚠️
    /// The caller must pass a valid shader blob.
    /// The underlying Direct3D API is unsound - it doesn't even take a length for the DWORD array.
    /// This function will likely attempt to validate the shader blob bytecode in the future and/or offload such validation onto the parameter, but until then this is unsound as heck.
    /// Do not trust user-generated-content for shader bytecode blobs.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([PixelShader])
    unsafe fn create_pixel_shader(&self, function: &[u32]) -> Result<PixelShader, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_pixel_shader => IDirect3DDevice9::CreatePixelShader);
        let mut shader = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreatePixelShader(function.as_ptr(), &mut shader) })?;
        Ok(unsafe { PixelShader::from_raw(shader) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createquery)\]
    /// IDirect3DDevice9::CreateQuery
    ///
    /// Creates a status query.
    fn create_query(&self, type_: QueryType) -> Result<Query, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_query => IDirect3DDevice9::CreateQuery);
        let mut query = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateQuery(type_.into(), &mut query) })?;
        Ok(unsafe { Query::from_raw(query) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createrendertarget)\]
    /// IDirect3DDevice9::CreateRenderTarget
    ///
    /// Creates a render-target surface.
    fn create_render_target(&self, width: u32, height: u32, format: Format, multi_sample: MultiSample, multi_sample_quality: u32, lockable: bool, _shared_handle: impl SharedHandleParam) -> Result<Surface, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_render_target => IDirect3DDevice9::CreateRenderTarget);
        let mut surface = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateRenderTarget(width, height, format.into(), multi_sample.into(), multi_sample_quality, lockable.into(), &mut surface, null_mut()) })?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createstateblock)\]
    /// IDirect3DDevice9::CreateStateBlock
    ///
    /// Creates a new state block that contains the values for all device states, vertex-related states, or pixel-related states.
    ///
    /// Vertex-related device states typically refer to those states that affect how the system processes vertices.
    /// Pixel-related states generally refer to device states that affect how the system processes pixel or depth-buffer data during rasterization.
    /// Some states are contained in both groups.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([StateBlock])
    fn create_state_block(&self, type_: StateBlockType) -> Result<StateBlock, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_state_block => IDirect3DDevice9::CreateStateBlock);
        let mut sb = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateStateBlock(type_.into(), &mut sb) })?;
        Ok(unsafe { StateBlock::from_raw(sb) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture)\]
    /// IDirect3DDevice9::CreateTexture
    ///
    /// Creates a texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([Texture])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 128x128 ARGB texture with no mipmaps
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_texture(1 << 15, 1 << 15, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).map(|_|()));
    /// ```
    fn create_texture(&self, width: u32, height: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<Texture, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_texture => IDirect3DDevice9::CreateTexture);
        let mut texture = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateTexture(width, height, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut texture, null_mut()) })?;
        Ok(unsafe { Texture::from_raw(texture) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture)\]
    /// IDirect3DDevice9::CreateTexture
    ///
    /// Creates a texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([Texture])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 4x4 manually mipmapped ARGB texture
    /// let data = [0u8; 4*4*4];
    /// let mips = [
    ///     // Normally you wouldn't overlap mip data like this, but for solid colors its fine:
    ///     TextureMipRef { data: &data[..4 * 4*4 ], stride: 4*4 },
    ///     TextureMipRef { data: &data[..4 * 2*2 ], stride: 4*2 },
    ///     TextureMipRef { data: &data[..4 * 1*1 ], stride: 4*1 },
    /// ];
    /// let texture = device.create_texture_from(4, 4, &mips, Usage::None,    FixedTextureFormat::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let texture = device.create_texture_from(4, 4, &mips, Usage::Dynamic, FixedTextureFormat::A8R8G8B8, Pool::Default, ()).unwrap();
    /// ```
    fn create_texture_from(&self, width: u32, height: u32, mips: &[TextureMipRef], usage: impl Into<Usage>, format: &FixedTextureFormat, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<Texture, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_texture_from => IDirect3DDevice9::CreateTexture);
        // TODO: consider THINERR::* constants instead?
        if width == 0       { return Err(fn_param_error!(width,  D3DERR::INVALIDCALL)); }
        if height == 0      { return Err(fn_param_error!(height, D3DERR::INVALIDCALL)); }
        if mips.is_empty()  { return Err(fn_param_error!(mips,   D3DERR::INVALIDCALL)); } // 0 levels = autogenerate mips, which is different from no levels

        let levels          = fn_param_try_len32!(mips)?;
        let usage           = usage.into();
        let pool            = pool.into();
        let texture         = self.create_texture(width, height, levels, usage, format.format, pool, _shared_handle)?;
        let block_bits      = u32::from(format.bits_per_block);
        let block_width     = u32::from(format.block_size.0);
        let block_height    = u32::from(format.block_size.1);
        let is_dynamic      = 0 != (usage.into_inner() & d3d::Usage::Dynamic.into_inner());

        let mut mip_pixels_width    = width;
        let mut mip_pixels_height   = height;
        for (mip_level, mip_ref) in mips.iter().enumerate() {
            assert!(mip_pixels_width != 0 || mip_pixels_height != 0, "too many mips"); // should this bail out instead perhaps?
            mip_pixels_width    = mip_pixels_width.max(1);
            mip_pixels_height   = mip_pixels_height.max(1);

            let lock_type           = if !is_dynamic { Lock::NoOverwrite } else if mip_level == 0 { Lock::Discard } else { Lock::None };
            let mip_level           = mip_level as u32; // safe: mip_level < mips.len() == levels <= u32::MAX
            let mip_blocks_width    = (mip_pixels_width  + block_width  - 1) / block_width;
            let mip_blocks_height   = (mip_pixels_height + block_height - 1) / block_height;
            let block_row_bytes     = ((mip_blocks_width * block_bits + 7) / 8) as usize;

            let lock                = unsafe { texture.lock_rect_unchecked(mip_level, .., lock_type) }?;
            let dst_origin          = lock.pBits as *mut u8;
            let dst_pitch           = lock.Pitch as u32;
            debug_assert!(dst_pitch as usize >= block_row_bytes);

            for block_row in 0 .. mip_blocks_height {
                // while `[..block_row_bytes]` looks redundant, its bounds check is necessary for soundness!
                let src = mip_ref.data[block_row as usize * mip_ref.stride ..][..block_row_bytes].as_ptr();

                // In Direct3D 8+, Pitch is bytes per *row of blocks* (ref: https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-rect)
                let dst = unsafe { dst_origin.add((block_row * dst_pitch) as usize) };

                unsafe { std::ptr::copy_nonoverlapping(src, dst, block_row_bytes) };
            }

            texture.unlock_rect(mip_level)?;
            mip_pixels_width    >>= 1;
            mip_pixels_height   >>= 1;
        }

        Ok(texture)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer)\]
    /// IDirect3DDevice9::CreateVertexBuffer
    ///
    /// Creates an vertex buffer.
    ///
    /// ### Arguments
    /// *   `length`            Size of the index buffer, **in bytes**.
    ///                         For FVF vertex buffers, Length must be large enough to contain at least one vertex, but it need not be a multiple of the vertex size.
    ///                         Length is not validated for non-FVF buffers.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `fvf`               Combination of [FVF], a usage specifier that describes the vertex format of the verticies in this buffer.
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://learn.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one [FVF]-sized vertex (1 if [FVF::None])
    /// *   [D3DERR::INVALIDCALL]       if `usage` or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]  if allocation failed (driver or gpu memory)
    /// *   [E::OUTOFMEMORY]            if allocation failed (driver or d3d runtime)
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([VertexBuffer])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let vert_size = 3 * 4; // XYZ * floats
    /// let length = 3 * vert_size; // 3 verts
    /// let tri = device.create_vertex_buffer(length, Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// ```
    fn create_vertex_buffer(&self, length: u32, usage: impl Into<Usage>, fvf: impl Into<FVF>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<VertexBuffer, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_vertex_buffer => IDirect3DDevice9::CreateVertexBuffer);
        // !0 will fail OUTOFMEMORY
        // !0/2 spammed will fail OUTOFVIDEOMEMORY
        // !0-4 spammed will "succeed", hinting at an arithmetic overflow within d3d or the driver
        if length > MAX_BUFFER_ALLOC { return Err(fn_param_error!(length, THINERR::ALLOC_OVERFLOW)); }
        let _ = shared_handle;

        let mut buffer = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateVertexBuffer(length, usage.into().into(), fvf.into().into(), pool.into().into(), &mut buffer, null_mut()) })?;
        Ok(unsafe { VertexBuffer::from_raw(buffer) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer)\]
    /// IDirect3DDevice9::CreateVertexBuffer
    ///
    /// Creates an vertex buffer.
    ///
    /// ### Arguments
    /// *   `data`              Data to initialize the vertex buffer with.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `fvf`               Combination of [FVF], a usage specifier that describes the vertex format of the verticies in this buffer.
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://learn.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one [FVF]-sized vertex (1 if [FVF::None])
    /// *   [D3DERR::INVALIDCALL]       if `usage` or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]  if allocation failed (driver or gpu memory)
    /// *   [E::OUTOFMEMORY]            if allocation failed (driver or d3d runtime)
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([VertexBuffer])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_vertex_buffer_from(&[
    ///     [0.0_f32, 1.0, 0.0],
    ///     [1.0_f32, 0.0, 0.0],
    ///     [0.0_f32, 0.0, 1.0],
    /// ][..], Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// ```
    fn create_vertex_buffer_from<V: Pod>(&self, data: &[V], usage: impl Into<Usage>, fvf: impl Into<FVF>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<VertexBuffer, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_vertex_buffer_from => IDirect3DDevice9::CreateVertexBuffer);
        let bytes = std::mem::size_of_val(data);
        let bytes32 : u32 = bytes.try_into().map_err(|_| fn_param_error!(data, THINERR::ALLOC_OVERFLOW))?;
        let usage = usage.into();
        let lock = if usage.into_inner() & Usage::Dynamic.into_inner() != 0 { Lock::NoOverwrite } else { Lock::None };
        let vb = self.create_vertex_buffer(bytes32, usage, fvf, pool, shared_handle)?;
        unsafe {
            let dst = vb.lock_unchecked(0, 0, lock)?;
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, dst.cast(), bytes);
            vb.unlock()?;
        }
        Ok(vb)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexdeclaration)\]
    /// IDirect3DDevice9::CreateVertexDeclaration
    ///
    /// Create a vertex shader declaration from the device and the vertex elements.
    ///
    /// See the [Vertex Declaration (Direct3D 9)] page for a detailed description of how to map vertex declarations between different versions of DirectX.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if `elements.last()` != `Some(D3DDECL_END)`
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VertexDeclaration])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let decl = device.create_vertex_declaration(&[
    ///     VertexElement::new(0,  0, DeclType8::Float4, DeclMethod8::Default, DeclUsage8::Position, 0),
    ///     VertexElement::new(0, 16, DeclType8::Float2, DeclMethod8::Default, DeclUsage8::TexCoord, 0),
    ///     VertexElement::END
    /// ]).unwrap();
    /// ```
    ///
    /// [Vertex Declaration (Direct3D 9)]:          https://learn.microsoft.com/en-us/windows/desktop/direct3d9/vertex-declaration
    fn create_vertex_declaration(&self, elements: &[VertexElement]) -> Result<VertexDeclaration, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_vertex_declaration => IDirect3DDevice9::CreateVertexDeclaration);
        let end = elements.last().ok_or(fn_param_error!(elements, D3DERR::INVALIDCALL))?;
        // This check is required for CreateVertexDeclaration to be sound!
        if *end != VertexElement::END { return Err(fn_param_error!(elements, D3DERR::INVALIDCALL)); }

        let mut vd = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateVertexDeclaration(elements.as_ptr().cast(), &mut vd) })?;
        Ok(unsafe { VertexDeclaration::from_raw(vd) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexshader)\]
    /// IDirect3DDevice9::CreateVertexShader
    ///
    /// Creates a vertex shader.
    ///
    /// ### ⚠️ Safety ⚠️
    /// The caller must pass a valid shader blob.
    /// The underlying Direct3D API is unsound - it doesn't even take a length for the DWORD array.
    /// This function will likely attempt to validate the shader blob bytecode in the future and/or offload such validation onto the parameter, but until then this is unsound as heck.
    /// Do not trust user-generated-content for shader bytecode blobs.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([VertexShader])
    unsafe fn create_vertex_shader(&self, function: &[u32]) -> Result<VertexShader, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_vertex_shader => IDirect3DDevice9::CreateVertexShader);
        let mut shader = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateVertexShader(function.as_ptr(), &mut shader) })?;
        Ok(unsafe { VertexShader::from_raw(shader) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvolumetexture)\]
    /// IDirect3DDevice9::CreateVolumeTexture
    ///
    /// Creates a volume texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([VolumeTexture])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 32x32x32 volumetric ARGB texture with no mipmaps
    /// let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_volume_texture(1 << 10, 1 << 10, 1 << 10, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).map(|_|()));
    /// ```
    fn create_volume_texture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VolumeTexture, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_volume_texture => IDirect3DDevice9::CreateVolumeTexture);
        let mut volumetexture = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().CreateVolumeTexture(width, height, depth, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut volumetexture, null_mut()) })?;
        Ok(unsafe { VolumeTexture::from_raw(volumetexture) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvolumetexture)\]
    /// IDirect3DDevice9::CreateVolumeTexture
    ///
    /// Creates a volume texture resource.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([VolumeTexture])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 4x4x4 manually mipmapped ARGB texture
    /// let data = [0u8; 4*4*4*4];
    /// let mips = [
    ///     // Normally you wouldn't overlap mip data like this, but for solid colors its fine:
    ///     VolumeTextureMipRef { data: &data[..4 * 4*4*4 ], stride_row: 4*4, stride_slice: 4*4*4 },
    ///     VolumeTextureMipRef { data: &data[..4 * 2*2*2 ], stride_row: 4*2, stride_slice: 4*2*2 },
    ///     VolumeTextureMipRef { data: &data[..4 * 1*1*1 ], stride_row: 4*1, stride_slice: 4*1*1 },
    /// ];
    /// let texture = device.create_volume_texture_from(4, 4, 4, &mips, Usage::None,    FixedTextureFormat::A8R8G8B8, Pool::Managed, ()).unwrap();
    /// let texture = device.create_volume_texture_from(4, 4, 4, &mips, Usage::Dynamic, FixedTextureFormat::A8R8G8B8, Pool::Default, ()).unwrap();
    /// ```
    fn create_volume_texture_from(&self, width: u32, height: u32, depth: u32, mips: &[VolumeTextureMipRef], usage: impl Into<Usage>, format: &FixedTextureFormat, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VolumeTexture, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::create_volume_texture_from => IDirect3DDevice9::CreateVolumeTexture);
        // TODO: consider THINERR::* constants instead?
        if width == 0       { return Err(fn_param_error!(width,  D3DERR::INVALIDCALL)); }
        if height == 0      { return Err(fn_param_error!(height, D3DERR::INVALIDCALL)); }
        if mips.is_empty()  { return Err(fn_param_error!(mips,   D3DERR::INVALIDCALL)); } // 0 levels = autogenerate mips, which is different from no levels

        let levels          = fn_param_try_len32!(mips)?;
        let usage           = usage.into();
        let pool            = pool.into();
        let texture         = self.create_volume_texture(width, height, depth, levels, usage, format.format, pool, _shared_handle)?;
        let block_bits      = u32::from(format.bits_per_block);
        let block_width     = u32::from(format.block_size.0);
        let block_height    = u32::from(format.block_size.1);
        let is_dynamic      = 0 != (usage.into_inner() & d3d::Usage::Dynamic.into_inner());

        let mut mip_pixels_width    = width;
        let mut mip_pixels_height   = height;
        let mut mip_pixels_depth    = depth;
        for (mip_level, mip_ref) in mips.iter().enumerate() {
            assert!(mip_pixels_width != 0 || mip_pixels_height != 0 || mip_pixels_depth != 0, "too many mips"); // should this bail out instead perhaps?
            mip_pixels_width    = mip_pixels_width.max(1);
            mip_pixels_height   = mip_pixels_height.max(1);
            mip_pixels_depth    = mip_pixels_depth.max(1);

            let lock_type           = if !is_dynamic { Lock::None } else if mip_level == 0 { Lock::Discard } else { Lock::None };
            let mip_level           = mip_level as u32; // safe: mip_level < mips.len() == levels <= u32::MAX
            let mip_blocks_width    = (mip_pixels_width  + block_width  - 1) / block_width;
            let mip_blocks_height   = (mip_pixels_height + block_height - 1) / block_height;
            let block_row_bytes     = ((mip_blocks_width * block_bits + 7) / 8) as usize;

            dbg!((mip_level, lock_type));
            let lock                = unsafe { texture.lock_box_unchecked(mip_level, .., lock_type) }?;
            let dst_origin          = lock.pBits as *mut u8;
            let dst_pitch_row       = lock.RowPitch as u32;
            let dst_pitch_slice     = lock.SlicePitch as u32;
            debug_assert!(dst_pitch_row as usize >= block_row_bytes);

            for pixel_slice in 0 .. mip_pixels_depth {
                for block_row in 0 .. mip_blocks_height {
                    // while `[..block_row_bytes]` looks redundant, its bounds check is necessary for soundness!
                    let src = mip_ref.data[pixel_slice as usize * mip_ref.stride_slice + block_row as usize * mip_ref.stride_row ..][..block_row_bytes].as_ptr();

                    // In Direct3D 8+, Pitch is bytes per *row of blocks* (ref: https://learn.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-rect)
                    let dst = unsafe { dst_origin.add((pixel_slice * dst_pitch_slice + block_row * dst_pitch_row) as usize) };

                    unsafe { std::ptr::copy_nonoverlapping(src, dst, block_row_bytes) };
                }
            }

            texture.unlock_box(mip_level)?;
            mip_pixels_width    >>= 1;
            mip_pixels_height   >>= 1;
            mip_pixels_depth    >>= 1;
        }

        Ok(texture)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-deletepatch)\]
    /// IDirect3DDevice9::DeletePatch
    ///
    /// ### Errors
    /// *   [D3DERR::INVALIDCALL]   If `handle` wasn't a valid cached high-order patch handle.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// assert_eq!(D3DERR::INVALIDCALL, device.delete_patch(0));
    /// # for p in 0..32 {
    /// #   assert_eq!(D3DERR::INVALIDCALL, device.delete_patch(1 << p));
    /// # }
    /// ```
    ///
    /// ### See Also
    /// *   [Using Higher-Order Primitives (Direct3D 9)](https://learn.microsoft.com/en-us/windows/win32/direct3d9/using-higher-order-primitives)
    fn delete_patch(&self, handle: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::delete_patch => IDirect3DDevice9::DeletePatch);
        fn_check_hr!(unsafe { self.as_winapi().DeletePatch(handle) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitive)\]
    /// IDirect3DDevice9::DrawIndexedPrimitive
    ///
    /// ### ⚠️ Safety ⚠️
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_indexed_primitive(&self, primitive_type: PrimitiveType, base_vertex_index: i32, min_vertex_index: u32, num_verticies: u32, start_index: u32, primitive_count: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::draw_indexed_primitive => IDirect3DDevice9::DrawIndexedPrimitive);
        fn_check_hr!(unsafe { self.as_winapi().DrawIndexedPrimitive(primitive_type.into(), base_vertex_index, min_vertex_index, num_verticies, start_index, primitive_count) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitiveup)\]
    /// IDirect3DDevice9::DrawIndexedPrimitiveUP
    ///
    /// ### ⚠️ Safety ⚠️
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_indexed_primitive_up<I: Index, V: Pod>(&self, primitive_type: PrimitiveType, min_vertex_index: u32, num_verticies: u32, primitive_count: u32, indicies: &[I], vertex_stream_zero: &[V]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::draw_indexed_primitive_up => IDirect3DDevice9::DrawIndexedPrimitiveUP);
        fn_check_hr!(unsafe { self.as_winapi().DrawIndexedPrimitiveUP(primitive_type.into(), min_vertex_index, num_verticies, primitive_count, indicies.as_ptr().cast(), I::format().into(), vertex_stream_zero.as_ptr().cast(), std::mem::size_of::<V>() as _) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitive)\]
    /// IDirect3DDevice9::DrawPrimitive
    ///
    /// ### ⚠️ Safety ⚠️
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_primitive(&self, primitive_type: PrimitiveType, start_vertex: u32, primitive_count: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::draw_primitive => IDirect3DDevice9::DrawPrimitive);
        fn_check_hr!(unsafe { self.as_winapi().DrawPrimitive(primitive_type.into(), start_vertex, primitive_count) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitiveup)\]
    /// IDirect3DDevice9::DrawPrimitiveUP
    ///
    /// ### ⚠️ Safety ⚠️
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_primitive_up<V: Pod>(&self, primitive_type: PrimitiveType, primitive_count: u32, vertex_stream_zero: &[V]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::draw_primitive_up => IDirect3DDevice9::DrawPrimitiveUP);
        fn_check_hr!(unsafe { self.as_winapi().DrawPrimitiveUP(primitive_type.into(), primitive_count, vertex_stream_zero.as_ptr().cast(), std::mem::size_of::<V>() as _) })
    }

    //TODO:     IDirect3DDevice9::DrawRectPatch                     = d3d9::IDirect3DDevice9Ext::draw_rect_patch
    //TODO:     IDirect3DDevice9::DrawTriPatch                      = d3d9::IDirect3DDevice9Ext::draw_tri_patch

    // TODO: docs for args, remarks, deep links, etc.
    // TODO: safety sections

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endscene)\]
    /// IDirect3DDevice9::EndScene
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if the device was not within a scene (e.g. [end_scene] was called without a [begin_scene], or was called a second time)
    /// *   `Ok(())`                if the device was within a scene that has now ended
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.begin_scene().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.begin_scene());
    /// // ...issue draw calls and stuff...
    /// device.end_scene().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.end_scene());
    /// ```
    ///
    /// [begin_scene]:          Self::begin_scene
    /// [end_scene]:            Self::end_scene
    fn end_scene(&self) -> Result<(), Error> {
        // TODO: examples
        fn_context!(d3d9::IDirect3DDevice9Ext::end_scene => IDirect3DDevice9::EndScene);
        fn_check_hr!(unsafe { self.as_winapi().EndScene() })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endstateblock)\]
    /// IDirect3DDevice9::EndStateBlock
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if the device wasn't within a state block
    /// *   Ok([StateBlock])        if a state block was successfully captured
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.begin_state_block().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.begin_state_block());
    /// // ...issue draw calls and stuff...
    /// let block : StateBlock = device.end_state_block().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.end_state_block().map(|_| ()));
    /// ```
    fn end_state_block(&self) -> Result<StateBlock, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::end_state_block => IDirect3DDevice9::EndStateBlock);
        let mut sb = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().EndStateBlock(&mut sb) })?;
        Ok(unsafe { StateBlock::from_raw(sb) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-evictmanagedresources)]\
    /// IDirect3DDevice9::EvictManagedResources
    ///
    /// Evicts all managed resources, including both Direct3D and driver-managed resources.
    ///
    /// This function causes only the [Pool::Default] copy of resources to be evicted.
    /// The resource copy in system memory is retained. See [Pool].
    ///
    /// ### Returns
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::COMMAND_UNPARSED]
    /// *   Ok(())
    fn evict_managed_resources(&self) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::evict_managed_resources => IDirect3DDevice9::EvictManagedResources);
        fn_check_hr!(unsafe { self.as_winapi().EvictManagedResources() })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getavailabletexturemem)\]
    /// IDirect3DDevice9::GetAvailableTextureMem
    ///
    /// Returns an estimate of the amount of available texture memory.
    ///
    /// The returned value is rounded to the nearest MB.
    /// This is done to reflect the fact that video memory estimates are never precise due to alignment and other issues that affect consumption by certain resources.
    /// Applications can use this value to make gross estimates of memory availability to make large-scale resource decisions such as how many levels of a mipmap to attempt to allocate,
    /// but applications cannot use this value to make small-scale decisions such as if there is enough memory left to allocate another resource.
    ///
    /// ### Returns
    /// *   `0xFFE00000`
    /// *   Maybe occasionally some other values too
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let available = device.get_available_texture_mem();
    /// if available >= 0xFFE0_0000 {
    ///     println!("> 4 GiB available");
    /// } else {
    ///     println!("~ {} MiB available", available / 1024 / 1024);
    /// }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// > 4 GiB available
    /// ```
    fn get_available_texture_mem(&self) -> u32 {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_available_texture_mem => IDirect3DDevice9::GetAvailableTextureMem);
        unsafe { self.as_winapi().GetAvailableTextureMem() }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getbackbuffer)\]
    /// IDirect3DDevice9::GetBackBuffer
    ///
    /// Retrieves a back buffer from the device's swap chain.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]           if `back_buffer` >= number of back buffers
    /// *   Ok([Surface])
    fn get_back_buffer(&self, swap_chain: u32, back_buffer: u32, type_: BackBufferType) -> Result<Surface, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_back_buffer => IDirect3DDevice9::GetBackBuffer);
        let mut surface = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetBackBuffer(swap_chain, back_buffer, type_.into(), &mut surface) })?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipplane)\]
    /// IDirect3DDevice9::GetClipPlane
    ///
    /// Retrieves the coefficients of a user-defined clipping plane for the device.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   if the index exceeds the maximum clipping pane index supported by the device (if there is such a limit)
    /// *   `Ok([A, B, C, D])`, where points `Ax + By + Cz + Dw >= 0` are visible
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// println!("{:?}", device.get_clip_plane(0).unwrap());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// [0.0, 0.0, 0.0, 0.0]
    /// ```
    fn get_clip_plane(&self, index: u32) -> Result<[f32; 4], Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_clip_plane => IDirect3DDevice9::GetClipPlane);
        let mut plane = [0.0, 0.0, 0.0, 0.0];
        fn_check_hr!(unsafe { self.as_winapi().GetClipPlane(index, plane.as_mut_ptr()) })?;
        Ok(plane)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipstatus)\]
    /// IDirect3DDevice9::GetClipStatus
    ///
    /// Retrieves the clip status.
    ///
    /// ### Returns
    /// *   ~~[D3DERR::INVALIDCALL]~~   "If the returned argument is invalid" (impossible via thindx?)
    /// *   Ok([ClipStatus])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// println!("{:?}", device.get_clip_status().unwrap());
    /// ```
    ///
    /// ### Output
    /// ```text
    /// ClipStatus { ClipUnion: CS::None, ClipIntersection: CS::{All|0xfffff000} }
    /// ```
    fn get_clip_status(&self) -> Result<ClipStatus, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_clip_status => IDirect3DDevice9::GetClipStatus);
        let mut status = ClipStatus::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetClipStatus(status.as_mut()) })?;
        Ok(status)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getcreationparameters)\]
    /// IDirect3DDevice9::GetCreationParameters
    ///
    /// Retrieves the creation parameters of the device.
    ///
    /// ### Returns
    /// *   ~~[D3DERR::INVALIDCALL]~~   "If the returned argument is invalid" (impossible via thindx?)
    /// *   Ok([d3d::DeviceCreationParameters])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let params = device.get_creation_parameters().unwrap();
    /// assert_eq!(params.device_type, DevType::HAL);
    /// dbg!(params);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// params = DevieCreationParameters {
    ///     adapter_ordinal: 0,
    ///     device_type: DevType::HAL,
    ///     focus_window: 0x0000000000000000,
    ///     behavior_flags: Create::{FpuPreserve | PureDevice | HardwareVertexProcessing | NoWindowChanges},
    /// }
    /// ```
    fn get_creation_parameters(&self) -> Result<DeviceCreationParameters, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_creation_parameters => IDirect3DDevice9::GetCreationParameters);
        let mut dcp = DeviceCreationParameters::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetCreationParameters(&mut *dcp) })?;
        Ok(dcp)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getcurrenttexturepalette)\]
    /// IDirect3DDevice9::GetCurrentTexturePalette
    ///
    /// Retrieves the current texture palette
    ///
    /// ### Returns
    /// *   ~~[D3DERR::INVALIDCALL]~~   "If the method fails" (impossible via thindx?)
    /// *   Ok(`0xFFFF`)                If no palette was previously set (ambiguous!)
    /// *   Ok(`0xFFFF`)                If palette 0xFFFF was previously set (ambiguous!)
    /// *   Ok(`i`)                     If palette `i` was previously set
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let pal = device.get_current_texture_palette().unwrap();
    /// assert_eq!(0xFFFF, pal);
    /// ```
    ///
    /// ### See Also
    /// *   [Texture Palettes (Direct3D 9)](https://learn.microsoft.com/en-us/windows/win32/direct3d9/texture-palettes)
    /// *   [set_current_texture_palette_unchecked](Self::set_current_texture_palette_unchecked)
    fn get_current_texture_palette(&self) -> Result<u32, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_current_texture_palette => IDirect3DDevice9::GetCurrentTexturePalette);
        let mut pal = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetCurrentTexturePalette(&mut pal) })?;
        Ok(pal)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdepthstencilsurface)\]
    /// IDirect3DDevice9::GetDepthStencilSurface
    ///
    /// Gets the depth-stencil surface owned by the Direct3DDevice object.
    ///
    /// ### Returns
    /// *   ~~[D3DERR::INVALIDCALL]~~ ...?
    /// *   Ok(Some([Surface]))       the render target bound to that index
    /// *   Ok(None)                  no render target was bound to that index
    fn get_depth_stencil_surface(&self) -> Result<Option<Surface>, Error> {
        // TODO: verify soundness before making pub
        fn_context!(d3d9::IDirect3DDevice9Ext::get_depth_stencil_surface => IDirect3DDevice9::GetDepthStencilSurface);
        let mut ds = null_mut();
        let hr = unsafe { self.as_winapi().GetDepthStencilSurface(&mut ds) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            fn_check_hr!(hr)?;
            Ok(unsafe { Surface::from_raw_opt(ds) })
        }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdevicecaps)\]
    /// IDirect3DDevice9::GetDeviceCaps
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([Caps])                  otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let caps : Caps = device.get_device_caps().unwrap();
    /// assert_eq!(caps.device_type,     DevType::HAL);
    /// assert_eq!(caps.adapter_ordinal, 0);
    /// assert!(caps.max_texture_width  > 0);
    /// assert!(caps.max_texture_height > 0);
    /// dbg!(caps);
    /// // ...
    /// ```
    ///
    /// ### Output
    /// ```text
    /// Caps {
    ///     device_type: DevType::HAL,
    ///     adapter_ordinal: 0,
    ///     caps: Caps1::ReadScanline,
    ///     caps2: Caps2::{CanAutoGenMipMap|CanShareResource|DynamicTextures|FullScreenGamma},
    ///     caps3: Caps3::{AlphaFullscreenFlipOrDiscard|CopyToVidMem|CopyToSystemMem|LinearToSrgbPresentation},
    ///     presentation_intervals: Present::{IntervalOne|IntervalTwo|IntervalThree|IntervalFour|IntervalImmediate},
    ///     cursor_caps: CursorCaps::Color,
    ///     dev_caps: DevCaps::{CanBltSysToNonLocal|CanRenderAfterFlip|DrawPrimitives2|DrawPrimitives2Ex|DrawPrimTlVertex|ExecuteSystemMemory|ExecuteVideoMemory|HwRasterization|HwTransformAndLight|PureDevice|TextureNonLocalVidMem|TextureVideoMemory|TlVertexSystemMemory|TlVertexVideoMemory},
    ///     primitive_misc_caps: PMiscCaps::{MaskZ|CullNone|CullCW|CullCCW|ColorWriteEnable|TssArgTemp|BlendOp|IndependentWriteMasks|PerStageConstant|PostBlendSrgbConvert|FogAndSpecularAlpha|SeparateAlphaBlend|MrtIndependentBitDepths|MrtPostPixelShaderBlending|FogVertexClamped},
    ///     raster_caps: PRasterCaps::{Anisotropy|ColorPerspective|Dither|DepthBias|FogRange|FogTable|FogVertex|MipMapLodBias|MultisampleToggle|ScissorTest|SlopeScaleDepthBias|WFog|ZFog|ZTest|ZBias},
    ///     z_cmp_caps: PCmpCaps::{Always|Equal|Greater|GreaterEqual|Less|LessEqual|Never|NotEqual},
    ///     src_blend_caps: PBlendCaps::{BlendFactor|BothInvSrcAlpha|BothSrcAlpha|DestAlpha|DestColor|InvDestAlpha|InvDestColor|InvSrcAlpha|InvSrcColor|One|SrcAlpha|SrcAlphaSat|SrcColor|Zero},
    ///     dest_blend_caps: PBlendCaps::{BlendFactor|BothInvSrcAlpha|BothSrcAlpha|DestAlpha|DestColor|InvDestAlpha|InvDestColor|InvSrcAlpha|InvSrcColor|One|SrcAlpha|SrcAlphaSat|SrcColor|Zero},
    ///     alpha_cmp_caps: PCmpCaps::{Always|Equal|Greater|GreaterEqual|Less|LessEqual|Never|NotEqual},
    ///     shade_caps: PShadeCaps::{AlphaGouraudBlend|ColorGouraudRgb|FogGouraud|SpecularGouraudRgb},
    ///     texture_caps: PTextureCaps::{Alpha|CubeMap|MipCubeMap|MipMap|MipVolumeMap|Perspective|Projected|TexRepeatNotScaledBySize|VolumeMap},
    ///     texture_filter_caps: PTFilterCaps::{MagFPoint|MagFLinear|MagFAnisotropic|MinFPoint|MinFLinear|MinFAnisotropic|MipFPoint|MipFLinear},
    ///     cube_texture_filter_caps: PTFilterCaps::{MagFPoint|MagFLinear|MinFPoint|MinFLinear|MipFPoint|MipFLinear},
    ///     volume_texture_filter_caps: PTFilterCaps::{MagFPoint|MagFLinear|MagFAnisotropic|MinFPoint|MinFLinear|MinFAnisotropic|MipFPoint|MipFLinear},
    ///     texture_address_caps: PTAddressCaps::{Border|Clamp|IndependentUV|Mirror|MirrorOnce|Wrap},
    ///     volume_texture_address_caps: PTAddressCaps::{Border|Clamp|IndependentUV|Mirror|MirrorOnce|Wrap},
    ///     line_caps: LineCaps::{AlphaCmp|Blend|Fog|Texture|ZTest},
    ///     max_texture_width: 16384,
    ///     max_texture_height: 16384,
    ///     max_volume_extent: 8192,
    ///     max_texture_repeat: 8192,
    ///     max_texture_aspect_ratio: 8192,
    ///     max_anisotropy: 16,
    ///     max_vertex_w: 10000000000.0,
    ///     guard_band_left: -32768.0,
    ///     guard_band_top: -32768.0,
    ///     guard_band_right: 32768.0,
    ///     guard_band_bottom: 32768.0,
    ///     extents_adjust: 0.0,
    ///     stencil_caps: StencilCaps::{Keep|Zero|Replace|IncrSat|DecrSat|Invert|Incr|Decr|TwoSided},
    ///     fvf_caps: FvfCaps::{PSize|0x00000008},
    ///     texture_op_caps: TexOpCaps::{Add|AddSigned|AddSigned2x|AddSmooth|BlendCurrentAlpha|BlendDiffuseAlpha|BlendFactorAlpha|BlendTextureAlpha|BlendTextureAlphaPM|BumpEnvMap|BumpEnvMapLuminance|Disable|DotProduct3|Lerp|Modulate|Modulate2x|Modulate4x|ModulateAlphaAddColor|ModulateColorAddAlpha|ModulateInvAlphaAddColor|ModulateInvColorAddAlpha|MultiplyAdd|Premodulate|SelectArg1|SelectArg2|Subtract},
    ///     max_texture_blend_stages: 8,
    ///     max_simultaneous_textures: 8,
    ///     vertex_processing_caps: VtxPCaps::{DirectionalLights|LocalViewer|MaterialSource7|PositionalLights|TexGen|TexGenSphereMap|Tweening},
    ///     max_active_lights: 10,
    ///     max_user_clip_planes: 6,
    ///     max_vertex_blend_matrices: 4,
    ///     max_vertex_blend_matrix_index: 8,
    ///     max_point_size: 256.0,
    ///     max_primitive_count: 5592405,
    ///     max_vertex_index: 16777215,
    ///     max_streams: 16,
    ///     max_stream_stride: 508,
    ///     vertex_shader_version: ShaderVersion::VS_3_0,
    ///     max_vertex_shader_const: 256,
    ///     pixel_shader_version: ShaderVersion::PS_3_0,
    ///     pixel_shader_1x_max_value: 3.4028235e38,
    ///     dev_caps2: DevCaps2::{CanStretchRectFromTextures|PresampledDMapNPatch|StreamOffset|VertexElementsCanShareStreamOffset},
    ///     max_npatch_tessellation_level: 1.0,
    ///     reserved5: Reserved5(
    ///         0,
    ///     ),
    ///     master_adapter_ordinal: 0,
    ///     adapter_ordinal_in_group: 0,
    ///     number_of_adapters_in_group: 4,
    ///     decl_types: DtCaps::{UByte4|UByte4N|Short2N|Short4N|UShort2N|UShort4N|UDec3|Dec3N|Float16_2|Float16_4},
    ///     num_simultaneous_rts: 4,
    ///     stretch_rect_filter_caps: PTFilterCaps::{MagFPoint|MagFLinear|MinFPoint|MinFLinear},
    ///     vs_20_caps: VShaderCaps20 {
    ///         caps: Vs20Caps::Predication,
    ///         dynamic_flow_control_depth: 24,
    ///         num_temps: 32,
    ///         static_flow_control_depth: 4,
    ///     },
    ///     ps_20_caps: PShaderCaps20 {
    ///         caps: Ps20Caps::{ArbitrarySwizzle|GradientInstructions|Predication|NoDependentReadLimit|NoTexInstructionLimit},
    ///         dynamic_flow_control_depth: 24,
    ///         num_temps: 32,
    ///         static_flow_control_depth: 4,
    ///         num_instruction_slots: 512,
    ///     },
    ///     vertex_texture_filter_caps: PTFilterCaps::{MagFPoint|MagFLinear|MinFPoint|MinFLinear},
    ///     max_vshader_instructions_executed: 4294967295,
    ///     max_pshader_instructions_executed: 4294967295,
    ///     max_vertex_shader_30_instruction_slots: 32768,
    ///     max_pixel_shader_30_instruction_slots: 32768,
    /// }
    /// ```
    fn get_device_caps(&self) -> Result<Caps, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_device_caps => IDirect3DDevice9::GetDeviceCaps);
        let mut caps = Caps::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetDeviceCaps(caps.as_mut()) })?;
        Ok(caps)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdirect3d)\]
    /// IDirect3DDevice9::GetDirect3D
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([Direct3D])              otherwise
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let d3d : Direct3D = device.get_direct3d().unwrap();
    /// ```
    fn get_direct3d(&self) -> Result<Direct3D, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_direct3d => IDirect3DDevice9::GetDirect3D);
        let mut d3d = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetDirect3D(&mut d3d) })?;
        Ok(unsafe { Direct3D::from_raw(d3d) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdisplaymode)\]
    /// IDirect3DDevice9::GetDisplayMode
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([DisplayMode])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let mode : DisplayMode = device.get_display_mode(0).unwrap();
    /// println!("{:#?}", mode);
    /// assert!(mode.width > 0);
    /// assert_eq!(mode.format, Format::X8R8G8B8);
    /// ```
    ///
    /// ### Output
    /// ```text
    /// DisplayMode {
    ///     width: 2160,
    ///     height: 3840,
    ///     refresh_rate: 60,
    ///     format: Format(D3DFMT_X8R8G8B8),
    /// }
    /// ```
    fn get_display_mode(&self, swap_chain: u32) -> Result<DisplayMode, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_display_mode => IDirect3DDevice9::GetDisplayMode);
        let mut dm = DisplayMode::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetDisplayMode(swap_chain, &mut *dm) })?;
        Ok(dm)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getfrontbufferdata)\]
    /// IDirect3DDevice9::GetFrontBufferData
    fn get_front_buffer_data(&self, swap_chain: u32, surface: &Surface) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_front_buffer_data => IDirect3DDevice9::GetFrontBufferData);
        // TODO: verify soundness before making pub
        fn_check_hr!(unsafe { self.as_winapi().GetFrontBufferData(swap_chain, surface.as_raw()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getfvf)\]
    /// IDirect3DDevice9::GetFVF
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([FVF])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// assert_eq!(device.get_fvf().unwrap(), FVF::None);
    /// ```
    fn get_fvf(&self) -> Result<FVF, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_fvf => IDirect3DDevice9::GetFVF);
        let mut fvf = FVF::None;
        fn_check_hr!(unsafe { self.as_winapi().GetFVF(&mut *fvf) })?;
        Ok(fvf)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getgammaramp)\]
    /// IDirect3DDevice9::GetGammaRamp
    ///
    /// ### Returns
    /// *   [d3d::GammaRamp]
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let ramp = device.get_gamma_ramp(0);
    /// ```
    fn get_gamma_ramp(&self, swap_chain: u32) -> GammaRamp {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_gamma_ramp => IDirect3DDevice9::GetGammaRamp);
        let mut ramp = GammaRamp::zeroed();
        let _nohr : () = unsafe { self.as_winapi().GetGammaRamp(swap_chain, ramp.as_mut()) };
        ramp
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getindices)\]
    /// IDirect3DDevice9::GetIndices
    ///
    /// Retrieves index data.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(Some([IndexBuffer]))     if an index buffer was bound
    /// *   Ok(None)                    if no index buffer was bound
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// # let tri = device.create_index_buffer(3*2, Usage::None, Format::Index16, Pool::Default, ()).unwrap();
    /// let ib : Option<IndexBuffer> = device.get_indices().unwrap();
    /// assert!(ib.is_none(), "device has no index buffer by default");
    ///
    /// device.set_indices(Some(&tri));
    /// assert_eq!(tri.as_raw(), device.get_indices().unwrap().unwrap().as_raw());
    /// ```
    fn get_indices(&self) -> Result<Option<IndexBuffer>, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_indices => IDirect3DDevice9::GetIndices);
        let mut buffer = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetIndices(&mut buffer) })?;
        Ok(unsafe { IndexBuffer::from_raw_opt(buffer) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlight)\]
    /// IDirect3DDevice9::GetLight
    ///
    /// Get the [Light] that was previously set for this device.
    ///
    /// This API mirrors [set_light] by accepting [u16], instead of the underlying [u32].
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   - if no light was previously set at `index`
    /// *   Ok([Light])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// // Since there's no real way to clear previously set lights,
    /// // I recommend not treating untouched lights special:
    /// let light = device.get_light( 0).unwrap_or(Light::default());
    /// let light = device.get_light(!0).unwrap_or(Light::default());
    ///
    /// // That said, you can:
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light( 0));
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light(!0));
    /// device.set_light(0, Light::default()).unwrap();
    ///
    /// let light0 = device.get_light(0).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light(!0));
    /// ```
    ///
    /// [set_light]:    Self::set_light
    fn get_light(&self, index: u16) -> Result<Light, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_light => IDirect3DDevice9::GetLight);
        self.get_light_32(index.into())
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlight)\]
    /// IDirect3DDevice9::GetLight
    ///
    /// This API appears sound despite the 32-bit indicies
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   - if no light was previously set at `index`
    /// *   Ok([Light])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// for light in [0, 1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_32(light));
    /// }
    ///
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// device.set_light(0, light).unwrap();
    ///
    /// let light = device.get_light_32(0).unwrap();
    /// for light in [1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_32(light));
    /// }
    /// ```
    fn get_light_32(&self, index: u32) -> Result<Light, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_light_32 => IDirect3DDevice9::GetLight);
        let mut light = Light::default();
        fn_check_hr!(unsafe { self.as_winapi().GetLight(index, &mut *light) })?;
        Ok(light)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlightenable)
    /// IDirect3DDevice9::GetLightEnable
    ///
    /// Queries if the light is enabled.
    ///
    /// This API mirrors [light_enable] by accepting [u16], instead of the underlying [u32].
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If the light was never explicitly previously enabled or disabled
    /// *   [D3DERR::INVALIDCALL]       The device is a pure device?
    /// *   Ok(`true`)                  The light is enabled
    /// *   Ok(`false`)                 The light is disabled
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// // Since there's no real way to invalidate previously enabled/disabled lights,
    /// // I recommend not treating untouched lights special:
    /// let enabled = device.get_light_enable( 0).unwrap_or(false);
    /// let enabled = device.get_light_enable(!0).unwrap_or(false);
    ///
    /// // That said, you can:
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable( 0));
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable(!0));
    ///
    /// device.light_enable(!0, false).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable( 0));
    /// assert_eq!(false,               device.get_light_enable(!0).unwrap());
    /// ```
    ///
    /// [light_enable]:    Self::light_enable
    fn get_light_enable(&self, index: u16) -> Result<bool, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_light_enable => IDirect3DDevice9::GetLightEnable);
        self.get_light_enable_32(index.into())
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlightenable)\]
    /// IDirect3DDevice9::GetLightEnable
    ///
    /// This API appears sound despite the 32-bit indicies
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If the light was never explicitly previously enabled or disabled
    /// *   [D3DERR::INVALIDCALL]       The device is a pure device?
    /// *   Ok(`true`)                  The light is enabled
    /// *   Ok(`false`)                 The light is disabled
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// for light in [0, 1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable_32(light));
    /// }
    ///
    /// device.light_enable(0, false).unwrap();
    ///
    /// let enabled0 = device.get_light_enable_32(0).unwrap();
    /// assert_eq!(enabled0, false);
    /// for light in [1, 100, 10000, 1000000, !0].iter().copied() {
    ///     assert_eq!(D3DERR::INVALIDCALL, device.get_light_enable_32(light));
    /// }
    /// ```
    fn get_light_enable_32(&self, index: u32) -> Result<bool, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_light_enable_32 => IDirect3DDevice9::GetLightEnable);
        let mut enable = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetLightEnable(index, &mut enable) })?;
        Ok(enable != 0)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getmaterial)\]
    /// IDirect3DDevice9::GetMaterial
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok([Material])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let material = device.get_material().unwrap();
    /// ```
    fn get_material(&self) -> Result<Material, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_material => IDirect3DDevice9::GetMaterial);
        let mut material = Material::default();
        fn_check_hr!(unsafe { self.as_winapi().GetMaterial(&mut *material) })?;
        Ok(material)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getnpatchmode)\]
    /// IDirect3DDevice9::GetNPatchMode
    ///
    /// Gets the N-patch mode segments.
    ///
    /// Specifies the number of subdivision segments.
    /// If the number of segments is less than 1.0, N-patches are disabled.
    /// The default value is 0.0.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// assert_eq!(device.get_npatch_mode(), 0.0);
    /// ```
    fn get_npatch_mode(&self) -> f32 {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_npatch_mode => IDirect3DDevice9::GetNPatchMode);
        unsafe { self.as_winapi().GetNPatchMode() }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getnumberofswapchains)\]
    /// IDirect3DDevice9::GetNumberOfSwapChains
    ///
    /// Gets the number of implicit swap chains created for this device during [IDirect3D9Ext::create_device].
    ///
    /// (Does not include additional explicit swap chains created using [IDirect3DDevice9Ext::create_additional_swap_chain]?)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// assert_eq!(device.get_number_of_swap_chains(), 1);
    /// ```
    fn get_number_of_swap_chains(&self) -> u32 {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_number_of_swap_chains => IDirect3DDevice9::GetNumberOfSwapChains);
        unsafe { self.as_winapi().GetNumberOfSwapChains() }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpaletteentries)\]
    /// IDirect3DDevice9::GetPaletteEntries
    ///
    /// Retrieves palette entries.
    ///
    /// ### ⚠️ Safety ⚠️
    /// This function may crash if no palette was previously set!
    ///
    /// *   Windows version:      `10.0.19041.630`
    /// *   `d3d9.dll` version:   `10.0.19041.546`
    /// *   Driver version:       `24.20.11026.2001`
    /// *   Driver name:          `C:\Windows\System32\DriverStore\FileRepository\u0332836.inf_amd64_9f6b5ef5a1aed97e\B332771\aticfx64.dll,...`
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   "If the method fails"
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// // XXX: No palette set, this may crash!!!
    /// // let pal = unsafe { device.get_palette_entries_unchecked(0) }.unwrap();
    ///
    /// device.set_palette_entries(0, &[Color::argb(0xFF112233); 256]).unwrap();
    ///
    /// let pal = unsafe { device.get_palette_entries_unchecked(0) }.unwrap();
    /// assert_eq!(pal.len(), 256);
    /// assert_eq!(pal[  0], Color::argb(0xFF112233));
    /// assert_eq!(pal[255], Color::argb(0xFF112233));
    /// ```
    unsafe fn get_palette_entries_unchecked(&self, palette_number: u32) -> Result<[Color; 256], Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_palette_entries_unchecked => IDirect3DDevice9::GetPaletteEntries);
        // D3D9 uses PALETTEENTRYs but misuses the flags field.  D3DCOLORs are much better fits.
        let mut colors = [Color::argb(0); 256];
        fn_check_hr!(unsafe { self.as_winapi().GetPaletteEntries(palette_number, colors.as_mut_ptr().cast()) })?;
        Ok(colors)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpixelshader)\]
    /// IDirect3DDevice9::GetPixelShader
    ///
    /// Gets the pixel shader currently bound to the device, if any.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If the device was created with D3DCREATE_PUREDEVICE?
    /// *   Ok(Some([PixelShader]))     If a pixel shader was bound
    /// *   Ok(None)                    If no pixel shader was bound
    fn get_pixel_shader(&self) -> Result<PixelShader, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_pixel_shader => IDirect3DDevice9::GetPixelShader);
        let mut shader = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetPixelShader(&mut shader) })?;
        Ok(unsafe { PixelShader::from_raw(shader) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpixelshaderconstantb)\]
    /// IDirect3DDevice9::GetPixelShaderConstantB
    ///
    /// Gets boolean shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // ps_3_0 (d3d9 max) only supports 16 boolean registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-ps-registers-ps-3-0
    /// let mut constants = [abibool::bool32::FALSE; 16];
    /// let mut too_many  = [abibool::bool32::FALSE; 16+1];
    ///
    /// device.get_pixel_shader_constant_b(0, &mut constants).unwrap();         // b0 .. b16
    /// device.get_pixel_shader_constant_b(8, &mut constants[8..]).unwrap();    // b8 .. b16
    ///
    /// // Out of bounds:
    /// let r1 = device.get_pixel_shader_constant_b(!0, &mut constants);        // b-1.. b15
    /// let r2 = device.get_pixel_shader_constant_b(0, &mut too_many);          // b0 .. b17
    /// let r3 = device.get_pixel_shader_constant_b(1, &mut constants);         // b1 .. b17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn get_pixel_shader_constant_b(&self, start_register: u32, constant_data: &mut [bool32]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_pixel_shader_constant_b => IDirect3DDevice9::GetPixelShaderConstantB);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetPixelShaderConstantB(start_register, constant_data.as_mut_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpixelshaderconstantf)\]
    /// IDirect3DDevice9::GetPixelShaderConstantF
    ///
    /// Gets floating-point shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // ps_3_0 (d3d9 max) supports 224 float4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-ps-registers-ps-3-0
    /// let cmax = 224;
    /// let mut constants = vec![[0.0, 0.0, 0.0, 0.0]; cmax];
    /// let mut too_many  = vec![[0.0, 0.0, 0.0, 0.0]; cmax+1];
    ///
    /// device.get_pixel_shader_constant_f(0, &mut constants).unwrap();         // c0 .. c224
    /// device.get_pixel_shader_constant_f(8, &mut constants[8..]).unwrap();    // c8 .. c224
    ///
    /// // Out of bounds:
    /// let r1 = device.get_pixel_shader_constant_f(!0, &mut constants);        // c-1.. c223
    /// let r2 = device.get_pixel_shader_constant_f(0, &mut too_many);          // c0 .. c225
    /// let r3 = device.get_pixel_shader_constant_f(1, &mut constants);         // c1 .. c225
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn get_pixel_shader_constant_f(&self, start_register: u32, constant_data: &mut [[f32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_pixel_shader_constant_f => IDirect3DDevice9::GetPixelShaderConstantF);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetPixelShaderConstantF(start_register, constant_data.as_mut_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpixelshaderconstanti)\]
    /// IDirect3DDevice9::GetPixelShaderConstantI
    ///
    /// Gets integer shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max integer vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // ps_3_0 (d3d9 max) only supports 16 int4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-ps-registers-ps-3-0
    /// let mut constants = [[0, 0, 0, 0]; 16];
    /// let mut too_many  = [[0, 0, 0, 0]; 16+1];
    ///
    /// device.get_pixel_shader_constant_i(0, &mut constants).unwrap();         // i0 .. i16
    /// device.get_pixel_shader_constant_i(8, &mut constants[8..]).unwrap();    // i8 .. i16
    ///
    /// // Out of bounds:
    /// let r1 = device.get_pixel_shader_constant_i(!0, &mut constants);        // i-1.. i15
    /// let r2 = device.get_pixel_shader_constant_i(0, &mut too_many);          // i0 .. i17
    /// let r3 = device.get_pixel_shader_constant_i(1, &mut constants);         // i1 .. i17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn get_pixel_shader_constant_i(&self, start_register: u32, constant_data: &mut [[i32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_pixel_shader_constant_i => IDirect3DDevice9::GetPixelShaderConstantI);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetPixelShaderConstantI(start_register, constant_data.as_mut_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrasterstatus)\]
    /// IDirect3DDevice9::GetRasterStatus
    ///
    /// Returns information describing the [RasterStatus] of the monitor on which the swap chain is presented.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `swap_chain` is not a valid swap chain.
    /// *   Ok([RasterStatus])      If `swap_chain` is a valid swap chain.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let swap_chains = device.get_number_of_swap_chains();
    /// for i in 0 .. swap_chains {
    ///     dbg!(device.get_raster_status(i).unwrap());
    /// }
    /// # for i in (swap_chains .. 255).chain((8 .. 32).map(|pow| 1<<pow)) {
    /// #   assert_eq!(D3DERR::INVALIDCALL, device.get_raster_status(i));
    /// # }
    /// ```
    ///
    /// ### Output
    /// ```text
    /// RasterStatus {
    ///     in_vblank: false,
    ///     scan_line: 3082,
    /// }
    /// ```
    fn get_raster_status(&self, swap_chain: u32) -> Result<RasterStatus, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_raster_status => IDirect3DDevice9::GetRasterStatus);
        let mut raster_status = RasterStatus::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetRasterStatus(swap_chain, &mut *raster_status) })?;
        Ok(raster_status)
    }

    //TODO: get_render_state (typed)

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrenderstate)\]
    /// IDirect3DDevice9::GetRenderState
    ///
    /// Retrieves a render-state value for a device.
    ///
    /// May fail for pure devices.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `state` is not a valid render state
    /// *   Ok([u32])               If `state` has a value
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// assert_eq!(
    ///     device.get_render_state_untyped(RS::ZEnable).unwrap() != 0,
    ///     false
    /// );
    ///
    /// assert_eq!(
    ///     device.get_render_state_untyped(RS::FillMode).map(FillMode::from_unchecked).unwrap(),
    ///     FillMode::Solid
    /// );
    ///
    /// assert_eq!(
    ///     device.get_render_state_untyped(RS::from_unchecked(!0)),
    ///     D3DERR::INVALIDCALL
    /// );
    /// #
    /// # for s in (0 .. 256).chain((8..32).map(|pow| 1<<pow)).map(|i| RenderStateType::from_unchecked(i)) {
    /// #   if let Err(err) = device.get_render_state_untyped(s) {
    /// #       assert_eq!(err, D3DERR::INVALIDCALL);
    /// #   }
    /// # }
    /// ```
    fn get_render_state_untyped(&self, state: RenderStateType) -> Result<u32, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_render_state_untyped => IDirect3DDevice9::GetRenderState);
        let mut value = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetRenderState(state.into(), &mut value) })?;
        Ok(value)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertarget)\]
    /// IDirect3DDevice9::GetRenderTarget
    ///
    /// Typically, methods that return state will not work on a device that is created using D3DCREATE_PUREDEVICE. This method however, will work even on a pure device because it returns an interface.
    /// The device can now support multiple render targets. The number of render targets supported by a device is contained in the NumSimultaneousRTs member of [Caps]. See [Multiple Render Targets (Direct3D 9)].
    ///
    /// ### Returns
    /// *   [D3DERR]::???             `render_target_index` > [Caps].NumSimultaneousRTs ?
    /// *   Ok(Some([Surface]))       the render target bound to that index
    /// *   Ok(None)                  no render target was bound to that index
    ///
    /// [Multiple Render Targets (Direct3D 9)]:         https://learn.microsoft.com/en-us/windows/win32/direct3d9/multiple-render-targets
    fn get_render_target(&self, render_target_index: u32) -> Result<Option<Surface>, Error> {
        // TODO: verify soundness before making pub
        fn_context!(d3d9::IDirect3DDevice9Ext::get_render_target => IDirect3DDevice9::GetRenderTarget);
        let mut rt = null_mut();
        let hr = unsafe { self.as_winapi().GetRenderTarget(render_target_index, &mut rt) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            fn_check_hr!(hr)?;
            Ok(unsafe { Surface::from_raw_opt(rt) })
        }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertargetdata)\]
    /// IDirect3DDevice9::GetRenderTargetData
    ///
    /// Copies the render-target data from device memory to system memory.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If the source and destination formats do not match
    /// *   [D3DERR]::???           If the source and destination sizes do not match
    /// *   [D3DERR]::???           If the source is multisampled
    /// *   [D3DERR]::???           If the source is not a regular render target, or level thereof, created with [Pool::Default]
    /// *   [D3DERR]::???           If the destination is not an off-screen plain surface, or level of a texture created with [Pool::SystemMem]
    ///
    fn get_render_target_data(&self, render_target: &Surface, dest_surface: &Surface) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_render_target_data => IDirect3DDevice9::GetRenderTargetData);
        fn_check_hr!(unsafe { self.as_winapi().GetRenderTargetData(render_target.as_raw(), dest_surface.as_raw()) })
    }

    //TODO: get_sampler_state (typed)

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsamplerstate)\]
    /// IDirect3DDevice9::GetSamplerState
    ///
    /// Retrieves a sampler state value for a device.
    /// Prefer [`get_sampler_state_untyped`](Self::get_sampler_state_untyped).
    ///
    /// May fail for pure devices.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `ty` is not bounds checked by thindx nor DirectX, and will cause undefined behavior if invalid!
    /// *   `sampler` is not bounds checked by thindx.  DirectX *might* bounds check, but has concerningly inconsistent behavior if `sampler >= 16`.
    ///
    /// ### Returns
    /// *   Ok([u32])           If `(sampler, state)` has a value
    /// *   Ok(0xBAADCAFE)      If `sampler >= 16` (on some systems)
    /// *   Ok(sane default)    If `sampler >= 16` (on some systems)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let sampler : u32;
    /// // ...
    /// # sampler = 0;
    /// # let _ = sampler;
    /// # for sampler in (0 .. 256).chain((8 .. 32).map(|pow| 1<<pow)) {
    ///
    /// if sampler >= 16 && unsafe { device.get_sampler_state_unchecked(sampler, Samp::AddressU) }.unwrap() == 0xBAADCAFE { continue }
    ///
    /// assert_eq!(
    ///     unsafe { device.get_sampler_state_unchecked(sampler, Samp::AddressU) }.map(TextureAddress::from_unchecked).unwrap(),
    ///     TAddress::Wrap
    /// );
    ///
    /// assert_eq!(
    ///     unsafe { device.get_sampler_state_unchecked(sampler, Samp::BorderColor) }.map(Color::argb).unwrap(),
    ///     Color::argb(0x00000000),
    /// );
    ///
    /// if false {
    ///     // undefined behavior (invalid sampler):
    ///     let _ = unsafe { device.get_sampler_state_unchecked(sampler, Samp::from_unchecked(!0)) };
    /// }
    /// # }
    /// ```
    unsafe fn get_sampler_state_unchecked(&self, sampler: u32, ty: SamplerStateType) -> Result<u32, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_sampler_state_unchecked => IDirect3DDevice9::GetSamplerState);
        let mut value = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetSamplerState(sampler, ty.into(), &mut value) })?;
        Ok(value)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsamplerstate)\]
    /// IDirect3DDevice9::GetSamplerState
    ///
    /// Retrieves a sampler state value for a device.
    ///
    /// May fail for pure devices.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `state` is not a normal valid render state
    /// *   [D3DERR::INVALIDCALL]   If `sampler >= 16` (max [sampler](https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-ps-registers-sampler) registers for ps_3_0 / D3D9)
    /// *   Ok([u32])               If `(sampler, state)` has a (possibly default) value
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let sampler : u32;
    /// // ...
    /// # sampler = 0;
    /// # let _ = sampler;
    /// # for sampler in (0 .. 256).chain((8 .. 32).map(|pow| 1<<pow)) {
    ///
    /// if sampler >= 16 {
    ///     assert_eq!(
    ///         device.get_sampler_state_untyped(sampler, Samp::AddressU),
    ///         D3DERR::INVALIDCALL
    ///     );
    /// } else {
    ///     assert_eq!(
    ///         device.get_sampler_state_untyped(sampler, Samp::AddressU).map(TextureAddress::from_unchecked).unwrap(),
    ///         TAddress::Wrap
    ///     );
    ///
    ///     assert_eq!(
    ///         device.get_sampler_state_untyped(sampler, Samp::BorderColor).map(Color::argb).unwrap(),
    ///         Color::argb(0x00000000),
    ///     );
    ///
    ///     assert_eq!(
    ///         device.get_sampler_state_untyped(sampler, Samp::from_unchecked(!0)),
    ///         D3DERR::INVALIDCALL
    ///     );
    /// }
    /// #
    /// # for s in (0 .. 256).chain((8..32).map(|pow| 1<<pow)).map(|i| SamplerStateType::from_unchecked(i)) {
    /// #   let r = device.get_sampler_state_untyped(sampler, s);
    /// #   dbg!((sampler, s, r));
    /// #   if let Err(err) = device.get_sampler_state_untyped(sampler, s) {
    /// #       assert_eq!(err, D3DERR::INVALIDCALL);
    /// #   }
    /// # }
    /// # }
    /// ```
    fn get_sampler_state_untyped(&self, sampler: u32, ty: SamplerStateType) -> Result<u32, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_sampler_state_untyped => IDirect3DDevice9::GetSamplerState);
        if !matches!(ty.into(), 1 ..= 13) { return Err(fn_param_error!(ty, D3DERR::INVALIDCALL)) } // SamplerStateType::from_unchecked isn't `unsafe` so we must bounds check here
        if sampler >= 16 { return Err(fn_param_error!(sampler, D3DERR::INVALIDCALL)); }
        unsafe { self.get_sampler_state_unchecked(sampler, ty) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getscissorrect)\]
    /// IDirect3DDevice9::GetScissorRect
    ///
    /// ### Returns
    /// *   Ok([Rect])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let rect = device.get_scissor_rect().unwrap();
    /// dbg!(rect); // ex: Rect { x: 0..784, y: 0..561 }
    /// ```
    fn get_scissor_rect(&self) -> Result<Rect, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_scissor_rect => IDirect3DDevice9::GetScissorRect);
        let mut rect = Rect::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetScissorRect(rect.as_mut()) })?;
        Ok(rect)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsoftwarevertexprocessing)\]
    /// IDirect3DDevice9::GetSoftwareVertexProcessing
    ///
    /// ### Returns
    /// *   `true`      if in software processing mode
    /// *   `false`     if in hardware processing mode
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let software_mode : bool = device.get_software_vertex_processing();
    /// let hardware_mode : bool = !software_mode;
    /// assert!(hardware_mode);
    /// ```
    fn get_software_vertex_processing(&self) -> bool {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_software_vertex_processing => IDirect3DDevice9::GetSoftwareVertexProcessing);
        unsafe { self.as_winapi().GetSoftwareVertexProcessing() != 0 }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getstreamsource)\]
    /// IDirect3DDevice9::GetStreamSource
    ///
    /// Retrieves a vertex buffer bound to the specified data stream.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(Some([VertexBuffer]), `offset_in_bytes`, `stride`)
    /// *   Ok(`(None, 0, 0)`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // No stream bound to start
    /// let (vb, offset, stride) = device.get_stream_source(0).unwrap();
    /// assert!(vb.is_none());
    /// assert_eq!(offset, 0);
    /// assert_eq!(stride, 0);
    ///
    /// let tri = device.create_vertex_buffer(3*4*3, Usage::None, FVF::XYZ, Pool::Default, ()).unwrap();
    /// device.set_stream_source(0, &tri, 0, 4*3).unwrap(); // bind the vertex buffer
    ///
    /// // No stream bound to start
    /// let (vb, offset, stride) = device.get_stream_source(0).unwrap();
    /// assert_eq!(vb.map(|vb| vb.as_raw()), Some(tri.as_raw()));
    /// assert_eq!(offset, 0);
    /// assert_eq!(stride, 4*3);
    /// ```
    fn get_stream_source(&self, stream_number: u32) -> Result<(Option<VertexBuffer>, u32, u32), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_stream_source => IDirect3DDevice9::GetStreamSource);
        let mut buffer = null_mut();
        let mut offset = 0;
        let mut stride = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetStreamSource(stream_number, &mut buffer, &mut offset, &mut stride) })?;
        let buffer = unsafe { VertexBuffer::from_raw_opt(buffer) };
        Ok((buffer, offset, stride))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getstreamsourcefreq)\]
    /// IDirect3DDevice9::GetStreamSourceFreq
    ///
    /// Gets the stream source frequency divider value.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([StreamSource])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// assert_eq!(device.get_stream_source_freq(0).unwrap(), StreamSource::regular());
    /// assert_eq!(device.get_stream_source_freq(1).unwrap(), StreamSource::regular());
    ///
    /// device.set_stream_source_freq(0, StreamSource::indexed_data(100)).unwrap();
    /// device.set_stream_source_freq(1, StreamSource::instance_data(1)).unwrap();
    ///
    /// assert_eq!(device.get_stream_source_freq(0).unwrap(), StreamSource::indexed_data(100));
    /// assert_eq!(device.get_stream_source_freq(1).unwrap(), StreamSource::instance_data(1));
    /// ```
    fn get_stream_source_freq(&self, stream_number: u32) -> Result<StreamSource, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_stream_source_freq => IDirect3DDevice9::GetStreamSourceFreq);
        let mut freq = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetStreamSourceFreq(stream_number, &mut freq) })?;
        Ok(StreamSource::from_unchecked(freq))
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getswapchain)\]
    /// IDirect3DDevice9::GetSwapChain
    ///
    /// Gets a pointer to a swap chain.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([SwapChain])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// for swap_chain in 0 .. device.get_number_of_swap_chains() {
    ///     let swap_chain = device.get_swap_chain(swap_chain).unwrap();
    /// }
    /// # for swap_chain in (device.get_number_of_swap_chains() .. 255).chain((8..32).map(|pow| 1<<pow)) {
    /// #   assert_eq!(D3DERR::INVALIDCALL, device.get_swap_chain(swap_chain).map(|_|()));
    /// # }
    /// ```
    fn get_swap_chain(&self, swap_chain: u32) -> Result<SwapChain, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_swap_chain => IDirect3DDevice9::GetSwapChain);
        let mut sc = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetSwapChain(swap_chain, &mut sc) })?;
        let swap_chain = unsafe { SwapChain::from_raw(sc) };
        Ok(swap_chain)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-gettexture)\]
    /// IDirect3DDevice9::GetTexture
    ///
    /// Retrieves a texture assigned to a stage for a device.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   This function will crash (or worse!) if `set_texture` was never called for `stage`!
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If the device is a pure device?
    /// *   Ok(Some([BaseTexture]))     If a texture was bound to that stage
    /// *   Ok(None)                    If no texture was bound to that stage
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // XXX: No texture set for stage 0, this may crash!!!
    /// // let texture = unsafe { device.get_texture(0) }.unwrap();
    ///
    /// unsafe { device.set_texture(0, None) }.unwrap();
    /// let texture0 = unsafe { device.get_texture(0) };
    /// assert!(texture0.unwrap().is_none());
    ///
    /// // XXX: No texture set for stage 1, this may crash!!!
    /// // let texture = unsafe { device.get_texture(1) }.unwrap();
    ///
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// unsafe { device.set_texture(0, &texture) }.unwrap();
    /// assert_eq!(unsafe { device.get_texture(0) }.unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// ```
    unsafe fn get_texture(&self, stage: u32) -> Result<Option<BaseTexture>, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_texture => IDirect3DDevice9::GetTexture);
        let mut texture = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetTexture(stage, &mut texture) })?;
        Ok(unsafe { BaseTexture::from_raw_opt(texture) })
    }

    //TODO: get_texture_stage_state (typed)

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-gettexturestagestate)\]
    /// IDirect3DDevice9::GetTextureStageState
    ///
    /// Retrieves the value associated with a [TextureStageStateType]
    ///
    /// ### Returns
    /// *   Ok([u32])
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let value = device.get_texture_stage_state_untyped(0, TSS::ColorArg1).unwrap();
    /// let value = device.get_texture_stage_state_untyped(0, TSS::TexCoordIndex).unwrap();
    ///
    /// let value = device.get_texture_stage_state_untyped(7, TSS::ColorArg1).unwrap();
    /// let value = device.get_texture_stage_state_untyped(8, TSS::ColorArg1).unwrap(); // out of bounds `stage`
    /// let value = device.get_texture_stage_state_untyped(0, TSS::from_unchecked(0)).unwrap(); // invalid `ty`
    /// let value = device.get_texture_stage_state_untyped(0, TSS::from_unchecked(!0)).unwrap(); // out of bounds `ty`
    /// #
    /// # for stage in (0 .. 256).chain((8..32).map(|pow| 1<<pow)) {
    /// #   for tss in (0 .. 256).chain((8..32).map(|pow| 1<<pow)) {
    /// #       if let Err(err) = device.get_texture_stage_state_untyped(stage, TSS::from_unchecked(tss)) {
    /// #           assert_eq!(err, D3DERR::INVALIDCALL);
    /// #       }
    /// #   }
    /// # }
    /// ```
    fn get_texture_stage_state_untyped(&self, stage: u32, ty: impl Into<TextureStageStateType>) -> Result<u32, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_texture_stage_state_untyped => IDirect3DDevice9::GetTextureStageState);
        let mut value = 0;
        fn_check_hr!(unsafe { self.as_winapi().GetTextureStageState(stage, ty.into().into(), &mut value) })?;
        Ok(value)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-gettransform)\]
    /// IDirect3DDevice9::GetTransform
    ///
    /// Gets a single device transform.
    ///
    /// May fail for pure devices.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If the specified transform state type is invalid.
    /// *   Ok([d3d::Matrix])
    ///
    /// ### Examples
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let m = device.get_transform(d3d::TS::View).unwrap();
    /// assert_eq!(m.m, d3d::Matrix::identity().m);
    ///
    /// let m = device.get_transform(d3d::TS::world_matrix(255)).unwrap();
    /// assert_eq!(m.m, d3d::Matrix::identity().m);
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_transform(d3d::TS::from_unchecked(0xFFFFFFF)));
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_transform(d3d::TS::from_unchecked(0x0FFFF00)));
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_transform(d3d::TS::from_unchecked(0x1000000)));
    /// assert_eq!(D3DERR::INVALIDCALL, device.get_transform(d3d::TS::from_unchecked(0x10000FF)));
    /// ```
    fn get_transform(&self, ts: impl Into<TransformStateType>) -> Result<d3d::Matrix, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_transform => IDirect3DDevice9::GetTransform);
        let mut matrix = Matrix::zeroed();
        fn_check_hr!(unsafe { self.as_winapi().GetTransform(ts.into().into(), matrix.as_mut()) })?;
        Ok(matrix)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexdeclaration)\]
    /// IDirect3DDevice9::GetVertexDeclaration
    ///
    /// Gets the currently bound [VertexDeclaration], or None if none was set.
    ///
    /// ### Returns
    /// *   Ok(Some([VertexDeclaration]))   If a vertex declaration was set
    /// *   Ok(None)                        If no vertex declaration was set
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// # let some_decl = device.create_vertex_declaration(&[
    /// #     VertexElement::new(0,  0, DeclType8::Float4, DeclMethod8::Default, DeclUsage8::Position, 0),
    /// #     VertexElement::new(0, 16, DeclType8::Float2, DeclMethod8::Default, DeclUsage8::TexCoord, 0),
    /// #     VertexElement::END
    /// # ]).unwrap();
    /// let decl1 = device.get_vertex_declaration().unwrap();
    /// assert!(decl1.is_none());
    ///
    /// device.set_vertex_declaration(Some(&some_decl)).unwrap();
    ///
    /// let decl2 = device.get_vertex_declaration().unwrap().unwrap();
    /// assert_eq!(some_decl.as_raw(), decl2.as_raw());
    /// ```
    fn get_vertex_declaration(&self) -> Result<Option<VertexDeclaration>, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_vertex_declaration => IDirect3DDevice9::GetVertexDeclaration);
        let mut vd = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetVertexDeclaration(&mut vd) })?;
        Ok(unsafe { VertexDeclaration::from_raw_opt(vd) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getvertexshader)\]
    /// IDirect3DDevice9::GetVertexShader
    ///
    /// Gets the vertex shader currently bound to the device, if any.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If the device was created with D3DCREATE_PUREDEVICE?
    /// *   Ok(Some([VertexShader]))    If a vertex shader was bound
    /// *   Ok(None)                    If no vertex shader was bound
    fn get_vertex_shader(&self) -> Result<VertexShader, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_vertex_shader => IDirect3DDevice9::GetVertexShader);
        let mut shader = null_mut();
        fn_check_hr!(unsafe { self.as_winapi().GetVertexShader(&mut shader) })?;
        Ok(unsafe { VertexShader::from_raw(shader) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getvertexshaderconstantb)\]
    /// IDirect3DDevice9::GetVertexShaderConstantB
    ///
    /// Gets boolean shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // vs_3_0 (d3d9 max) only supports 16 boolean registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-vs-registers-vs-3-0
    /// let mut constants = [abibool::bool32::FALSE; 16];
    /// let mut too_many  = [abibool::bool32::FALSE; 16+1];
    ///
    /// device.get_vertex_shader_constant_b(0, &mut constants).unwrap();        // b0 .. b16
    /// device.get_vertex_shader_constant_b(8, &mut constants[8..]).unwrap();   // b8 .. b16
    ///
    /// // Out of bounds:
    /// let r1 = device.get_vertex_shader_constant_b(!0, &mut constants);       // b-1.. b15
    /// let r2 = device.get_vertex_shader_constant_b(0, &mut too_many);         // b0 .. b17
    /// let r3 = device.get_vertex_shader_constant_b(1, &mut constants);        // b1 .. b17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn get_vertex_shader_constant_b(&self, start_register: u32, constant_data: &mut [bool32]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_vertex_shader_constant_b => IDirect3DDevice9::GetVertexShaderConstantB);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetVertexShaderConstantB(start_register, constant_data.as_mut_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getvertexshaderconstantf)\]
    /// IDirect3DDevice9::GetVertexShaderConstantF
    ///
    /// Gets floating-point shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // vs_3_0 (d3d9 max) only supports at least 256 float4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-vs-registers-vs-3-0
    /// let cmax = device.get_device_caps().unwrap().max_vertex_shader_const as usize;
    /// let mut constants = vec![[0.0, 0.0, 0.0, 0.0]; cmax];
    /// let mut too_many  = vec![[0.0, 0.0, 0.0, 0.0]; cmax+1];
    ///
    /// device.get_vertex_shader_constant_f(0, &mut constants).unwrap();        // c0 .. cMAX
    /// device.get_vertex_shader_constant_f(8, &mut constants[8..]).unwrap();   // c8 .. cMAX
    ///
    /// // Out of bounds:
    /// let r1 = device.get_vertex_shader_constant_f(!0, &mut constants);       // c-1.. cMAX-1
    /// let r2 = device.get_vertex_shader_constant_f(0, &mut too_many);         // c0 .. cMAX+1
    /// let r3 = device.get_vertex_shader_constant_f(1, &mut constants);        // c1 .. cMAX+1
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn get_vertex_shader_constant_f(&self, start_register: u32, constant_data: &mut [[f32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_vertex_shader_constant_f => IDirect3DDevice9::GetVertexShaderConstantF);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetVertexShaderConstantF(start_register, constant_data.as_mut_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getvertexshaderconstanti)\]
    /// IDirect3DDevice9::GetVertexShaderConstantI
    ///
    /// Gets integer shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max integer vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // vs_3_0 (d3d9 max) only supports 16 int4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-vs-registers-vs-3-0
    /// let mut constants = [[0, 0, 0, 0]; 16];
    /// let mut too_many  = [[0, 0, 0, 0]; 16+1];
    ///
    /// device.get_vertex_shader_constant_i(0, &mut constants).unwrap();        // i0 .. i16
    /// device.get_vertex_shader_constant_i(8, &mut constants[8..]).unwrap();   // i8 .. i16
    ///
    /// // Out of bounds:
    /// let r1 = device.get_vertex_shader_constant_i(!0, &mut constants);       // i-1.. i15
    /// let r2 = device.get_vertex_shader_constant_i(0, &mut too_many);         // i0 .. i17
    /// let r3 = device.get_vertex_shader_constant_i(1, &mut constants);        // i1 .. i17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn get_vertex_shader_constant_i(&self, start_register: u32, constant_data: &mut [[i32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_vertex_shader_constant_i => IDirect3DDevice9::GetVertexShaderConstantI);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().GetVertexShaderConstantI(start_register, constant_data.as_mut_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getviewport)\]
    /// IDirect3DDevice9::GetViewport
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If this is a pure device?
    /// *   [D3DERR::INVALIDCALL]   If the viewport is invalid
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let viewport : Viewport = device.get_viewport().unwrap();
    /// ```
    fn get_viewport(&self) -> Result<Viewport, Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::get_viewport => IDirect3DDevice9::GetViewport);
        let mut viewport = Viewport::default();
        fn_check_hr!(unsafe { self.as_winapi().GetViewport(&mut *viewport) })?;
        Ok(viewport)
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-lightenable)\]
    /// IDirect3DDevice9::LightEnable
    ///
    /// Enables or disables a set of lighting parameters within a device.
    ///
    /// For soundness, this limits `index` to [u16], instead of accepting [u32] like the underlying API does.
    ///
    /// ### Returns
    /// *   <span style="opacity: 25%">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.light_enable(0,  true).unwrap(); // Ok
    /// device.light_enable(1,  true).unwrap(); // Ok
    /// device.light_enable(!0, true).unwrap(); // Ok (16-bit)
    /// ```
    fn light_enable(&self, index: u16, enable: bool) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::light_enable => IDirect3DDevice9::LightEnable);
        // Safe: `index` max == `u16::MAX` == `0xFFFF`, well bellow when GArrayT starts buffer overflowing (`0x1000_0000 - 8`)
        unsafe { self.light_enable_32_unchecked(index.into(), enable) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-lightenable)\]
    /// IDirect3DDevice9::LightEnable
    ///
    /// Enables or disables a set of lighting parameters within a device.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   This will buffer overflow and crash (or worse!) if `index` >= `0x1000_0000 - 8` on some systems!
    /// *   Other, smaller sizes may also crash on other system I haven't tested.
    /// *   Prefer [light_enable] (limits the index to [u16], and is thus 100% infalliable.)
    ///
    /// [light_enable]:                Self::light_enable
    ///
    /// ### Returns
    /// *   <span style="opacity: 25%">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// unsafe {
    ///     device.light_enable_32_unchecked(0, true).unwrap();
    ///     device.light_enable_32_unchecked(0, false).unwrap();
    ///     // device.light_enable_32_unchecked(!0, true).unwrap();
    ///     // XXX: Buffer overflow, crash, or worse!
    /// }
    /// ```
    unsafe fn light_enable_32_unchecked(&self, index: u32, enable: bool) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::light_enable_32_unchecked => IDirect3DDevice9::LightEnable);
        fn_check_hr!(unsafe { self.as_winapi().LightEnable(index, enable.into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-multiplytransform)\]
    /// IDirect3DDevice9::MultiplyTransform
    ///
    /// Multiplies a device's world, view, or projection matrices by a specified matrix.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `ts` is not a valid [TransformStateType].
    /// *   Ok(())
    ///
    /// ### Examples
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.multiply_transform(d3d::TS::View,  d3d::Matrix::identity()).unwrap();
    /// device.multiply_transform(d3d::TS::World, d3d::Matrix::identity()).unwrap();
    /// device.multiply_transform(d3d::TS::world_matrix(255), d3d::Matrix::identity()).unwrap();
    ///
    /// assert_eq!(D3DERR::INVALIDCALL, device.multiply_transform(d3d::TS::from_unchecked(0xFFFFFFF), d3d::Matrix::identity()));
    /// # assert_eq!(D3DERR::INVALIDCALL, device.multiply_transform(d3d::TS::from_unchecked(0x0FFFF00), d3d::Matrix::identity()));
    /// # assert_eq!(D3DERR::INVALIDCALL, device.multiply_transform(d3d::TS::from_unchecked(0x1000000), d3d::Matrix::identity()));
    /// # assert_eq!(D3DERR::INVALIDCALL, device.multiply_transform(d3d::TS::from_unchecked(0x10000FF), d3d::Matrix::identity()));
    /// ```
    fn multiply_transform(&self, ts: impl Into<TransformStateType>, matrix: impl Into<Matrix>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::multiply_transform => IDirect3DDevice9::MultiplyTransform);
        fn_check_hr!(unsafe { self.as_winapi().MultiplyTransform(ts.into().into(), &matrix.into().into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-present)\]
    /// IDirect3DDevice9::Present
    ///
    /// Presents the contents of the next buffer in the sequence of back buffers owned by the device.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   It's likely unsound to use an invalid, non-null `hwnd`
    /// *   It's likely unsound to use a null `hwnd` if the original `presentation_parameters.hDeviceWindow` is an invalid, non-null HWND
    /// *   Out of bounds rects might also be an issue IDK?
    ///
    /// ### Arguments
    /// *   `source_rect`           - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire source surface is presented.)
    /// *   `dest_rect`             - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire client area is filled.)
    /// *   `dest_window_override`  - The destination window to render to.  If null / `()`, the runtime uses the `device_window` member of [PresentParameters] for the presentation.
    /// *   `dirty_region`          - "Must be [None]" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be [None] even then (the entire region will be considered dirty.)  The implementation is free to copy more than the exact dirty region.
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICEREMOVED]     When you least expect it
    /// *   [D3DERR::DEVICELOST]        When switching into/out-of fullscreen, or when invoking `C:\Windows\System32\DXCap.exe -forcetdr`
    /// *   [D3DERR::INVALIDCALL]       If called within a [IDirect3DDevice9Ext::begin_scene] .. [IDirect3DDevice9Ext::end_scene] section, if the render target is the current render target.
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use std::ptr::null_mut;   let hwnd = null_mut();
    /// # use dev::d3d9::*;               let device = device_test();
    /// // Present the entire back buffer (should work with all swap chains, probably:)
    /// device.present(.., .., (), None).unwrap();
    /// // TODO: Handle D3DERR::DEVICEREMOVED
    ///
    /// // Or, with a SwapEffect::Copy swap chain, this should succeed (might succeed by simply ignoring the args, even for other SwapEffect s:)
    /// let hwnd = unsafe { SafeHWND::assert(&hwnd) };
    /// let r = device.present((0,0)..(100,100), Rect::from((0,0)..(100,100)), hwnd, None);
    /// if r.is_ok() {} // Huzzah!
    /// else if r == D3DERR::DEVICEREMOVED { /* oooh, a removable GPU?  Nifty!  Might switch to the laptop's built-in Device (might have different caps!) */ }
    /// else if r == D3DERR::DEVICELOST    { /* switching fullscreen modes? GPU driver crashed? might prompt the user before recreating the device to avoid hang loops */ }
    /// else if r == D3DERR::DEVICEHUNG    { /* ...is it your fault? crazy shaders? might prompt the user before recreating the device to avoid hang loops */ }
    /// else                               { panic!("oh no something bad happened: {r:?}") }
    /// ```
    fn present<'r>(&self, source_rect: impl IntoRectOrFull, dest_rect: impl IntoRectOrFull, dest_window_override: impl AsHWND, dirty_region: impl Into<Option<&'r RgnData>>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::present => IDirect3DDevice9::Present);
        let source_rect     = source_rect.into_rect();
        let dest_rect       = dest_rect.into_rect();
        let hwnd            = dest_window_override.as_hwnd();
        let dirty_region    = dirty_region.into();

        let source_rect     = source_rect   .map_or(null(), |r| &*r).cast();
        let dest_rect       = dest_rect     .map_or(null(), |r| &*r).cast();
        let dirty_region    = match dirty_region {
            None => null::<RGNDATA>(),
            Some(dr) => {
                if dr.rdh.dwSize as usize   != std::mem::size_of::<RGNDATAHEADER>() { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.iType             != RDH_RECTANGLES                       { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nCount as usize   > dr.buffer.len()                       { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nRgnSize as usize > std::mem::size_of_val(dr)             { return Err(fn_param_error!(dirty_region, THINERR::INVALID_STRUCT_FIELD)); }
                let dr : *const RgnData = dr;
                dr.cast()
            },
        };

        fn_check_hr!(unsafe { self.as_winapi().Present(source_rect, dest_rect, hwnd, dirty_region) })
    }

    //TODO:     IDirect3DDevice9::ProcessVertices                   = d3d9::IDirect3DDevice9Ext::process_vertices

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-reset)\]
    /// IDirect3DDevice9::Reset
    ///
    /// Resets the type, size, and format of the swap chain.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   `presentation_parameters.device_window` must be [None] or a valid window
    /// *   `presentation_parameters.*` in general probably needs certain "valid" values
    ///
    /// ### Returns
    /// *   [D3DERR::DEVICELOST]
    /// *   [D3DERR::DEVICEREMOVED]
    /// *   [D3DERR::DRIVERINTERNALERROR]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   Ok(())
    unsafe fn reset(&self, presentation_parameters: &mut PresentParameters) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::reset => IDirect3DDevice9::Reset);
        fn_check_hr!(unsafe { self.as_winapi().Reset(presentation_parameters.as_mut()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setclipplane)\]
    /// IDirect3DDevice9::SetClipPlane
    ///
    /// Sets the coefficients of a user-defined clipping plane for the device.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If out of memory / address space?
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_clip_plane(0, &[0.1, 0.2, 0.3, 0.4]).unwrap();
    ///
    /// let max = device.get_device_caps().unwrap().max_user_clip_planes;
    /// // Maximum number of *simultaniously enabled* clip planes?  Not limit on total set:
    /// for i in 0 .. max   { device.set_clip_plane(i,        &[0.1, 0.2, 0.3, 0.4]).unwrap() }
    /// for i in max .. 256 { device.set_clip_plane(i,        &[0.1, 0.2, 0.3, 0.4]).unwrap() }
    /// for pow in 8 .. 32  { device.set_clip_plane(1 << pow, &[0.1, 0.2, 0.3, 0.4]).unwrap() }
    /// ```
    fn set_clip_plane(&self, index: u32, plane: &[f32; 4]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_clip_plane => IDirect3DDevice9::SetClipPlane);
        fn_check_hr!(unsafe { self.as_winapi().SetClipPlane(index, plane.as_ptr().cast()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setclipstatus)\]
    /// IDirect3DDevice9::SetClipStatus
    ///
    /// Sets the [ClipStatus], indicating what clipping plane(s) to use.
    ///
    /// ### Returns
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let cs = ClipStatus { clip_union: CS::None, clip_intersection: CS::All };
    /// device.set_clip_status(&cs).unwrap();
    /// # for u in (0 .. 32).map(|p| 1<<p).chain([0x55555555, 0xAAAAAAAA, 0xFFFFFFFF]) {
    /// #   for i in (0 .. 32).map(|p| 1<<p).chain([0x55555555, 0xAAAAAAAA, 0xFFFFFFFF]) {
    /// #       device.set_clip_status(&ClipStatus {
    /// #           clip_union:         CS::from_unchecked(u),
    /// #           clip_intersection:  CS::from_unchecked(i),
    /// #       }).unwrap();
    /// #   }
    /// # }
    /// ```
    fn set_clip_status(&self, clip_status: &ClipStatus) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_clip_status => IDirect3DDevice9::SetClipStatus);
        fn_check_hr!(unsafe { self.as_winapi().SetClipStatus(clip_status.as_ref()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcurrenttexturepalette)\]
    /// IDirect3DDevice9::SetCurrentTexturePalette
    ///
    /// Sets the current texture palette.
    ///
    /// ### ⚠️ Safety ⚠️
    /// Undefined behavior may result unless:
    /// *   `palette_number` <= `max`, given a previous call to [set_palette_entries](Self::set_palette_entries)\(max\)
    /// *   `palette_number` <= `0xFFFF` (max palette count per [Texture Palettes (Direct3D 9)](https://learn.microsoft.com/en-us/windows/win32/direct3d9/texture-palettes))
    ///
    /// ### Returns
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// # if false {
    /// // undefined behavior!
    /// unsafe { device.set_current_texture_palette_unchecked(0).unwrap() };
    /// # }
    ///
    /// // make palettes 0 ..= 0xFFFF safe:
    /// device.set_palette_entries(0xFFFF, &[Color::argb(0xFFFFFFFF); 256]).unwrap();
    ///
    /// // now sound?
    /// unsafe { device.set_current_texture_palette_unchecked(0).unwrap() };
    /// #
    /// # for i in (0 .. 16).map(|p| 1<<p).chain([0xFFFF]) { dbg!(i);
    /// #   unsafe { device.set_current_texture_palette_unchecked(i).unwrap() };
    /// # }
    /// ```
    ///
    /// ### See Also
    /// *   [Texture Palettes (Direct3D 9)](https://learn.microsoft.com/en-us/windows/win32/direct3d9/texture-palettes)
    /// *   [get_current_texture_palette](Self::get_current_texture_palette)
    /// *   [set_palette_entries](Self::set_palette_entries)
    /// *   [set_palette_entries_unchecked](Self::set_palette_entries_unchecked)
    unsafe fn set_current_texture_palette_unchecked(&self, palette_number: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_current_texture_palette_unchecked => IDirect3DDevice9::SetCurrentTexturePalette);
        fn_check_hr!(unsafe { self.as_winapi().SetCurrentTexturePalette(palette_number) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorposition)\]
    /// IDirect3DDevice9::SetCursorPosition
    ///
    /// Sets the cursor position (either in virtual desktop coordinates when in windowed mode, with `0,0` generally
    /// being the top left corner of the primary monitor, or in backbuffer coordinates when in fullscreen mode.)
    ///
    /// ### Errors
    /// May silently fail to do anything if... the window is hidden?  Not focused?
    /// Perhaps also with invalid flags or out-of-bounds coordinates?
    /// Is this setting the position of the emulated cursor position, rather than controlling the OS cursor position?
    /// All I'm really certain of, is that there's no HRESULT or crashes.  On my system.  So far.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_cursor_position(0, 0, d3d::Cursor::None);
    /// device.set_cursor_position(-500, 500, d3d::Cursor::ImmediateUpdate);
    /// device.set_cursor_position(i32::MIN, i32::MAX, d3d::Cursor::from_unchecked(!0));
    /// ```
    fn set_cursor_position(&self, x: i32, y: i32, flags: impl Into<Cursor>) {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_cursor_position => IDirect3DDevice9::SetCursorPosition);
        unsafe { self.as_winapi().SetCursorPosition(x, y, flags.into().into()) };
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorproperties)\]
    /// IDirect3DDevice9::SetCursorProperties
    ///
    /// Set cursor properties.
    ///
    /// ### Arguments
    /// *   `x`             - Horizontal center of the cursor (in pixels)
    /// *   `y`             - Vertical center of the cursor (in pixels)
    /// *   `cursor_bitmap` - A [d3d::Format::A8R8G8B8] surface describing the cursor.
    ///     *   Must be smaller than the display mode.
    ///     *   Alpha "must" be binary (0x00 or 0xFF).
    ///     *   May be resized down to 32x32 if larger than 32x32.
    ///     *   Should also be 32x32 if you desire an OS cursor in windowed mode.
    ///
    /// ### Errors
    /// *   [D3DERR::INVALIDCALL]   - If fed a cursor that's too big
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let pixels = vec![d3d::Color::argb(0xFF112233); 32 * 32];
    /// let data = bytemuck::cast_slice(&pixels);
    /// let mips = [TextureMipRef { data, stride: 4 * 32 }];
    /// let texture = device.create_texture_from(32, 32, &mips, Usage::None, FixedTextureFormat::A8R8G8B8, Pool::SystemMem, ()).unwrap();
    /// let surface = texture.get_surface_level(0).unwrap();
    ///
    /// device.set_cursor_properties(16, 16, &surface).unwrap();
    /// #
    /// # // 128x128, non-binary alpha cursors don't seem to break anything:
    /// # let pixels = vec![d3d::Color::argb(0x80112233); 128 * 128];
    /// # let data = bytemuck::cast_slice(&pixels);
    /// # let mips = [TextureMipRef { data, stride: 4 * 128 }];
    /// # let texture = device.create_texture_from(128, 128, &mips, Usage::None, FixedTextureFormat::A8R8G8B8, Pool::SystemMem, ()).unwrap();
    /// # let surface = texture.get_surface_level(0).unwrap();
    /// # device.set_cursor_properties(0, 0, &surface).expect("128x128 nonbinary alpha cursor");
    /// #
    /// # // Watch the world burn, with a 4k cursor:
    /// # let pixels = vec![d3d::Color::argb(0xFF112233); 4096 * 4096];
    /// # let data = bytemuck::cast_slice(&pixels);
    /// # let mips = [TextureMipRef { data, stride: 4 * 4096 }];
    /// # let texture = device.create_texture_from(4096, 4096, &mips, Usage::None, FixedTextureFormat::A8R8G8B8, Pool::SystemMem, ()).unwrap();
    /// # let surface = texture.get_surface_level(0).unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.set_cursor_properties(0, 0, &surface), "4k x 4k cursor");
    /// ```
    fn set_cursor_properties(&self, x: u32, y: u32, cursor_bitmap: &Surface) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_cursor_properties => IDirect3DDevice9::SetCursorProperties);
        fn_check_hr!(unsafe { self.as_winapi().SetCursorProperties(x, y, cursor_bitmap.as_mut_ptr()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setdepthstencilsurface)\]
    /// IDirect3DDevice9::SetDepthStencilSurface
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]         if `depth_stencil_surface == Some(surface)` and `surface.usage() != Usage::DepthStencil`
    /// *   `Ok(())`                      if the depth stencil was successfully (un)bound
    fn set_depth_stencil_surface(&self, depth_stencil_surface: Option<&Surface>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_depth_stencil_surface => IDirect3DDevice9::SetDepthStencilSurface);
        let ds = depth_stencil_surface.map_or(null_mut(), |ds| ds.as_raw());
        fn_check_hr!(unsafe { self.as_winapi().SetDepthStencilSurface(ds) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setdialogboxmode)\]
    /// IDirect3DDevice9::SetDialogBoxMode
    ///
    /// Enable the use of GDI dialog boxes in full-screen mode.
    ///
    /// Has no effect on windowed mode applications.
    ///
    /// ### Errors
    /// *   [D3DERR::INVALIDCALL]   Unless using a GDI compatible back buffer format:<br>
    ///     [d3d::Format]::{[X1R5G5B5](d3d::Format::X1R5G5B5), [R5G6B5](d3d::Format::R5G6B5), [X8R8G8B8](d3d::Format::X8R8G8B8)}
    /// *   [D3DERR::INVALIDCALL]   Unless [d3d::SwapEffect::Discard] was specified.
    /// *   [D3DERR::INVALIDCALL]   Unless [d3d::PresentFlag::LockableBackBuffer] was specified.
    /// *   [D3DERR::INVALIDCALL]   If multisampling is enabled.
    /// *   [D3DERR::INVALIDCALL]   If [d3d::Create::AdapterGroupDevice] was specified.
    /// *   [D3DERR::INVALIDCALL]   If between [begin_scene](Self::begin_scene)\(\) and [end_scene](Self::end_scene)\(\).
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_dialog_box_mode(true).unwrap();
    /// #
    /// # device.set_dialog_box_mode(false).unwrap();
    /// # device.begin_scene().unwrap();
    /// # assert_eq!(D3DERR::INVALIDCALL, device.set_dialog_box_mode(false));
    /// # device.end_scene().unwrap();
    /// ```
    fn set_dialog_box_mode(&self, mode: bool) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_dialog_box_mode => IDirect3DDevice9::SetDialogBoxMode);
        fn_check_hr!(unsafe { self.as_winapi().SetDialogBoxMode(mode as _) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setfvf)\]
    /// IDirect3DDevice9::SetFVF
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       "If the method fails (impossible via thindx?)
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.set_fvf(FVF::None).unwrap();
    /// device.set_fvf(FVF::XYZ).unwrap();
    /// ```
    fn set_fvf(&self, fvf: impl Into<FVF>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_fvf => IDirect3DDevice9::SetFVF);
        fn_check_hr!(unsafe { self.as_winapi().SetFVF(fvf.into().into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setgammaramp)\]
    /// IDirect3DDevice9::SetGammaRamp
    ///
    /// Sets the gamma correction ramp for the implicit swap chain. This method will affect the entire screen (not just the active window if you are running in windowed mode).
    ///
    /// If the device does not support gamma ramps in the swap chain's current presentation mode (full-screen or windowed), no error return is given.
    /// Applications can check the D3DCAPS2_FULLSCREENGAMMA and D3DCAPS2_CANCALIBRATEGAMMA capability bits in the Caps2 member of the [d3d9::Caps] structure to determine the capabilities of the device and whether a calibrator is installed.
    ///
    /// For windowed gamma correction presentation, use [SwapChain::present] if the hardware supports the feature.
    /// In DirectX 8, SetGammaRamp will set the gamma ramp only on a full-screen mode application.
    /// For more information about gamma correction, see [Gamma (Direct3D 9)].
    ///
    /// ### Example
    /// ```rust,no_run
    /// # use dev::d3d9::*; let device = device_test();
    /// let ramp = device.get_gamma_ramp(0);
    /// // ...modify ramp?..
    /// device.set_gamma_ramp(0, SGR::NoCalibration, &ramp);
    /// ```
    ///
    /// [Gamma (Direct3D 9)]:           https://learn.microsoft.com/en-us/windows/desktop/direct3d9/gamma
    fn set_gamma_ramp(&self, swap_chain: u32, flags: impl Into<SGR>, ramp: &GammaRamp) {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_gamma_ramp => IDirect3DDevice9::SetGammaRamp);
        let _nohr : () = unsafe { self.as_winapi().SetGammaRamp(swap_chain, flags.into().into(), ramp.as_ref()) };
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setindices)\]
    /// IDirect3DDevice9::SetIndices
    ///
    /// Sets index data.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       (perhaps only on an invalid [IndexBuffer] that thindx's API prevents?)
    /// *   [THINERR::DEVICE_MISMATCH]   If the [IndexBuffer] was created with a different [Device].
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let [device, device2] = device_pure2();
    /// let tri = device.create_index_buffer(3*2, Usage::None, Format::Index16, Pool::Default, ()).unwrap();
    /// // ...initialize tri...
    ///
    /// device.set_indices(&tri).unwrap();          // bind the index buffer
    /// device.set_indices(Some(&tri)).unwrap();    // bind the index buffer
    /// device.set_indices(None).unwrap();          // unbind the index buffer
    ///
    /// assert_eq!(device2.set_indices(&tri), THINERR::DEVICE_MISMATCH);
    /// ```
    fn set_indices<'ib>(&self, index_data: impl Into<Option<&'ib IndexBuffer>>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_indices => IDirect3DDevice9::SetIndices);
        let ptr = match index_data.into() {
            None => null_mut(),
            Some(ib) => { ib.check_compatible_with(self).map_err(|e| fn_param_error!(index_data, e))?; ib.as_raw() }
            // Mixing index buffers between devices crashes on my computer, compatability check 100% necessary!
        };
        fn_check_hr!(unsafe { self.as_winapi().SetIndices(ptr) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setlight)\]
    /// IDirect3DDevice9::SetLight
    ///
    /// Assigns a set of lighting properties for this device.
    ///
    /// For soundness, this limits `index` to [u16], instead of accepting [u32] like the underlying API does.
    ///
    /// ### Returns
    /// *   <span style="opacity: 25%">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// device.set_light(0,  light).unwrap(); // Ok
    /// device.set_light(1,  light).unwrap(); // Ok
    /// device.set_light(!0, light).unwrap(); // Ok (16-bit)
    /// ```
    fn set_light(&self, index: u16, light: impl Into<Light>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_light => IDirect3DDevice9::SetLight);
        // Safe: `index` max == `u16::MAX` == `0xFFFF`, well bellow when GArrayT starts buffer overflowing (`0x1000_0000 - 8`)
        unsafe { self.set_light_32_unchecked(index.into(), light.into()) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setlight)\]
    /// IDirect3DDevice9::SetLight
    ///
    /// Assigns a set of lighting properties for this device.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   This will buffer overflow and crash (or worse!) if `index` >= `0x1000_0000 - 8` on some systems!
    /// *   Other, smaller sizes may also crash on other system I haven't tested.
    /// *   Prefer [set_light] (limits the index to [u16], and is thus 100% infalliable.)
    ///
    /// Crash call stacks may look like:
    ///
    /// ```text
    /// d3d9.dll!CHandle::CHandle(void)
    /// d3d9.dll!`eh vector constructor iterator'(void *,unsigned __int64,int,void (*)(void *),void (*)(void *))
    /// d3d9.dll!GArrayT<struct CHandle>::AllocArray(unsigned long)
    /// d3d9.dll!GArrayT<CHandle>::Grow()
    /// d3d9.dll!CD3DHal::SetLightI()
    /// d3d9.dll!CD3DBase::SetLight()
    /// ```
    ///
    /// It appears that some array of 16-byte structures (`CHandle`s?) + header size overflows a 32-bit size.
    /// This successfully allocates, and then is copied into, resulting in a buffer overflow.
    /// Other, smaller sizes may also crash on other system I haven't tested.
    /// Prefer the 16-bit API variations (above) which are 100% sound.
    ///
    /// The `get_*` APIs appear sound, and will not allocate.
    ///
    /// [set_light]:                Self::set_light
    ///
    /// ### Returns
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// unsafe {
    ///     device.set_light_32_unchecked(0, light).unwrap(); // Ok
    ///     device.set_light_32_unchecked(1, light).unwrap(); // Ok
    ///     // device.set_light_32_unchecked(!0, light).unwrap();
    ///     // XXX: Buffer overflow, crash, or worse!
    /// }
    /// ```
    unsafe fn set_light_32_unchecked(&self, index: u32, light: impl Into<Light>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_light_32_unchecked => IDirect3DDevice9::SetLight);
        let light = light.into();
        fn_check_hr!(unsafe { self.as_winapi().SetLight(index, &*light) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setmaterial)\]
    /// IDirect3DDevice9::SetMaterial
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let material = Material {
    ///     diffuse: ColorValue::default(),
    ///     .. Material::default()
    /// };
    /// device.set_material(material).unwrap();
    /// ```
    fn set_material(&self, material: impl Into<Material>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_material => IDirect3DDevice9::SetMaterial);
        fn_check_hr!(unsafe { self.as_winapi().SetMaterial(&*material.into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getnpatchmode)\]
    /// IDirect3DDevice9::SetNPatchMode
    ///
    /// Specifies the number of subdivision segments.
    /// If the number of segments is less than 1.0, N-patches are disabled.
    /// The default value is 0.0.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.set_npatch_mode(0.0).unwrap();
    /// device.set_npatch_mode(1.0).unwrap();
    /// device.set_npatch_mode(9001.0).unwrap();
    /// ```
    fn set_npatch_mode(&self, mode: f32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_npatch_mode => IDirect3DDevice9::SetNPatchMode);
        fn_check_hr!(unsafe { self.as_winapi().SetNPatchMode(mode) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpaletteentries)\]
    /// IDirect3DDevice9::SetPaletteEntries
    ///
    /// Sets palette entries.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If [d3d::PTextureCaps::AlphaPalette] is not set and any entries has an alpha other than 1.0.
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let caps = device.get_device_caps().unwrap();
    ///
    /// let pal = [Color::argb(0xFF112233); 256];
    /// device.set_palette_entries( 0, &pal).unwrap();
    /// device.set_palette_entries(!0, &pal).unwrap();
    ///
    /// // May or may not succeed depending on system
    /// let pal2 = [Color::argb(0x00112233); 256]; // alpha != 1.0
    /// let r = device.set_palette_entries(0, &pal2);
    /// if caps.texture_caps.into_inner() & PTextureCaps::AlphaPalette.into_inner() == 0 { // e.g. Sacrilige
    ///     assert_eq!(D3DERR::INVALIDCALL, r);
    /// } else { // e.g. Github Actions's Windows 2019 Server
    ///     r.unwrap();
    /// }
    /// ```
    ///
    /// ### See Also
    /// *   [Texture Palettes (Direct3D 9)](https://learn.microsoft.com/en-us/windows/win32/direct3d9/texture-palettes)
    fn set_palette_entries(&self, palette_number: u16, entries: &[Color; 256]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_palette_entries => IDirect3DDevice9::SetPaletteEntries);
        // Safety: ✔️ u16::MAX is documented to be valid, tests OK
        unsafe { self.set_palette_entries_unchecked(palette_number.into(), entries) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpaletteentries)\]
    /// IDirect3DDevice9::SetPaletteEntries
    ///
    /// Sets palette entries.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   Prefer [set_palette_entries](Self::set_palette_entries), which should be sound!
    /// *   `palette_number` should probably be <= `0x0000FFFF` as "[There is a maximum of 65,536 (0x0000FFFF) palettes.](https://learn.microsoft.com/en-us/windows/win32/direct3d9/texture-palettes)"
    /// *   `palette_number` > `0x0000FFFF` may work with some runtimes... or may **hang** (my experience trying e.g. `!0u32`), overflow an alloc, crash, etc.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If [d3d::PTextureCaps::AlphaPalette] is not set and any entries has an alpha other than 1.0.
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let caps = device.get_device_caps().unwrap();
    ///
    /// unsafe {
    ///     // Should succeed:
    ///     let pal = [Color::argb(0xFF112233); 256];
    ///     device.set_palette_entries_unchecked(0, &pal).unwrap();
    ///     device.set_palette_entries_unchecked(0xFFFF, &pal).unwrap();
    ///
    ///     // May or may not succeed depending on system
    ///     let pal2 = [Color::argb(0x00112233); 256]; // alpha != 1.0
    ///     let r = device.set_palette_entries_unchecked(0, &pal2);
    ///     if caps.texture_caps.into_inner() & PTextureCaps::AlphaPalette.into_inner() == 0 { // e.g. Sacrilige
    ///         assert_eq!(D3DERR::INVALIDCALL, r);
    ///     } else { // e.g. Github Actions's Windows 2019 Server
    ///         r.unwrap();
    ///     }
    ///
    /// #   if false {
    ///     // Undefined behavior? (palette_number > 0xFFFF)
    ///     device.set_palette_entries_unchecked(0x10000, &pal).unwrap();
    /// #   }
    /// }
    /// ```
    ///
    /// ### See Also
    /// *   [Texture Palettes (Direct3D 9)](https://learn.microsoft.com/en-us/windows/win32/direct3d9/texture-palettes)
    /// *   [set_current_texture_palette_unchecked](Self::set_current_texture_palette_unchecked)
    /// *   [set_palette_entries](Self::set_palette_entries)
    unsafe fn set_palette_entries_unchecked(&self, palette_number: u32, entries: &[Color; 256]) -> Result<(), Error> {
        // D3D9 uses PALETTEENTRYs but misuses the flags field.  D3DCOLORs are much better fits.
        fn_context!(d3d9::IDirect3DDevice9Ext::set_palette_entries_unchecked => IDirect3DDevice9::SetPaletteEntries);
        fn_check_hr!(unsafe { self.as_winapi().SetPaletteEntries(palette_number, entries.as_ptr().cast()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshader)\]
    /// IDirect3DDevice9::SetPixelShader
    ///
    /// Sets the pixel shader to render with.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `pixel_shader` was created by another device?
    /// *   Ok(())
    fn set_pixel_shader<'sh>(&self, pixel_shader: impl Into<Option<&'sh PixelShader>>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_pixel_shader => IDirect3DDevice9::SetPixelShader);
        let pixel_shader = pixel_shader.into();
        let ps = pixel_shader.map_or(null_mut(), |ps| ps.as_raw());
        fn_check_hr!(unsafe { self.as_winapi().SetPixelShader(ps) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantb)\]
    /// IDirect3DDevice9::SetPixelShaderConstantB
    ///
    /// Sets boolean shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // ps_3_0 (d3d9 max) only supports 16 boolean registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-ps-registers-ps-3-0
    /// let constants = [abibool::bool32::TRUE; 16];
    /// let too_many  = [abibool::bool32::TRUE; 17];
    ///
    /// device.set_pixel_shader_constant_b(0, &constants).unwrap();         // b0 .. b16
    /// device.set_pixel_shader_constant_b(8, &constants[8..]).unwrap();    // b8 .. b16
    ///
    /// // Out of bounds:
    /// let r1 = device.set_pixel_shader_constant_b(!0, &constants);        // b-1.. b15
    /// let r2 = device.set_pixel_shader_constant_b(0, &too_many);          // b0 .. b17
    /// let r3 = device.set_pixel_shader_constant_b(1, &constants);         // b1 .. b17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn set_pixel_shader_constant_b(&self, start_register: u32, constant_data: &[bool32]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_pixel_shader_constant_b => IDirect3DDevice9::SetPixelShaderConstantB);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetPixelShaderConstantB(start_register, constant_data.as_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantf)\]
    /// IDirect3DDevice9::SetPixelShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // ps_3_0 (d3d9 max) supports 224 float4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-ps-registers-ps-3-0
    /// let constants   = vec![[0.0, 0.0, 0.0, 0.0]; 224];
    /// let too_many    = vec![[0.0, 0.0, 0.0, 0.0]; 225];
    ///
    /// device.set_pixel_shader_constant_f(0, &constants).unwrap();         // c0 .. c224
    /// device.set_pixel_shader_constant_f(8, &constants[8..]).unwrap();    // c8 .. c224
    ///
    /// // Out of bounds:
    /// let r1 = device.set_pixel_shader_constant_f(!0, &constants);        // c-1.. c223
    /// let r2 = device.set_pixel_shader_constant_f(0, &too_many);          // c0 .. c225
    /// let r3 = device.set_pixel_shader_constant_f(1, &constants);         // c1 .. c225
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn set_pixel_shader_constant_f(&self, start_register: u32, constant_data: &[[f32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_pixel_shader_constant_f => IDirect3DDevice9::SetPixelShaderConstantF);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetPixelShaderConstantF(start_register, constant_data.as_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstanti)\]
    /// IDirect3DDevice9::SetPixelShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // ps_3_0 (d3d9 max) only supports 16 int4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-ps-registers-ps-3-0
    /// let constants = [[0, 0, 0, 0]; 16];
    /// let too_many  = [[0, 0, 0, 0]; 17];
    ///
    /// device.set_pixel_shader_constant_i(0, &constants).unwrap();         // i0 .. i16
    /// device.set_pixel_shader_constant_i(8, &constants[8..]).unwrap();    // i8 .. i16
    ///
    /// // Out of bounds:
    /// let r1 = device.set_pixel_shader_constant_i(!0, &constants);        // i-1.. i15
    /// let r2 = device.set_pixel_shader_constant_i(0, &too_many);          // i0 .. i17
    /// let r3 = device.set_pixel_shader_constant_i(1, &constants);         // i1 .. i17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn set_pixel_shader_constant_i(&self, start_register: u32, constant_data: &[[i32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_pixel_shader_constant_i => IDirect3DDevice9::SetPixelShaderConstantI);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetPixelShaderConstantI(start_register, constant_data.as_ptr().cast(), n) })
    }

    //TODO: set_render_state (typed)

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setrenderstate)\]
    /// IDirect3DDevice9::SetRenderState
    ///
    /// Sets a single device render-state parameter
    ///
    /// ### Returns
    /// *   Ok(())              no matter what invalid parameters are used?
    ///
    /// ### Examples
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_render_state_untyped(d3d::RS::Lighting,          true as u32             ).unwrap();
    /// device.set_render_state_untyped(d3d::RS::AlphaBlendEnable,  true as u32             ).unwrap();
    /// device.set_render_state_untyped(d3d::RS::DestBlend,         d3d::Blend::InvSrcAlpha ).unwrap();
    /// device.set_render_state_untyped(d3d::RS::Ambient,           0xFFFFFFFFu32           ).unwrap();
    /// # for rs in [1000, 10000, 100000, 1000000, !0u32].iter().copied() {
    /// #     for value in [0, 1, 100, 10000, 1000000, !0u32].iter().copied() {
    /// #         // bad inputs silently ignored?
    /// #         device.set_render_state_untyped(d3d::RS::from_unchecked(rs), value).unwrap();
    /// #     }
    /// # }
    /// ```
    fn set_render_state_untyped(&self, state: impl Into<d3d9::RenderStateType>, value: impl Into<u32>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_render_state_untyped => IDirect3DDevice9::SetRenderState);
        fn_check_hr!(unsafe { self.as_winapi().SetRenderState(state.into().into(), value.into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setrendertarget)\]
    /// IDirect3DDevice9::SetRenderTarget
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]         if `render_target_index == 0` and `render_target == None`
    /// *   [D3DERR::INVALIDCALL]         if `render_target == Some(surface)` and `surface.usage() != Usage::RenderTarget`
    /// *   `Ok(())`                      if the render target was successfully bound
    fn set_render_target(&self, render_target_index: u32, render_target: Option<&Surface>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_render_target => IDirect3DDevice9::SetRenderTarget);
        let rt = render_target.map_or(null_mut(), |rt| rt.as_raw());
        fn_check_hr!(unsafe { self.as_winapi().SetRenderTarget(render_target_index, rt) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsamplerstate)\]
    /// IDirect3DDevice9::SetSamplerState
    ///
    /// Prefer [set_sampler_state](Self::set_sampler_state)
    ///
    /// ### ⚠️ Safety ⚠️
    /// Known undefined behavior:
    /// *   This function may crash with out-of-bounds sampler state types! (possibly only if fed nonzero values?)
    ///
    /// Possible undefined behavior on older Runtimes/Drivers?
    /// *   This function may crash with out-of-bounds samplers?
    /// *   This function may crash with out-of-bounds sampler state type values?
    ///
    /// ### Returns
    /// *   Ok(())              no matter what invalid parameters are used?
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// unsafe {
    ///     device.set_sampler_state_unchecked(0, d3d::Samp::MinFilter, d3d::TexF::Linear).unwrap();
    ///     device.set_sampler_state_unchecked(0, d3d::Samp::MagFilter, d3d::TexF::Linear).unwrap();
    ///     device.set_sampler_state_unchecked(0, d3d::Samp::MipFilter, d3d::TexF::Linear).unwrap();
    ///
    ///     if false { // undefined behavior:
    ///         device.set_sampler_state_unchecked(0, d3d::Samp::from_unchecked(1000000), 1u32).unwrap();
    /// #       // fuzz testing:
    /// #       for i in [0, 1, 2, 4, 8, 16, 32, 64, 128, 1000, 100000, 1000000, !0u32].iter().copied() { dbg!(i);
    /// #           for ss in [1000, 10000, 100000, 1000000, !0u32].iter().copied() { dbg!(ss);
    /// #               for value in [0, 1, 100, 10000, 1000000, !0u32].iter().copied() { dbg!(value);
    /// #                   // bad inputs silently ignored?
    /// #                   device.set_sampler_state_unchecked(i, d3d::Samp::from_unchecked(ss), value).unwrap();
    /// #               }
    /// #           }
    /// #       }
    ///     }
    /// }
    /// ```
    unsafe fn set_sampler_state_unchecked(&self, sampler: u32, ty: SamplerStateType, value: impl Into<u32>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_sampler_state_unchecked => IDirect3DDevice9::SetSamplerState);
        fn_check_hr!(unsafe { self.as_winapi().SetSamplerState(sampler, ty.into(), value.into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsamplerstate)\]
    /// IDirect3DDevice9::SetSamplerState
    ///
    /// ### Returns
    /// *   Ok(())              no matter what invalid parameters are used?
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_sampler_state(0, d3d::SampV::MinFilter(d3d::TexF::Linear)).unwrap();
    /// device.set_sampler_state(0, d3d::SampV::MagFilter(d3d::TexF::Linear)).unwrap();
    /// device.set_sampler_state(0, d3d::SampV::MipFilter(d3d::TexF::Linear)).unwrap();
    /// # // fuzz testing:
    /// # for s in [0, 1, 2, 4, 8, 16, 32, 64, 128, 1000, 100000, 1000000, !0u32].iter().copied() { dbg!(s);
    /// #   for taddr in (0..16).into_iter().chain([60, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, !0u32].iter().copied()).map(|i| d3d::TAddress::from_unchecked(i)) { dbg!(taddr);
    /// #       device.set_sampler_state(s, d3d::SampV::AddressU(taddr)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::AddressV(taddr)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::AddressW(taddr)).unwrap();
    /// #   }
    /// #   for color in (0u32..=0xFF).into_iter().map(|i| d3d::Color::argb(i * 0x01010101)) { dbg!(color);
    /// #       device.set_sampler_state(s, d3d::SampV::BorderColor(color)).unwrap();
    /// #   }
    /// #   for texf in (0..16).into_iter().chain([60, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, !0u32].iter().copied()).map(|i| d3d::TexF::from_unchecked(i)) { dbg!(texf);
    /// #       device.set_sampler_state(s, d3d::SampV::MinFilter(texf)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::MagFilter(texf)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::MipFilter(texf)).unwrap();
    /// #   }
    /// #   for i in (0..256).into_iter().chain([60, 100, 1000, 10000, 100000, 1000000, 10000000, 100000000, !0u32].iter().copied()) { dbg!(i);
    /// #       device.set_sampler_state(s, d3d::SampV::MipMapLODBias(i)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::MaxMipLevel(i)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::MaxAnisotropy(i)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::SRGBTexture(i&1 != 0)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::ElementIndex(i)).unwrap();
    /// #       device.set_sampler_state(s, d3d::SampV::DMapOffset(i)).unwrap();
    /// #   }
    /// # }
    /// ```
    fn set_sampler_state(&self, sampler: u32, ssv: impl Into<SamplerStateValue>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_sampler_state => IDirect3DDevice9::SetSamplerState);
        let (ty, value) = ssv.into().ty_value();
        unsafe { self.set_sampler_state_unchecked(sampler, ty, value) }
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setscissorrect)\]
    /// IDirect3DDevice9::SetScissorRect
    ///
    /// Sets the scissor rectangle.
    ///
    /// ### Errors
    /// Speculation: [D3DERR::INVALIDCALL]   If scissor rects not supported?
    ///
    /// Observed: None
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_scissor_rect([0..0, 0..0]).unwrap();
    /// device.set_scissor_rect([0..100, 0..100]).unwrap();
    /// device.set_scissor_rect([100..0, 100..0]).unwrap();
    /// device.set_scissor_rect([i32::MIN .. i32::MAX, i32::MIN .. i32::MAX]).unwrap();
    /// ```
    fn set_scissor_rect(&self, rect: impl Into<d3d::Rect>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_scissor_rect => IDirect3DDevice9::SetScissorRect);
        fn_check_hr!(unsafe { self.as_winapi().SetScissorRect(rect.into().as_ref()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsoftwarevertexprocessing)\]
    /// IDirect3DDevice9::SetSoftwareVertexProcessing
    ///
    /// Use this method to switch between software and hardware vertex processing on devices created with [d3d::Create::MixedVertexProcessing].
    ///
    /// ### Errors
    /// *   [D3DERR::INVALIDCALL]   If `software` was `false`, unless the device was created with [d3d::Create::MixedVertexProcessing] or [SoftwareVertexProcessing](d3d::Create::SoftwareVertexProcessing)
    /// *   [D3DERR::INVALIDCALL]   If `software` was `true`, unless the device was created with [d3d::Create::MixedVertexProcessing] or [HardwareVertexProcessing](d3d::Create::HardwareVertexProcessing)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*;
    /// # for create in [
    /// #   Create::FpuPreserve | Create::NoWindowChanges | Create::SoftwareVertexProcessing,
    /// #   Create::FpuPreserve | Create::NoWindowChanges | Create::MixedVertexProcessing,
    /// #   Create::FpuPreserve | Create::NoWindowChanges | Create::HardwareVertexProcessing,
    /// # ].iter().copied() {
    /// # let device = device_test_pp(false, |_,c| *c = create).unwrap();
    /// let params = device.get_creation_parameters().unwrap();
    /// let device_is_software = params.behavior_flags.into() & Create::SoftwareVertexProcessing.into() == Create::SoftwareVertexProcessing.into();
    /// let device_is_mixed    = params.behavior_flags.into() & Create::MixedVertexProcessing   .into() == Create::MixedVertexProcessing   .into();
    /// let device_is_hardware = params.behavior_flags.into() & Create::HardwareVertexProcessing.into() == Create::HardwareVertexProcessing.into();
    ///
    /// let r = device.set_software_vertex_processing(true);
    /// assert_eq!(r.is_ok(), device_is_software || device_is_mixed);
    /// r.map_err(|e| assert_eq!(e, D3DERR::INVALIDCALL));
    ///
    /// let r = device.set_software_vertex_processing(false);
    /// assert_eq!(r.is_ok(), device_is_hardware || device_is_mixed);
    /// r.map_err(|e| assert_eq!(e, D3DERR::INVALIDCALL));
    /// # }
    /// ```
    fn set_software_vertex_processing(&self, software: bool) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_software_vertex_processing => IDirect3DDevice9::SetSoftwareVertexProcessing);
        fn_check_hr!(unsafe { self.as_winapi().SetSoftwareVertexProcessing(software as _) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsource)\]
    /// IDirect3DDevice9::SetStreamSource
    ///
    /// Binds a vertex buffer to a device data stream. For more information, see [Setting the Stream Source (Direct3D 9)].
    ///
    /// [Setting the Stream Source (Direct3D 9)]:       https://learn.microsoft.com/en-us/windows/desktop/direct3d9/setting-the-stream-source
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       if the [VertexBuffer] belongs to another device?
    /// *   [THINERR::DEVICE_MISMATCH]   If the [IndexBuffer] was created with a different [Device].
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let [device, device2] = device_pure2();
    /// let tri = device.create_vertex_buffer(3*4*3, Usage::None, FVF::XYZ, Pool::Default, ()).unwrap();
    /// // ...initialize tri...
    /// device.set_stream_source(0, &tri,       0, 4*3).unwrap(); // bind the vertex buffer
    /// device.set_stream_source(0, Some(&tri), 0, 4*3).unwrap(); // bind the vertex buffer
    /// device.set_stream_source(0, None,       0, 0  ).unwrap(); // unbind the vertex buffer
    ///
    /// assert_eq!(device2.set_stream_source(0, &tri, 0, 4*3), THINERR::DEVICE_MISMATCH);
    /// ```
    fn set_stream_source<'b>(&self, stream_number: u32, stream_data: impl Into<Option<&'b VertexBuffer>>, offset_in_bytes: u32, stride: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_stream_source => IDirect3DDevice9::SetStreamSource);
        let stream_data = match stream_data.into() {
            None => null_mut(),
            Some(sd) => { sd.check_compatible_with(self).map_err(|e| fn_param_error!(stream_data, e))?; sd.as_raw() },
            // XXX: Might be able to skip check_compatible_with with software vertex buffers?  Those might be safe?
            // They don't seem to crash for me, but I'm erring on the side of caution for now.
        };
        fn_check_hr!(unsafe { self.as_winapi().SetStreamSource(stream_number, stream_data, offset_in_bytes, stride) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsourcefreq)\]
    /// IDirect3DDevice9::SetStreamSourceFreq
    ///
    /// Sets the stream source frequency divider value. This may be used to draw several instances of geometry.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Setup instanced rendering, 100 instances, with:
    /// // shared geometry in stream 0, repeated 100 times:
    /// device.set_stream_source_freq(0, StreamSource::indexed_data(100)).unwrap();
    /// // per-instance data in stream 1:
    /// device.set_stream_source_freq(1, StreamSource::instance_data(1)).unwrap();
    ///
    /// // Restore non-instanced rendering
    /// device.set_stream_source_freq(0, StreamSource::regular()).unwrap();
    /// device.set_stream_source_freq(1, StreamSource::regular()).unwrap();
    /// ```
    fn set_stream_source_freq(&self, stream_number: u32, setting: impl Into<StreamSource>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_stream_source_freq => IDirect3DDevice9::SetStreamSourceFreq);
        let setting = setting.into().into();
        fn_check_hr!(unsafe { self.as_winapi().SetStreamSourceFreq(stream_number, setting) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-settexture)\]
    /// IDirect3DDevice9::SetTexture
    ///
    /// Assigns a texture to a stage for a device.
    ///
    /// ### ⚠️ Safety ⚠️
    /// *   This function will crash (or worse!) if `stage` is too large (> `MaxSimultaneousTextures`?)
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    ///
    /// unsafe{device.set_texture(0,      &texture  )}.unwrap();
    /// unsafe{device.set_texture(1, Some(&*texture))}.unwrap();
    /// unsafe{device.set_texture(2, None           )}.unwrap();
    /// assert_eq!(unsafe{device.get_texture(0)}.unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// assert_eq!(unsafe{device.get_texture(1)}.unwrap().map(|t| t.as_raw()), Some((*texture).as_raw()));
    /// assert!(unsafe{device.get_texture(2)}.unwrap().is_none());
    ///
    /// // unsafe{device.set_texture(1000,  &texture)}.unwrap(); // XXX: works on my machine, but might could be doing bad shit like corrupting memory?
    /// // unsafe{device.set_texture(10000, &texture)}.unwrap(); // XXX: crashes on my machine!
    /// ```
    unsafe fn set_texture<'t>(&self, stage: u32, texture: impl Into<Option<&'t BaseTexture>>) -> Result<(), Error> {
        // TODO: make a more convenient trait for texture binding
        fn_context!(d3d9::IDirect3DDevice9Ext::set_texture => IDirect3DDevice9::SetTexture);
        let texture = texture.into();
        let texture = texture.map_or(null_mut(), |t| t.as_raw());
        fn_check_hr!(unsafe { self.as_winapi().SetTexture(stage, texture) })
    }

    // TODO: set_texture_stage_state (typed)

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-settexturestagestate)\]
    /// IDirect3DDevice9::SetTextureStageState
    ///
    /// Sets the state value for the currently assigned texture.
    ///
    /// ### Errors
    /// *   ~~[D3DERR::INVALIDCALL]~~ None observed?
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_texture_stage_state_untyped(0, TSS::ColorArg1, 0xFF112233).unwrap();
    /// # device.set_texture_stage_state_untyped(!0, TSS::ColorArg1, 0xFF112233).unwrap();
    /// # device.set_texture_stage_state_untyped(0, TSS::from_unchecked(0), 0xFF112233).unwrap();
    /// # device.set_texture_stage_state_untyped(0, TSS::from_unchecked(!0), 0xFF112233).unwrap();
    /// # for stage in (0 .. 32).map(|p| 1<<p) {
    /// #   for ty in (0 .. 32).map(|p| 1<<p) {
    /// #       for val in [0, 1, 2, 4, 8, 0x11223344, 0xFF112233, 0xFFFFFFFF] {
    /// #           device.set_texture_stage_state_untyped(stage, TSS::from_unchecked(ty), val).unwrap();
    /// #       }
    /// #   }
    /// # }
    /// ```
    fn set_texture_stage_state_untyped(&self, stage: u32, ty: impl Into<TextureStageStateType>, value: u32) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_texture_stage_state_untyped => IDirect3DDevice9::SetTextureStageState);
        fn_check_hr!(unsafe { self.as_winapi().SetTextureStageState(stage, ty.into().into(), value) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-settransform)\]
    /// IDirect3DDevice9::SetTransform
    ///
    /// Sets a single device transform
    ///
    /// ### Returns
    /// *   Ok(())
    ///
    /// ### Examples
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.set_transform(d3d::TS::World, d3d::Matrix::identity()).unwrap();
    /// # device.set_transform(d3d::TS::from_unchecked(0x0FFFF00), d3d::Matrix::identity()).unwrap();
    /// # device.set_transform(d3d::TS::from_unchecked(0x1000000), d3d::Matrix::identity()).unwrap();
    /// # device.set_transform(d3d::TS::from_unchecked(0x10000FF), d3d::Matrix::identity()).unwrap();
    ///
    /// // no errors generated no matter what?
    /// device.set_transform(d3d::TS::from_unchecked(0xFFFFFFF), d3d::Matrix::identity()).unwrap();
    /// ```
    fn set_transform(&self, ts: impl Into<TransformStateType>, matrix: impl Into<Matrix>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_transform => IDirect3DDevice9::SetTransform);
        fn_check_hr!(unsafe { self.as_winapi().SetTransform(ts.into().into(), &matrix.into().into()) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexdeclaration)\]
    /// IDirect3DDevice9::SetVertexDeclaration
    ///
    /// Describes the layout of vertexes for rendering.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `decl` was created by another device?
    /// *   Ok(())
    fn set_vertex_declaration<'d>(&self, decl: impl Into<Option<&'d VertexDeclaration>>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_vertex_declaration => IDirect3DDevice9::SetVertexDeclaration);
        let decl = decl.into();
        let decl = decl.map_or(null_mut(), |d| d.as_raw());
        fn_check_hr!(unsafe { self.as_winapi().SetVertexDeclaration(decl) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshader)\]
    /// IDirect3DDevice9::SetVertexShader
    ///
    /// Sets the vertex shader to render with.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `vertex_shader` was created by another device?
    /// *   Ok(())
    fn set_vertex_shader<'sh>(&self, vertex_shader: impl Into<Option<&'sh VertexShader>>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_vertex_shader => IDirect3DDevice9::SetVertexShader);
        let vertex_shader = vertex_shader.into();
        let ps = vertex_shader.map_or(null_mut(), |ps| ps.as_raw());
        fn_check_hr!(unsafe { self.as_winapi().SetVertexShader(ps) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantb)\]
    /// IDirect3DDevice9::SetVertexShaderConstantB
    ///
    /// Sets boolean shader constants (b#). Unlike floating-point or integer constants, these have a dimension of 1, and are **not** grouped into 4-element vectors.
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // vs_3_0 (d3d9 max) only supports 16 boolean registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-vs-registers-vs-3-0
    /// let constants = [abibool::bool32::TRUE; 16];
    /// let too_many  = [abibool::bool32::TRUE; 16+1];
    ///
    /// device.set_vertex_shader_constant_b(0, &constants).unwrap();        // b0 .. b16
    /// device.set_vertex_shader_constant_b(8, &constants[8..]).unwrap();   // b8 .. b16
    ///
    /// // Out of bounds:
    /// let r1 = device.set_vertex_shader_constant_b(!0, &constants);       // b-1.. b15
    /// let r2 = device.set_vertex_shader_constant_b(0, &too_many);         // b0 .. b17
    /// let r3 = device.set_vertex_shader_constant_b(1, &constants);        // b1 .. b17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn set_vertex_shader_constant_b(&self, start_register: u32, constant_data: &[bool32]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_vertex_shader_constant_b => IDirect3DDevice9::SetVertexShaderConstantB);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetVertexShaderConstantB(start_register, constant_data.as_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantf)\]
    /// IDirect3DDevice9::SetVertexShaderConstantF
    ///
    /// Sets floating-point shader constants (c#).  Each individual c# register is a 4-element floating point vector: \[[f32]; 4\]
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // vs_3_0 (d3d9 max) supports at least 256 float4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-vs-registers-vs-3-0
    /// let cmax = device.get_device_caps().unwrap().max_vertex_shader_const as usize;
    /// let constants = vec![[0.0, 0.0, 0.0, 0.0]; cmax];
    /// let too_many  = vec![[0.0, 0.0, 0.0, 0.0]; cmax+1];
    ///
    /// device.set_vertex_shader_constant_f(0, &constants).unwrap();        // c0 .. cMAX
    /// device.set_vertex_shader_constant_f(8, &constants[8..]).unwrap();   // c8 .. cMAX
    ///
    /// // Out of bounds:
    /// let r1 = device.set_vertex_shader_constant_f(!0, &constants);       // c-1.. cMAX+1
    /// let r2 = device.set_vertex_shader_constant_f(0, &too_many);         // c0 .. cMAX+1
    /// let r3 = device.set_vertex_shader_constant_f(1, &constants);        // c1 .. cMAX-1
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn set_vertex_shader_constant_f(&self, start_register: u32, constant_data: &[[f32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_vertex_shader_constant_f => IDirect3DDevice9::SetVertexShaderConstantF);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetVertexShaderConstantF(start_register, constant_data.as_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstanti)\]
    /// IDirect3DDevice9::SetVertexShaderConstantI
    ///
    /// Sets integer shader constants (i#).  Each individual i# register is a 4-element integer vector: \[[i32]; 4\]
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max integer vector registers
    /// *   Ok(())
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // vs_3_0 (d3d9 max) only supports 16 int4 registers
    /// // ref: https://learn.microsoft.com/en-us/windows/win32/direct3dhlsl/dx9-graphics-reference-asm-vs-registers-vs-3-0
    /// let constants = [[0, 0, 0, 0]; 16];
    /// let too_many  = [[0, 0, 0, 0]; 16+1];
    ///
    /// device.set_vertex_shader_constant_i(0, &constants).unwrap();        // i0 .. i16
    /// device.set_vertex_shader_constant_i(8, &constants[8..]).unwrap();   // i8 .. i16
    ///
    /// // Out of bounds:
    /// let r1 = device.set_vertex_shader_constant_i(!0, &constants);       // i-1.. i15
    /// let r2 = device.set_vertex_shader_constant_i(0, &too_many);         // i0 .. i17
    /// let r3 = device.set_vertex_shader_constant_i(1, &constants);        // i1 .. i17
    /// assert_eq!(D3DERR::INVALIDCALL, r1, "start: negative");
    /// assert_eq!(D3DERR::INVALIDCALL, r2, "count: oob");
    /// assert_eq!(D3DERR::INVALIDCALL, r3, "end: oob");
    /// ```
    fn set_vertex_shader_constant_i(&self, start_register: u32, constant_data: &[[i32; 4]]) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_vertex_shader_constant_i => IDirect3DDevice9::SetVertexShaderConstantI);
        let n = fn_param_try_len32!(constant_data)?;
        fn_check_hr!(unsafe { self.as_winapi().SetVertexShaderConstantI(start_register, constant_data.as_ptr().cast(), n) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setviewport)\]
    /// IDirect3DDevice9::SetViewport
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If the viewport is invalid / describes a region that cannot exist within the render target surface.
    /// *   Ok(`()`)
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.set_viewport(Viewport{ x: 0, y: 0, width: 100, height: 100, min_z: 0.0, max_z: 1.0 }).unwrap();
    /// ```
    fn set_viewport(&self, viewport: impl Into<Viewport>) -> Result<(), Error> {
        fn_context!(d3d9::IDirect3DDevice9Ext::set_viewport => IDirect3DDevice9::SetViewport);
        let viewport = viewport.into();
        fn_check_hr!(unsafe { self.as_winapi().SetViewport(&*viewport) })
    }

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-showcursor)\]
    /// IDirect3DDevice9::ShowCursor
    ///
    /// Displays or hides the cursor.
    ///
    /// ### Returns
    /// *   `true`  - If the cursor was previously visible.
    /// *   `false` - If the cursor was previously hidden.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.show_cursor(false); // hide
    /// let was_visible = device.show_cursor(true); // show
    /// assert_eq!(was_visible, false);
    /// ```
    fn show_cursor(&self, show: bool) -> bool {
        fn_context!(d3d9::IDirect3DDevice9Ext::show_cursor => IDirect3DDevice9::ShowCursor);
        unsafe { self.as_winapi().ShowCursor(show as _) != 0 }
    }

    //TODO:     IDirect3DDevice9::StretchRect                       = d3d9::IDirect3DDevice9Ext::stretch_rect

    /// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-testcooperativelevel)\]
    /// IDirect3DDevice9::TestCooperativeLevel
    ///
    /// Reports the current cooperative-level status of the Direct3D device for a windowed or full-screen application.
    ///
    /// ### Returns
    /// *   Ok([`S::OK`])                           If the device is operational and the calling application can continue.
    /// *   Err([`D3DERR::DEVICELOST`])             If the device is lost, and cannot be restored at the current time (e.g. perhaps a full-screen device has lost focus.)
    /// *   Err([`D3DERR::DEVICENOTRESET`])         If the device is lost, and can either be restored via [`d3d::IDirect3DDevice9Ext::reset`], or via device recreation.
    /// *   Err([`D3DERR::DRIVERINTERNALERROR`])    If things went boom.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// assert!(device.test_cooperative_level().is_ok());
    /// ```
    fn test_cooperative_level(&self) -> Result<HResultSuccess, HResultError> {
        fn_context!(d3d9::IDirect3DDevice9Ext::test_cooperative_level => IDirect3DDevice9::TestCooperativeLevel);
        HResult::from(unsafe { self.as_winapi().TestCooperativeLevel() } as u32).succeeded()
    }

    //TODO:     IDirect3DDevice9::UpdateSurface                     = d3d9::IDirect3DDevice9Ext::update_surface
    //TODO:     IDirect3DDevice9::UpdateTexture                     = d3d9::IDirect3DDevice9Ext::update_texture
    //TODO:     IDirect3DDevice9::ValidateDevice                    = d3d9::IDirect3DDevice9Ext::validate_device
}

impl<T: AsSafe<IDirect3DDevice9>> IDirect3DDevice9Ext for T {}



/// \[[microsoft.com](https://learn.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgndata)\]
/// RGNDATA placeholder
///
/// RGNDATA is a header-prefixed array.  While constructable in Rust, they're slightly awkward at best.
#[repr(C)]
pub struct RgnData {
    pub(crate) rdh:    RGNDATAHEADER,
    pub(crate) buffer: [Rect],
}



#[cfg(test)] mod tests {
    use dev::d3d9::*;

    #[test] fn begin_end_scene() {
        let device = device_test();
        assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

        device.begin_scene().unwrap();
        device.end_scene().unwrap();
        assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

        device.begin_scene().unwrap();
        assert_eq!(D3DERR::INVALIDCALL, device.begin_scene());
        device.end_scene().unwrap();
        assert_eq!(D3DERR::INVALIDCALL, device.end_scene());

        device.begin_scene().unwrap();
        for _ in 0..1000 { assert_eq!(D3DERR::INVALIDCALL, device.begin_scene()); }
        device.end_scene().unwrap();
        for _ in 0..1000 { assert_eq!(D3DERR::INVALIDCALL, device.end_scene()); }
    }

    #[test] fn present() {
        let device = device_test_pp(false, |pp, _| pp.swap_effect = SwapEffect::Copy).unwrap();
        device.present(.., .., (), None).unwrap();

        for rect in [
            (0, 0) .. (1, 1),
            (-100, -100) .. (100, 100),
            (100, 100) .. (-100, -100),
            (-1000, -1000) .. (1000, 1000),
            (1000, 1000) .. (-1000, -1000),
            (-100000, -100000) .. (100000, 100000),
            (0, 0) .. (100000, 100000),
            (0, 0) .. (i32::MAX, i32::MAX),
            (i32::MIN, i32::MIN) .. (i32::MAX, i32::MAX),
            (i32::MAX, i32::MAX) .. (i32::MIN, i32::MIN),
        ].iter().cloned() {
            let rect = Rect::from(rect);
            device.present(rect, rect, (), None).unwrap();
        }
    }
}



//#cpp2rust IDirect3DDevice9                                    = d3d9::Device
//#cpp2rust IDirect3DDevice9                                    = d3d9::IDirect3DDevice9Ext

