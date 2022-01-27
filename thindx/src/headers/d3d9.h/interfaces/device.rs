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

use std::convert::TryInto;
use std::ptr::*;

pub(crate) const MAX_BUFFER_ALLOC : u32 = 0xFFFF_0000;

// TODO: support for Device s in doc comment examples (via dev crate?)
// TODO: fuzz / torture-test Device operations in randomized combinations for odd interactions


/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
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



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nn-d3d9-idirect3ddevice9)\]
/// IDirect3DDevice9 extension methods
///
/// ### Methods
///
/// | thindx                                                                    | docs.microsoft.com            | Description |
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
/// | [get_palette_entries](Self::get_palette_entries)                          | [GetPaletteEntries]           | Retrieves palette entries.
/// | [get_pixel_shader](Self::get_pixel_shader)                                | [GetPixelShader]              | Retrieves the currently set pixel shader.
/// | [get_pixel_shader_constant_b](Self::get_pixel_shader_constant_b)          | [GetPixelShaderConstantB]     | Gets a Boolean shader constant.
/// | [get_pixel_shader_constant_f](Self::get_pixel_shader_constant_f)          | [GetPixelShaderConstantF]     | Gets a floating-point shader constant.
/// | [get_pixel_shader_constant_i](Self::get_pixel_shader_constant_i)          | [GetPixelShaderConstantI]     | Gets an integer shader constant.
/// | [get_raster_status](Self::get_raster_status)                              | [GetRasterStatus]             | Returns information describing the raster of the monitor on which the swap chain is presented.
/// | [get_render_state](Self::get_render_state)                                | [GetRenderState]              | Retrieves a render-state value for a device.
/// | [get_render_target](Self::get_render_target)                              | [GetRenderTarget]             | Retrieves a render-target surface.
/// | [get_render_target_data](Self::get_render_target_data)                    | [GetRenderTargetData]         | Copies the render-target data from device memory to system memory.
/// | [get_sampler_state](Self::get_sampler_state)                              | [GetSamplerState]             | Gets the sampler state value.
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
/// | [set_current_texture_palette](Self::set_current_texture_palette)          | [SetCurrentTexturePalette]    | Sets the current texture palette.
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
/// | [set_pixel_shader_constant_fv](Self::set_pixel_shader_constant_fv)        | [SetPixelShaderConstantF]     | Sets a floating-point shader constant.
/// | [set_pixel_shader_constant_i](Self::set_pixel_shader_constant_i)          | [SetPixelShaderConstantI]     | Sets an integer shader constant.
/// | [set_pixel_shader_constant_iv](Self::set_pixel_shader_constant_iv)        | [SetPixelShaderConstantI]     | Sets an integer shader constant.
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
/// | [set_vertex_shader_constant_fv](Self::set_vertex_shader_constant_fv)      | [SetVertexShaderConstantF]    | Sets a floating-point vertex shader constant.
/// | [set_vertex_shader_constant_i](Self::set_vertex_shader_constant_i)        | [SetVertexShaderConstantI]    | Sets an integer vertex shader constant.
/// | [set_vertex_shader_constant_iv](Self::set_vertex_shader_constant_iv)      | [SetVertexShaderConstantI]    | Sets an integer vertex shader constant.
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
/// [BeginScene]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginscene
/// [BeginStateBlock]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginstateblock
/// [Clear]:                        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-clear
/// [ColorFill]:                    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-colorfill
/// [CreateAdditionalSwapChain]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createadditionalswapchain
/// [CreateCubeTexture]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture
/// [CreateDepthStencilSurface]:    https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createdepthstencilsurface
/// [CreateIndexBuffer]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer
/// [CreateOffscreenPlainSurface]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createoffscreenplainsurface
/// [CreatePixelShader]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createpixelshader
/// [CreateQuery]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createquery
/// [CreateRenderTarget]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createrendertarget
/// [CreateStateBlock]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createstateblock
/// [CreateTexture]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture
/// [CreateVertexBuffer]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer
/// [CreateVertexDeclaration]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexdeclaration
/// [CreateVertexShader]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexshader
/// [CreateVolumeTexture]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvolumetexture
/// [DeletePatch]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-deletepatch
/// [DrawIndexedPrimitive]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitive
/// [DrawIndexedPrimitiveUP]:       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitiveup
/// [DrawPrimitive]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitive
/// [DrawPrimitiveUP]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitiveup
/// [DrawRectPatch]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawrectpatch
/// [DrawTriPatch]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawtripatch
/// [EndScene]:                     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endscene
/// [EndStateBlock]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endstateblock
/// [EvictManagedResources]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-evictmanagedresources
/// [GetAvailableTextureMem]:       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getavailabletexturemem
/// [GetBackBuffer]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getbackbuffer
/// [GetClipPlane]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipplane
/// [GetClipStatus]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipstatus
/// [GetCreationParameters]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getcreationparameters
/// [GetCurrentTexturePalette]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getcurrenttexturepalette
/// [GetDepthStencilSurface]:       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdepthstencilsurface
/// [GetDeviceCaps]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdevicecaps
/// [GetDirect3D]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdirect3d
/// [GetDisplayMode]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdisplaymode
/// [GetFrontBufferData]:           https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getfrontbufferdata
/// [GetFVF]:                       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getfvf
/// [GetGammaRamp]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getgammaramp
/// [GetIndices]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getindices
/// [GetLight]:                     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getlight
/// [GetLightEnable]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getlightenable
/// [GetMaterial]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getmaterial
/// [GetNPatchMode]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getnpatchmode
/// [GetNumberOfSwapChains]:        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getnumberofswapchains
/// [GetPaletteEntries]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpaletteentries
/// [GetPixelShader]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshader
/// [GetPixelShaderConstantB]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshaderconstantb
/// [GetPixelShaderConstantF]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshaderconstantf
/// [GetPixelShaderConstantI]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getpixelshaderconstanti
/// [GetRasterStatus]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrasterstatus
/// [GetRenderState]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrenderstate
/// [GetRenderTarget]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertarget
/// [GetRenderTargetData]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertargetdata
/// [GetSamplerState]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsamplerstate
/// [GetScissorRect]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getscissorrect
/// [GetSoftwareVertexProcessing]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsoftwarevertexprocessing
/// [GetStreamSource]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getstreamsource
/// [GetStreamSourceFreq]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getstreamsourcefreq
/// [GetSwapChain]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getswapchain
/// [GetTexture]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-gettexture
/// [GetTextureStageState]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-gettexturestagestate
/// [GetTransform]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-gettransform
/// [GetVertexDeclaration]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexdeclaration
/// [GetVertexShader]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshader
/// [GetVertexShaderConstantB]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshaderconstantb
/// [GetVertexShaderConstantF]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshaderconstantf
/// [GetVertexShaderConstantI]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getvertexshaderconstanti
/// [GetViewport]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getviewport
/// [LightEnable]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-lightenable
/// [MultiplyTransform]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-multiplytransform
/// [Present]:                      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-present
/// [ProcessVertices]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-processvertices
/// [Reset]:                        https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-reset
/// [SetClipPlane]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setclipplane
/// [SetClipStatus]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setclipstatus
/// [SetCurrentTexturePalette]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcurrenttexturepalette
/// [SetCursorPosition]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorposition
/// [SetCursorProperties]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setcursorproperties
/// [SetDepthStencilSurface]:       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setdepthstencilsurface
/// [SetDialogBoxMode]:             https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setdialogboxmode
/// [SetFVF]:                       https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setfvf
/// [SetGammaRamp]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setgammaramp
/// [SetIndices]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setindices
/// [SetLight]:                     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setlight
/// [SetMaterial]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setmaterial
/// [SetNPatchMode]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setnpatchmode
/// [SetPaletteEntries]:            https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpaletteentries
/// [SetPixelShader]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshader
/// [SetPixelShaderConstantB]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshaderconstantb
/// [SetPixelShaderConstantF]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshaderconstantf
/// [SetPixelShaderConstantI]:      https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setpixelshaderconstanti
/// [SetRenderState]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setrenderstate
/// [SetRenderTarget]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setrendertarget
/// [SetSamplerState]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsamplerstate
/// [SetScissorRect]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setscissorrect
/// [SetSoftwareVertexProcessing]:  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsoftwarevertexprocessing
/// [SetStreamSource]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsource
/// [SetStreamSourceFreq]:          https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsourcefreq
/// [SetTexture]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-settexture
/// [SetTextureStageState]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-settexturestagestate
/// [SetTransform]:                 https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-settransform
/// [SetVertexDeclaration]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexdeclaration
/// [SetVertexShader]:              https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshader
/// [SetVertexShaderConstantB]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshaderconstantb
/// [SetVertexShaderConstantF]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshaderconstantf
/// [SetVertexShaderConstantI]:     https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexshaderconstanti
/// [SetViewport]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setviewport
/// [ShowCursor]:                   https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-showcursor
/// [StretchRect]:                  https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-stretchrect
/// [TestCooperativeLevel]:         https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-testcooperativelevel
/// [UpdateSurface]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-updatesurface
/// [UpdateTexture]:                https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-updatetexture
/// [ValidateDevice]:               https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-validatedevice
///
pub trait IDirect3DDevice9Ext : AsSafe<IDirect3DDevice9> + Sized {
    // TODO: fn scene(&self) with sane error handling / drop behavior?

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginscene)\]
    /// IDirect3DDevice9::BeginScene
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.begin_scene().unwrap();
    /// // ...issue draw calls and stuff...
    /// device.end_scene().unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device was already within a scene (e.g. [begin_scene] was called twice without an intervening [end_scene])
    /// *   `Ok(())`                if the device was not already within a scene, and now is
    ///
    /// [begin_scene]:          Self::begin_scene
    /// [end_scene]:            Self::end_scene
    fn begin_scene(&self) -> Result<(), MethodError> {
        // TODO: examples
        let hr = unsafe { self.as_winapi().BeginScene() };
        MethodError::check("IDirect3DDevice9::BeginScene", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-beginstateblock)\]
    /// IDirect3DDevice9::BeginStateBlock
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device was already within a state block
    /// *   `Ok(())`                otherwise
    fn begin_state_block(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().BeginStateBlock() };
        MethodError::check("IDirect3DDevice9::BeginStateBlock", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-clear)\]
    /// IDirect3DDevice9::Clear
    ///
    /// Clears one or more surfaces such as a render target, multiple render targets, a stencil buffer, and a depth buffer.
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]     if `rects.len()` > `u32::MAX`
    /// * [D3DERR::INVALIDCALL]     if all non-`rects` parameters were `None`
    /// * [D3DERR::INVALIDCALL]     if `color`   was `Some(...)` without a render target being bound
    /// * [D3DERR::INVALIDCALL]     if `depth`   was `Some(...)` without a depth buffer being bound
    /// * [D3DERR::INVALIDCALL]     if `stencil` was `Some(...)` without a stencil buffer being bound
    /// * `Ok(())`                  otherwise
    fn clear(&self, rects: Option<&[Rect]>, color: Option<Color>, depth: Option<f32>, stencil: Option<u32>) -> Result<(), MethodError> {
        // TODO: more clear docs
        // TODO: conversion traits for params?

        let n : u32 = rects.map_or(0, |r| r.len()).try_into().map_err(|_| MethodError("Device::clear", D3DERR::INVALIDCALL))?;
        let rects = rects.map_or(null(), |r| r.as_ptr().cast());

        let flags =
            ((color  .is_some() as u32) * D3DCLEAR_TARGET ) |
            ((depth  .is_some() as u32) * D3DCLEAR_ZBUFFER) |
            ((stencil.is_some() as u32) * D3DCLEAR_STENCIL);

        let color   = color.unwrap_or_default().into();
        let depth   = depth.unwrap_or(0.0);
        let stencil = stencil.unwrap_or(0);

        let hr = unsafe { self.as_winapi().Clear(n, rects, flags, color, depth, stencil) };
        MethodError::check("IDirect3DDevice9::Clear", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-colorfill)\]
    /// IDirect3DDevice9::ColorFill
    ///
    /// Allows an application to fill a rectangular area of a [Pool::Default] surface with a specified color.
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]     if `surface` isn't from [Pool::Default] ?
    /// * [D3DERR::INVALIDCALL]     if `surface` isn't a supported format ?
    /// * [D3DERR::INVALIDCALL]     if `rect` exceeds the bounds of the surface
    /// * `Ok(())`                  on success
    fn color_fill(&self, surface: &Surface, rect: Option<Rect>, color: impl Into<Color>) -> Result<(), MethodError> {
        let rect = rect.map(RECT::from);
        let rect = rect.as_ref().map_or(null(), |r| r);
        let hr = unsafe { self.as_winapi().ColorFill(surface.as_raw(), rect, color.into().into()) };
        MethodError::check("IDirect3DDevice9::ColorFill", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createadditionalswapchain)\]
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
    ///
    /// *   The caller's codebase is responsible for ensuring any [HWND]s inside [D3DPRESENT_PARAMETERS] outlive the resulting [SwapChain]s that depend on them.
    ///     See [IDirect3D9Ext::create_device] for details and guidance about dealing with this lifetime issue.
    ///
    /// ### Returns
    ///
    /// * [D3DERR::NOTAVAILABLE]
    /// * [D3DERR::DEVICELOST]
    /// * [D3DERR::INVALIDCALL]
    /// * [D3DERR::OUTOFVIDEOMEMORY]
    /// * [E::OUTOFMEMORY]
    /// * Ok([SwapChain])
    ///
    /// ### See Also
    ///
    /// * [Presenting Multiple Views in Windowed Mode (Direct3D 9)](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/presenting-multiple-views-in-windowed-mode)
    ///
    /// [create_device]:            #method.create_device
    unsafe fn create_additional_swap_chain(&self, presentation_parameters: &mut D3DPRESENT_PARAMETERS) -> Result<SwapChain, MethodError> {
        let mut swap_chain = null_mut();
        let hr = unsafe { self.as_winapi().CreateAdditionalSwapChain(presentation_parameters, &mut swap_chain) };
        MethodError::check("IDirect3DDevice9::CreateAdditionalSwapChain", hr)?;
        Ok(unsafe { SwapChain::from_raw(swap_chain) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture)\]
    /// IDirect3DDevice9::CreateCubeTexture
    ///
    /// Creates a cube texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([CubeTexture])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 6 x 128x128 ARGB cubemap with no mipmaps
    /// let texture = device.create_cube_texture(128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_cube_texture(1 << 15, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).err());
    /// ```
    fn create_cube_texture(&self, edge_length: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<CubeTexture, MethodError> {
        let mut texture = null_mut();
        let hr = unsafe { self.as_winapi().CreateCubeTexture(edge_length, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut texture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateCubeTexture", hr)?;
        Ok(unsafe { CubeTexture::from_raw(texture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createcubetexture)\]
    /// IDirect3DDevice9::CreateCubeTexture
    ///
    /// Creates a cube texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([CubeTexture])
    ///
    /// ### Example
    ///
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
    fn create_cube_texture_from(&self, size: u32, mips: &[CubeTextureMipRef], usage: impl Into<Usage>, format: &FixedTextureFormat, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<CubeTexture, MethodError> {
        // TODO: consider THINERR::* constants instead?
        if size == 0        { return Err(MethodError("IDirect3DDevice9Ext::create_cube_texture_from", D3DERR::INVALIDCALL)); }
        if mips.is_empty()  { return Err(MethodError("IDirect3DDevice9Ext::create_cube_texture_from", D3DERR::INVALIDCALL)); } // 0 levels = autogenerate mips, which is different from no levels

        let levels : u32    = mips.len().try_into().map_err(|_| MethodError("IDirect3DDevice9Ext::create_cube_texture_from", THINERR::SLICE_TOO_LARGE))?;
        let usage           = usage.into();
        let pool            = pool.into();
        let texture         = self.create_cube_texture(size, levels, usage, format.format, pool, _shared_handle)?;
        let block_bits      = u32::from(format.bits_per_block);
        let block_width     = u32::from(format.block_size.0);
        let block_height    = u32::from(format.block_size.1);
        let is_dynamic      = 0 != (usage.into() & d3d::Usage::Dynamic.into());

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

                    // In Direct3D 8+, Pitch is bytes per *row of blocks* (ref: https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-rect)
                    let dst = unsafe { dst_origin.add((block_row * dst_pitch) as usize) };

                    unsafe { std::ptr::copy_nonoverlapping(src, dst, block_row_bytes) };
                }

                texture.unlock_rect(face, mip_level)?;
            }

            mip_pixels_size >>= 1;
        }

        Ok(texture)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createdepthstencilsurface)\]
    /// IDirect3DDevice9::CreateDepthStencilSurface
    ///
    /// Creates a depth-stencil resource.
    fn create_depth_stencil_surface(&self, width: u32, height: u32, format: Format, multi_sample: MultiSample, multi_sample_quality: u32, discard: bool, _shared_handle: impl SharedHandleParam) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.as_winapi().CreateDepthStencilSurface(width, height, format.into(), multi_sample.into(), multi_sample_quality, discard.into(), &mut surface, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateDepthStencilSurface", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer)\]
    /// IDirect3DDevice9::CreateIndexBuffer
    ///
    /// Creates an index buffer.
    ///
    /// ### Arguments
    ///
    /// *   `length`            Size of the index buffer, **in bytes**.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `format`            Typically [Format::Index16] or [Format::Index32] (type of index buffer)
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_index_buffer(3 * 2, Usage::None, Format::Index16, Pool::Managed, ()).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one index (2 for [Format::Index16], 4 for [Format::Index32])
    /// *   [D3DERR::INVALIDCALL]       if `usage`, `format`, or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::INVALIDDATA]
    /// *   [E::OUTOFMEMORY]
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([IndexBuffer])
    fn create_index_buffer(&self, length: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<IndexBuffer, MethodError> {
        // !0 will fail OUTOFMEMORY
        // !0/2 spammed will fail OUTOFVIDEOMEMORY
        // !0-4 spammed will "succeed", hinting at an arithmetic overflow within d3d or the driver
        if length > MAX_BUFFER_ALLOC { return Err(MethodError("IDirect3DDevice9Ext::create_index_buffer", THINERR::ALLOC_OVERFLOW)); }

        let _ = shared_handle;
        let mut buffer = null_mut();
        let hr = unsafe { self.as_winapi().CreateIndexBuffer(length, usage.into().into(), format.into().into(), pool.into().into(), &mut buffer, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateIndexBuffer", hr)?;
        Ok(unsafe { IndexBuffer::from_raw(buffer) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createindexbuffer)\]
    /// IDirect3DDevice9::CreateIndexBuffer
    ///
    /// Creates an index buffer.
    ///
    /// ### Arguments
    ///
    /// *   `data`              &\[u16\] or &\[u32\]
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let inds : &[u16] = &[0, 1, 2, 0, 2, 3];
    /// let tri = device.create_index_buffer_from(inds, Usage::None, Pool::Managed, ()).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one index (2 for [Format::Index16], 4 for [Format::Index32])
    /// *   [D3DERR::INVALIDCALL]       if `usage`, `format`, or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::INVALIDDATA]
    /// *   [E::OUTOFMEMORY]
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([IndexBuffer])
    fn create_index_buffer_from<I: Index>(&self, data: &[I], usage: impl Into<Usage>, pool: impl Into<Pool>, shared_handle: impl SharedHandleParam) -> Result<IndexBuffer, MethodError> {
        let bytes = std::mem::size_of_val(data);
        let bytes32 : u32 = bytes.try_into().map_err(|_| MethodError("IDirect3DDevice9Ext::create_index_buffer_from", THINERR::ALLOC_OVERFLOW))?;
        let usage = usage.into();
        let lock = if usage.into() & Usage::Dynamic.into() != 0 { Lock::NoOverwrite } else { Lock::None };
        let ib = self.create_index_buffer(bytes32, usage, I::format(), pool, shared_handle)?;
        unsafe {
            let dst = ib.lock_unchecked(0, 0, lock)?;
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, dst.cast(), bytes);
            ib.unlock()?;
        }
        Ok(ib)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createoffscreenplainsurface)\]
    /// IDirect3DDevice9::CreateOffscreenPlainSurface
    ///
    /// Create an off-screen surface.
    fn create_offscreen_plain_surface(&self, width: u32, height: u32, format: Format, pool: Pool, _shared_handle: impl SharedHandleParam) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.as_winapi().CreateOffscreenPlainSurface(width, height, format.into(), pool.into(), &mut surface, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateOffscreenPlainSurface", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createpixelshader)\]
    /// IDirect3DDevice9::CreatePixelShader
    ///
    /// Creates a pixel shader.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// The caller must pass a valid shader blob.
    /// The underlying Direct3D API is unsound - it doesn't even take a length for the DWORD array.
    /// This function will likely attempt to validate the shader blob bytecode in the future and/or offload such validation onto the parameter, but until then this is unsound as heck.
    /// Do not trust user-generated-content for shader bytecode blobs.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([PixelShader])
    unsafe fn create_pixel_shader(&self, function: &[u32]) -> Result<PixelShader, MethodError> {
        let mut shader = null_mut();
        let hr = unsafe { self.as_winapi().CreatePixelShader(function.as_ptr(), &mut shader) };
        MethodError::check("IDirect3DDevice9::CreatePixelShader", hr)?;
        Ok(unsafe { PixelShader::from_raw(shader) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createquery)\]
    /// IDirect3DDevice9::CreateQuery
    ///
    /// Creates a status query.
    fn create_query(&self, type_: QueryType) -> Result<Query, MethodError> {
        let mut query = null_mut();
        let hr = unsafe { self.as_winapi().CreateQuery(type_.into(), &mut query) };
        MethodError::check("IDirect3DDevice9::CreateQuery", hr)?;
        Ok(unsafe { Query::from_raw(query) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createrendertarget)\]
    /// IDirect3DDevice9::CreateRenderTarget
    ///
    /// Creates a render-target surface.
    fn create_render_target(&self, width: u32, height: u32, format: Format, multi_sample: MultiSample, multi_sample_quality: u32, lockable: bool, _shared_handle: impl SharedHandleParam) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.as_winapi().CreateRenderTarget(width, height, format.into(), multi_sample.into(), multi_sample_quality, lockable.into(), &mut surface, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateRenderTarget", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createstateblock)\]
    /// IDirect3DDevice9::CreateStateBlock
    ///
    /// Creates a new state block that contains the values for all device states, vertex-related states, or pixel-related states.
    ///
    /// Vertex-related device states typically refer to those states that affect how the system processes vertices.
    /// Pixel-related states generally refer to device states that affect how the system processes pixel or depth-buffer data during rasterization.
    /// Some states are contained in both groups.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([StateBlock])
    fn create_state_block(&self, type_: StateBlockType) -> Result<StateBlock, MethodError> {
        let mut sb = null_mut();
        let hr = unsafe { self.as_winapi().CreateStateBlock(type_.into(), &mut sb) };
        MethodError::check("IDirect3DDevice9::CreateStateBlock", hr)?;
        Ok(unsafe { StateBlock::from_raw(sb) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture)\]
    /// IDirect3DDevice9::CreateTexture
    ///
    /// Creates a texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([Texture])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 128x128 ARGB texture with no mipmaps
    /// let texture = device.create_texture(128, 128, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_texture(1 << 15, 1 << 15, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).err());
    /// ```
    fn create_texture(&self, width: u32, height: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<Texture, MethodError> {
        let mut texture = null_mut();
        let hr = unsafe { self.as_winapi().CreateTexture(width, height, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut texture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateTexture", hr)?;
        Ok(unsafe { Texture::from_raw(texture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createtexture)\]
    /// IDirect3DDevice9::CreateTexture
    ///
    /// Creates a texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([Texture])
    ///
    /// ### Example
    ///
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
    fn create_texture_from(&self, width: u32, height: u32, mips: &[TextureMipRef], usage: impl Into<Usage>, format: &FixedTextureFormat, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<Texture, MethodError> {
        // TODO: consider THINERR::* constants instead?
        if width == 0       { return Err(MethodError("IDirect3DDevice9Ext::create_texture_from", D3DERR::INVALIDCALL)); }
        if height == 0      { return Err(MethodError("IDirect3DDevice9Ext::create_texture_from", D3DERR::INVALIDCALL)); }
        if mips.is_empty()  { return Err(MethodError("IDirect3DDevice9Ext::create_texture_from", D3DERR::INVALIDCALL)); } // 0 levels = autogenerate mips, which is different from no levels

        let levels : u32    = mips.len().try_into().map_err(|_| MethodError("IDirect3DDevice9Ext::create_texture_from", THINERR::SLICE_TOO_LARGE))?;
        let usage           = usage.into();
        let pool            = pool.into();
        let texture         = self.create_texture(width, height, levels, usage, format.format, pool, _shared_handle)?;
        let block_bits      = u32::from(format.bits_per_block);
        let block_width     = u32::from(format.block_size.0);
        let block_height    = u32::from(format.block_size.1);
        let is_dynamic      = 0 != (usage.into() & d3d::Usage::Dynamic.into());

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

                // In Direct3D 8+, Pitch is bytes per *row of blocks* (ref: https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-rect)
                let dst = unsafe { dst_origin.add((block_row * dst_pitch) as usize) };

                unsafe { std::ptr::copy_nonoverlapping(src, dst, block_row_bytes) };
            }

            texture.unlock_rect(mip_level)?;
            mip_pixels_width    >>= 1;
            mip_pixels_height   >>= 1;
        }

        Ok(texture)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer)\]
    /// IDirect3DDevice9::CreateVertexBuffer
    ///
    /// Creates an vertex buffer.
    ///
    /// ### Arguments
    ///
    /// *   `length`            Size of the index buffer, **in bytes**.
    ///                         For FVF vertex buffers, Length must be large enough to contain at least one vertex, but it need not be a multiple of the vertex size.
    ///                         Length is not validated for non-FVF buffers.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `fvf`               Combination of [FVF], a usage specifier that describes the vertex format of the verticies in this buffer.
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let vert_size = 3 * 4; // XYZ * floats
    /// let length = 3 * vert_size; // 3 verts
    /// let tri = device.create_vertex_buffer(length, Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one [FVF]-sized vertex (1 if [FVF::None])
    /// *   [D3DERR::INVALIDCALL]       if `usage` or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]  if allocation failed (driver or gpu memory)
    /// *   [E::OUTOFMEMORY]            if allocation failed (driver or d3d runtime)
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([VertexBuffer])
    fn create_vertex_buffer(&self, length: u32, usage: impl Into<Usage>, fvf: impl Into<FVF>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VertexBuffer, MethodError> {
        // !0 will fail OUTOFMEMORY
        // !0/2 spammed will fail OUTOFVIDEOMEMORY
        // !0-4 spammed will "succeed", hinting at an arithmetic overflow within d3d or the driver
        if length > MAX_BUFFER_ALLOC { return Err(MethodError("IDirect3DDevice9Ext::create_vertex_buffer", THINERR::ALLOC_OVERFLOW)); }

        let mut buffer = null_mut();
        let hr = unsafe { self.as_winapi().CreateVertexBuffer(length, usage.into().into(), fvf.into().into(), pool.into().into(), &mut buffer, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateVertexBuffer", hr)?;
        Ok(unsafe { VertexBuffer::from_raw(buffer) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexbuffer)\]
    /// IDirect3DDevice9::CreateVertexBuffer
    ///
    /// Creates an vertex buffer.
    ///
    /// ### Arguments
    ///
    /// *   `data`              Data to initialize the vertex buffer with.
    /// *   `usage`             Typically [Usage::None] or [Usage::Dynamic]
    /// *   `fvf`               Combination of [FVF], a usage specifier that describes the vertex format of the verticies in this buffer.
    /// *   `pool`              Memory class into which to place the [IndexBuffer].
    /// *   `shared_handle`     Used in Direct3D 9 for Windows Vista to [share resources](https://docs.microsoft.com/en-us/windows/desktop/direct3d9/dx9lh); set it to `()` to not share a resource.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let tri = device.create_vertex_buffer_from(&[
    ///     [0.0_f32, 1.0, 0.0],
    ///     [1.0_f32, 0.0, 0.0],
    ///     [0.0_f32, 0.0, 1.0],
    /// ][..], Usage::None, FVF::XYZ, Pool::Managed, ()).unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if `length` cannot hold at least one [FVF]-sized vertex (1 if [FVF::None])
    /// *   [D3DERR::INVALIDCALL]       if `usage` or `pool` is invalid
    /// *   [D3DERR::OUTOFVIDEOMEMORY]  if allocation failed (driver or gpu memory)
    /// *   [E::OUTOFMEMORY]            if allocation failed (driver or d3d runtime)
    /// *   [THINERR::ALLOC_OVERFLOW]   if allocation rejected by thindx to avoid possible UB
    /// *   Ok([VertexBuffer])
    fn create_vertex_buffer_from<V: Pod>(&self, data: &[V], usage: impl Into<Usage>, fvf: impl Into<FVF>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VertexBuffer, MethodError> {
        let bytes = std::mem::size_of_val(data);
        let bytes32 : u32 = bytes.try_into().map_err(|_| MethodError("IDirect3DDevice9Ext::create_vertex_buffer_from", THINERR::ALLOC_OVERFLOW))?;
        let usage = usage.into();
        let lock = if usage.into() & Usage::Dynamic.into() != 0 { Lock::NoOverwrite } else { Lock::None };
        let vb = self.create_vertex_buffer(bytes32, usage, fvf, pool, _shared_handle)?;
        unsafe {
            let dst = vb.lock_unchecked(0, 0, lock)?;
            std::ptr::copy_nonoverlapping(data.as_ptr() as *const u8, dst.cast(), bytes);
            vb.unlock()?;
        }
        Ok(vb)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexdeclaration)\]
    /// IDirect3DDevice9::CreateVertexDeclaration
    ///
    /// Create a vertex shader declaration from the device and the vertex elements.
    ///
    /// See the [Vertex Declaration (Direct3D 9)] page for a detailed description of how to map vertex declarations between different versions of DirectX.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if `elements.last()` != `Some(D3DDECL_END)`
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([VertexDeclaration])
    ///
    /// [Vertex Declaration (Direct3D 9)]:          https://docs.microsoft.com/en-us/windows/desktop/direct3d9/vertex-declaration
    fn create_vertex_declaration(&self, elements: &[VertexElement]) -> Result<VertexDeclaration, MethodError> {
        let end = elements.last().ok_or(MethodError("Device::create_vertex_declaration", D3DERR::INVALIDCALL))?;
        // This check is required for CreateVertexDeclaration to be sound!
        if *end != VertexElement::END { return Err(MethodError("Device::create_vertex_declaration", D3DERR::INVALIDCALL)); }

        let mut vd = null_mut();
        let hr = unsafe { self.as_winapi().CreateVertexDeclaration(elements.as_ptr().cast(), &mut vd) };
        MethodError::check("IDirect3DDevice9::CreateVertexDeclaration", hr)?;
        Ok(unsafe { VertexDeclaration::from_raw(vd) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvertexshader)\]
    /// IDirect3DDevice9::CreateVertexShader
    ///
    /// Creates a vertex shader.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// The caller must pass a valid shader blob.
    /// The underlying Direct3D API is unsound - it doesn't even take a length for the DWORD array.
    /// This function will likely attempt to validate the shader blob bytecode in the future and/or offload such validation onto the parameter, but until then this is unsound as heck.
    /// Do not trust user-generated-content for shader bytecode blobs.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([VertexShader])
    unsafe fn create_vertex_shader(&self, function: &[u32]) -> Result<VertexShader, MethodError> {
        let mut shader = null_mut();
        let hr = unsafe { self.as_winapi().CreateVertexShader(function.as_ptr(), &mut shader) };
        MethodError::check("IDirect3DDevice9::CreateVertexShader", hr)?;
        Ok(unsafe { VertexShader::from_raw(shader) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvolumetexture)\]
    /// IDirect3DDevice9::CreateVolumeTexture
    ///
    /// Creates a volume texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([VolumeTexture])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// // Create a 32x32x32 volumetric ARGB texture with no mipmaps
    /// let texture = device.create_volume_texture(32, 32, 32, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).unwrap();
    /// assert_eq!(D3DERR::INVALIDCALL, device.create_volume_texture(1 << 10, 1 << 10, 1 << 10, 0, Usage::None, Format::A8R8G8B8, Pool::Default, ()).err());
    /// ```
    fn create_volume_texture(&self, width: u32, height: u32, depth: u32, levels: u32, usage: impl Into<Usage>, format: impl Into<Format>, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VolumeTexture, MethodError> {
        let mut volumetexture = null_mut();
        let hr = unsafe { self.as_winapi().CreateVolumeTexture(width, height, depth, levels, usage.into().into(), format.into().into(), pool.into().into(), &mut volumetexture, null_mut()) };
        MethodError::check("IDirect3DDevice9::CreateVolumeTexture", hr)?;
        Ok(unsafe { VolumeTexture::from_raw(volumetexture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-createvolumetexture)\]
    /// IDirect3DDevice9::CreateVolumeTexture
    ///
    /// Creates a volume texture resource.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       On various invalid parameters, including the texture size being beyond the device's capabilities
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [E::OUTOFMEMORY]
    /// *   Ok([VolumeTexture])
    ///
    /// ### Example
    ///
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
    fn create_volume_texture_from(&self, width: u32, height: u32, depth: u32, mips: &[VolumeTextureMipRef], usage: impl Into<Usage>, format: &FixedTextureFormat, pool: impl Into<Pool>, _shared_handle: impl SharedHandleParam) -> Result<VolumeTexture, MethodError> {
        // TODO: consider THINERR::* constants instead?
        if width == 0       { return Err(MethodError("IDirect3DDevice9Ext::create_volume_texture_from", D3DERR::INVALIDCALL)); }
        if height == 0      { return Err(MethodError("IDirect3DDevice9Ext::create_volume_texture_from", D3DERR::INVALIDCALL)); }
        if mips.is_empty()  { return Err(MethodError("IDirect3DDevice9Ext::create_volume_texture_from", D3DERR::INVALIDCALL)); } // 0 levels = autogenerate mips, which is different from no levels

        let levels : u32    = mips.len().try_into().map_err(|_| MethodError("IDirect3DDevice9Ext::create_volume_texture_from", THINERR::SLICE_TOO_LARGE))?;
        let usage           = usage.into();
        let pool            = pool.into();
        let texture         = self.create_volume_texture(width, height, depth, levels, usage, format.format, pool, _shared_handle)?;
        let block_bits      = u32::from(format.bits_per_block);
        let block_width     = u32::from(format.block_size.0);
        let block_height    = u32::from(format.block_size.1);
        let is_dynamic      = 0 != (usage.into() & d3d::Usage::Dynamic.into());

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

                    // In Direct3D 8+, Pitch is bytes per *row of blocks* (ref: https://docs.microsoft.com/en-us/windows/win32/direct3d9/d3dlocked-rect)
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

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitive)\]
    /// IDirect3DDevice9::DrawIndexedPrimitive
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_indexed_primitive(&self, primitive_type: PrimitiveType, base_vertex_index: i32, min_vertex_index: u32, num_verticies: u32, start_index: u32, primitive_count: u32) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().DrawIndexedPrimitive(primitive_type.into(), base_vertex_index, min_vertex_index, num_verticies, start_index, primitive_count) };
        MethodError::check("IDirect3DDevice9::DrawIndexedPrimitive", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawindexedprimitiveup)\]
    /// IDirect3DDevice9::DrawIndexedPrimitiveUP
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_indexed_primitive_up<I: Index, V: Pod>(&self, primitive_type: PrimitiveType, min_vertex_index: u32, num_verticies: u32, primitive_count: u32, indicies: &[I], vertex_stream_zero: &[V]) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().DrawIndexedPrimitiveUP(primitive_type.into(), min_vertex_index, num_verticies, primitive_count, indicies.as_ptr().cast(), I::format().into(), vertex_stream_zero.as_ptr().cast(), std::mem::size_of::<V>() as _) };
        MethodError::check("IDirect3DDevice9::DrawIndexedPrimitiveUP", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitive)\]
    /// IDirect3DDevice9::DrawPrimitive
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_primitive(&self, primitive_type: PrimitiveType, start_vertex: u32, primitive_count: u32) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().DrawPrimitive(primitive_type.into(), start_vertex, primitive_count) };
        MethodError::check("IDirect3DDevice9::DrawPrimitive", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-drawprimitiveup)\]
    /// IDirect3DDevice9::DrawPrimitiveUP
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// Draw calls are unsafe as heck.  Not just the parameters, but the [`Device`]'s current state must be valid for the draw call.
    /// Unbound resources that a shader depends on, missing index buffers for indexed draw calls... the possibilities for undefined behavior are endless.
    /// Safe-yet-performant wrappers around these calls will likely be fed entire scenegraphs or the like, alongside validated or trusted shaders and meshes.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    unsafe fn draw_primitive_up<V: Pod>(&self, primitive_type: PrimitiveType, primitive_count: u32, vertex_stream_zero: &[V]) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().DrawPrimitiveUP(primitive_type.into(), primitive_count, vertex_stream_zero.as_ptr().cast(), std::mem::size_of::<V>() as _) };
        MethodError::check("IDirect3DDevice9::DrawPrimitiveUP", hr)
    }

    // TODO: DrawRectPatch
    // TODO: DrawTriPatch

    // TODO: docs for args, remarks, deep links, etc.
    // TODO: safety sections

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endscene)\]
    /// IDirect3DDevice9::EndScene
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// device.begin_scene().unwrap();
    /// // ...issue draw calls and stuff...
    /// device.end_scene().unwrap();
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device was not within a scene (e.g. [end_scene] was called without a [begin_scene], or was called a second time)
    /// *   `Ok(())`                if the device was within a scene that has now ended
    ///
    /// [begin_scene]:          Self::begin_scene
    /// [end_scene]:            Self::end_scene
    fn end_scene(&self) -> Result<(), MethodError> {
        // TODO: examples
        let hr = unsafe { self.as_winapi().EndScene() };
        MethodError::check("IDirect3DDevice9::EndScene", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-endstateblock)\]
    /// IDirect3DDevice9::EndStateBlock
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the device wasn't within a state block
    /// *   `Ok(state_block)`       if a state block was successfully captured
    fn end_state_block(&self) -> Result<StateBlock, MethodError> {
        let mut sb = null_mut();
        let hr = unsafe { self.as_winapi().EndStateBlock(&mut sb) };
        MethodError::check("IDirect3DDevice9::EndStateBlock", hr)?;
        Ok(unsafe { StateBlock::from_raw(sb) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-evictmanagedresources)]\
    /// IDirect3DDevice9::EvictManagedResources
    ///
    /// Evicts all managed resources, including both Direct3D and driver-managed resources.
    ///
    /// This function causes only the [Pool::Default] copy of resources to be evicted.
    /// The resource copy in system memory is retained. See [Pool].
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::OUTOFVIDEOMEMORY]
    /// *   [D3DERR::COMMAND_UNPARSED]
    /// *   Ok(())
    fn evict_managed_resources(&self) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().EvictManagedResources() };
        MethodError::check("IDirect3DDevice9::EvictManagedResources", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getavailabletexturemem)\]
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
    ///
    /// *   `0xFFE00000`
    /// *   Maybe occasionally some other values too
    ///
    /// ### Example
    ///
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
    ///
    /// ```text
    /// > 4 GiB available
    /// ```
    fn get_available_texture_mem(&self) -> u32 {
        unsafe { self.as_winapi().GetAvailableTextureMem() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getbackbuffer)\]
    /// IDirect3DDevice9::GetBackBuffer
    ///
    /// Retrieves a back buffer from the device's swap chain.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]           if `back_buffer` >= number of back buffers
    /// *   Ok([Surface])
    fn get_back_buffer(&self, swap_chain: u32, back_buffer: u32, type_: BackBufferType) -> Result<Surface, MethodError> {
        let mut surface = null_mut();
        let hr = unsafe { self.as_winapi().GetBackBuffer(swap_chain, back_buffer, type_.into(), &mut surface) };
        MethodError::check("IDirect3DDevice9::GetBackBuffer", hr)?;
        Ok(unsafe { Surface::from_raw(surface) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipplane)\]
    /// IDirect3DDevice9::GetClipPlane
    ///
    /// Retrieves the coefficients of a user-defined clipping plane for the device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   if the index exceeds the maximum clipping pane index supported by the device (if there is such a limit)
    /// *   `Ok([A, B, C, D])`, where points `Ax + By + Cz + Dw >= 0` are visible
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// println!("{:?}", device.get_clip_plane(0).unwrap());
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// [0.0, 0.0, 0.0, 0.0]
    /// ```
    fn get_clip_plane(&self, index: u32) -> Result<[f32; 4], MethodError> {
        let mut plane = [0.0, 0.0, 0.0, 0.0];
        let hr = unsafe { self.as_winapi().GetClipPlane(index, plane.as_mut_ptr()) };
        MethodError::check("IDirect3DDevice9::GetClipPlane", hr)?;
        Ok(plane)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getclipstatus)\]
    /// IDirect3DDevice9::GetClipStatus
    ///
    /// Retrieves the clip status.
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]  - "if the argument is invalid", but this should always be valid"
    /// *   Ok([ClipStatus])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// println!("{:?}", device.get_clip_status().unwrap());
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// ClipStatus { ClipUnion: 0, ClipIntersection: 4294967295 }
    /// ```
    fn get_clip_status(&self) -> Result<ClipStatus, MethodError> {
        let mut status = D3DCLIPSTATUS9 { ClipUnion: 0, ClipIntersection: 0 };
        let hr = unsafe { self.as_winapi().GetClipStatus(&mut status) };
        MethodError::check("IDirect3DDevice9::GetClipStatus", hr)?;
        Ok(ClipStatus::from_unchecked(status))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getcreationparameters)\]
    /// IDirect3DDevice9::GetCreationParameters
    ///
    /// Retrieves the creation parameters of the device.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   "If the returned argument is invalid" (impossible via thindx?)
    /// *   Ok(())
    fn get_creation_parameters(&self) -> Result<D3DDEVICE_CREATION_PARAMETERS, MethodError> {
        let mut dcp = unsafe { std::mem::zeroed::<D3DDEVICE_CREATION_PARAMETERS>() };
        let hr = unsafe { self.as_winapi().GetCreationParameters(&mut dcp) };
        MethodError::check("IDirect3DDevice9::GetCreationParameters", hr)?;
        Ok(dcp)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getcurrenttexturepalette)\]
    /// IDirect3DDevice9::GetCurrentTexturePalette
    ///
    /// Retrieves the current texture palette
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   "If the method fails" (impossible via thindx?)
    /// *   Ok(`texture_palette_index`)
    fn get_current_texture_palette(&self) -> Result<u32, MethodError> {
        let mut pal = 0;
        let hr = unsafe { self.as_winapi().GetCurrentTexturePalette(&mut pal) };
        MethodError::check("IDirect3DDevice9::GetCurrentTexturePalette", hr)?;
        Ok(pal)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getdepthstencilsurface)\]
    /// IDirect3DDevice9::GetDepthStencilSurface
    ///
    /// Gets the depth-stencil surface owned by the Direct3DDevice object.
    ///
    /// ### Returns
    ///
    /// * <span style="inaccurate">[D3DERR::INVALIDCALL] ...?</span>
    /// * Ok(Some([Surface]))       the render target bound to that index
    /// * Ok(None)                  no render target was bound to that index
    ///
    /// [Multiple Render Targets (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-render-targets
    fn get_depth_stencil_surface(&self) -> Result<Option<Surface>, MethodError> {
        // TODO: verify soundness before making pub
        let mut ds = null_mut();
        let hr = unsafe { self.as_winapi().GetDepthStencilSurface(&mut ds) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            MethodError::check("IDirect3DDevice9::GetDepthStencilSurface", hr)?;
            Ok(unsafe { Surface::from_raw_opt(ds) })
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdevicecaps)\]
    /// IDirect3DDevice9::GetDeviceCaps
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([Caps])                  otherwise
    ///
    /// ### Example
    ///
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
    ///
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
    fn get_device_caps(&self) -> Result<Caps, MethodError> {
        let mut caps = Caps::zeroed();
        let hr = unsafe { self.as_winapi().GetDeviceCaps(&mut *caps) };
        MethodError::check("IDirect3DDevice9::GetDeviceCaps", hr)?;
        Ok(caps)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdirect3d)\]
    /// IDirect3DDevice9::GetDirect3D
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([Direct3D])              otherwise
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let d3d : Direct3D = device.get_direct3d().unwrap();
    /// ```
    fn get_direct3d(&self) -> Result<Direct3D, MethodError> {
        let mut d3d = null_mut();
        let hr = unsafe { self.as_winapi().GetDirect3D(&mut d3d) };
        MethodError::check("IDirect3DDevice9::GetDirect3D", hr)?;
        Ok(unsafe { Direct3D::from_raw(d3d) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getdisplaymode)\]
    /// IDirect3DDevice9::GetDisplayMode
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([DisplayMode])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let mode : DisplayMode = device.get_display_mode(0).unwrap();
    /// println!("{:#?}", mode);
    /// assert!(mode.width > 0);
    /// assert_eq!(mode.format, Format::X8R8G8B8);
    /// ```
    ///
    /// ### Output
    ///
    /// ```text
    /// DisplayMode {
    ///     width: 2160,
    ///     height: 3840,
    ///     refresh_rate: 60,
    ///     format: Format(D3DFMT_X8R8G8B8),
    /// }
    /// ```
    fn get_display_mode(&self, swap_chain: u32) -> Result<DisplayMode, MethodError> {
        let mut dm = DisplayMode::zeroed();
        let hr = unsafe { self.as_winapi().GetDisplayMode(swap_chain, &mut *dm) };
        MethodError::check("IDirect3DDevice9::GetDisplayMode", hr)?;
        Ok(dm)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getfrontbufferdata)\]
    /// IDirect3DDevice9::GetFrontBufferData
    fn get_front_buffer_data(&self, swap_chain: u32, surface: &Surface) -> Result<(), MethodError> {
        // TODO: verify soundness before making pub
        let hr = unsafe { self.as_winapi().GetFrontBufferData(swap_chain, surface.as_raw()) };
        MethodError::check("IDirect3DDevice9::GetFrontBufferData", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getfvf)\]
    /// IDirect3DDevice9::GetFVF
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails" (impossible via thindx?)
    /// *   Ok([FVF])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// assert_eq!(device.get_fvf().unwrap(), FVF::None);
    /// ```
    fn get_fvf(&self) -> Result<FVF, MethodError> {
        let mut fvf = FVF::None;
        let hr = unsafe { self.as_winapi().GetFVF(&mut *fvf) };
        MethodError::check("IDirect3DDevice9::GetFVF", hr)?;
        Ok(fvf)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getgammaramp)\]
    /// IDirect3DDevice9::GetGammaRamp
    ///
    /// ### Returns
    ///
    /// *   [D3DGAMMARAMP]
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let ramp = device.get_gamma_ramp(0);
    /// ```
    fn get_gamma_ramp(&self, swap_chain: u32) -> D3DGAMMARAMP {
        let mut ramp = unsafe { std::mem::zeroed::<D3DGAMMARAMP>() };
        let _nohr : () = unsafe { self.as_winapi().GetGammaRamp(swap_chain, &mut ramp) };
        ramp
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getindices)\]
    /// IDirect3DDevice9::GetIndices
    ///
    /// Retrieves index data.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// # let tri = device.create_index_buffer(3*2, Usage::None, Format::Index16, Pool::Default, ()).unwrap();
    /// let ib : Option<IndexBuffer> = device.get_indices().unwrap();
    /// assert!(ib.is_none(), "device has no index buffer by default");
    ///
    /// device.set_indices(Some(&tri));
    /// assert_eq!(tri.as_raw(), device.get_indices().unwrap().unwrap().as_raw());
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(Some([IndexBuffer]))     if an index buffer was bound
    /// *   Ok(None)                    if no index buffer was bound
    fn get_indices(&self) -> Result<Option<IndexBuffer>, MethodError> {
        let mut buffer = null_mut();
        let hr = unsafe { self.as_winapi().GetIndices(&mut buffer) };
        MethodError::check("IDirect3DDevice9::GetIndices", hr)?;
        Ok(unsafe { IndexBuffer::from_raw_opt(buffer) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlight)\]
    /// IDirect3DDevice9::GetLight
    ///
    /// Get the [Light] that was previously set for this device.
    ///
    /// This API mirrors [set_light] by accepting [u16], instead of the underlying [u32].
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   - if no light was previously set at `index`
    /// *   Ok([Light])
    ///
    /// ### Example
    ///
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
    fn get_light(&self, index: u16) -> Result<Light, MethodError> {
        self.get_light_32(index.into())
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlight)\]
    /// IDirect3DDevice9::GetLight
    ///
    /// This API appears sound despite the 32-bit indicies
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   - if no light was previously set at `index`
    /// *   Ok([Light])
    ///
    /// ### Example
    ///
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
    fn get_light_32(&self, index: u32) -> Result<Light, MethodError> {
        let mut light = Light::default();
        let hr = unsafe { self.as_winapi().GetLight(index, &mut *light) };
        MethodError::check("IDirect3DDevice9::GetLight", hr)?;
        Ok(light)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlightenable)
    /// IDirect3DDevice9::GetLightEnable
    ///
    /// Queries if the light is enabled.
    ///
    /// This API mirrors [light_enable] by accepting [u16], instead of the underlying [u32].
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the light was never explicitly previously enabled or disabled
    /// *   [D3DERR::INVALIDCALL]       The device is a pure device?
    /// *   Ok(`true`)                  The light is enabled
    /// *   Ok(`false`)                 The light is disabled
    ///
    /// ### Example
    ///
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
    fn get_light_enable(&self, index: u16) -> Result<bool, MethodError> {
        self.get_light_enable_32(index.into())
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getlightenable)\]
    /// IDirect3DDevice9::GetLightEnable
    ///
    /// This API appears sound despite the 32-bit indicies
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the light was never explicitly previously enabled or disabled
    /// *   [D3DERR::INVALIDCALL]       The device is a pure device?
    /// *   Ok(`true`)                  The light is enabled
    /// *   Ok(`false`)                 The light is disabled
    ///
    /// ### Example
    ///
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
    fn get_light_enable_32(&self, index: u32) -> Result<bool, MethodError> {
        let mut enable = 0;
        let hr = unsafe { self.as_winapi().GetLightEnable(index, &mut enable) };
        MethodError::check("IDirect3DDevice9::GetLightEnable", hr)?;
        Ok(enable != 0)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getmaterial)\]
    /// IDirect3DDevice9::GetMaterial
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok([Material])
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let material = device.get_material().unwrap();
    /// ```
    fn get_material(&self) -> Result<Material, MethodError> {
        let mut material = Material::default();
        let hr = unsafe { self.as_winapi().GetMaterial(&mut *material) };
        MethodError::check("IDirect3DDevice9::GetMaterial", hr)?;
        Ok(material)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getnpatchmode)\]
    /// IDirect3DDevice9::GetNPatchMode
    ///
    /// Gets the N-patch mode segments.
    ///
    /// Specifies the number of subdivision segments.
    /// If the number of segments is less than 1.0, N-patches are disabled.
    /// The default value is 0.0.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// assert_eq!(device.get_npatch_mode(), 0.0);
    /// ```
    fn get_npatch_mode(&self) -> f32 {
        unsafe { self.as_winapi().GetNPatchMode() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getnumberofswapchains)\]
    /// IDirect3DDevice9::GetNumberOfSwapChains
    ///
    /// Gets the number of implicit swap chains created for this device during [IDirect3D9Ext::create_device].
    ///
    /// (Does not include additional explicit swap chains created using [IDirect3DDevice9Ext::create_additional_swap_chain]?)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// assert_eq!(device.get_number_of_swap_chains(), 1);
    /// ```
    fn get_number_of_swap_chains(&self) -> u32 {
        unsafe { self.as_winapi().GetNumberOfSwapChains() }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpaletteentries)\]
    /// IDirect3DDevice9::GetPaletteEntries
    ///
    /// Retrieves palette entries.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// This function may crash if no palette was previously set!
    ///
    /// * Windows version:      `10.0.19041.630`
    /// * `d3d9.dll` version:   `10.0.19041.546`
    /// * Driver version:       `24.20.11026.2001`
    /// * Driver name:          `C:\Windows\System32\DriverStore\FileRepository\u0332836.inf_amd64_9f6b5ef5a1aed97e\B332771\aticfx64.dll,...`
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   "If the method fails"
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// // XXX: No palette set, this may crash!!!
    /// // let pal = unsafe { device.get_palette_entries(0) }.unwrap();
    ///
    /// device.set_palette_entries(0, &[Color::argb(0xFF112233); 256]).unwrap();
    ///
    /// let pal = unsafe { device.get_palette_entries(0) }.unwrap();
    /// assert_eq!(pal.len(), 256);
    /// assert_eq!(pal[  0], Color::argb(0xFF112233));
    /// assert_eq!(pal[255], Color::argb(0xFF112233));
    /// ```
    unsafe fn get_palette_entries(&self, palette_number: u32) -> Result<[Color; 256], MethodError> {
        // D3D9 uses PALETTEENTRYs but misuses the flags field.  D3DCOLORs are much better fits.
        let mut colors = [Color::argb(0); 256];
        let hr = unsafe { self.as_winapi().GetPaletteEntries(palette_number, colors.as_mut_ptr().cast()) };
        MethodError::check("IDirect3DDevice9::GetPaletteEntries", hr)?;
        Ok(colors)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getpixelshader)\]
    /// IDirect3DDevice9::GetPixelShader
    ///
    /// Gets the pixel shader currently bound to the device, if any.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the device was created with D3DCREATE_PUREDEVICE?
    /// *   Ok(Some([PixelShader]))     If a pixel shader was bound
    /// *   Ok(None)                    If no pixel shader was bound
    fn get_pixel_shader(&self) -> Result<PixelShader, MethodError> {
        let mut shader = null_mut();
        let hr = unsafe { self.as_winapi().GetPixelShader(&mut shader) };
        MethodError::check("IDirect3DDevice9::GetPixelShader", hr)?;
        Ok(unsafe { PixelShader::from_raw(shader) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrasterstatus)\]
    /// IDirect3DDevice9::GetRasterStatus
    ///
    /// Returns information describing the [RasterStatus] of the monitor on which the swap chain is presented.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let swap_chains = device.get_number_of_swap_chains();
    /// for i in 0 .. swap_chains {
    ///     dbg!(device.get_raster_status(i).unwrap());
    /// }
    /// # for i in (swap_chains .. 255).chain((8 .. 32).map(|pow| 1<<pow)) {
    /// #   assert_eq!(device.get_raster_status(i).err().unwrap().kind(), D3DERR::INVALIDCALL);
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
    ///
    /// ### Returns
    /// *   [D3DERR::INVALIDCALL]   If `swap_chain` is not a valid swap chain.
    /// *   Ok([RasterStatus])      If `swap_chain` is a valid swap chain.
    fn get_raster_status(&self, swap_chain: u32) -> Result<RasterStatus, MethodError> {
        let mut raster_status = RasterStatus::zeroed();
        let hr = unsafe { self.as_winapi().GetRasterStatus(swap_chain, &mut *raster_status) };
        MethodError::check("IDirect3DDevice9::GetRasterStatus", hr)?;
        Ok(raster_status)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertarget)\]
    /// IDirect3DDevice9::GetRenderTarget
    ///
    /// Typically, methods that return state will not work on a device that is created using D3DCREATE_PUREDEVICE. This method however, will work even on a pure device because it returns an interface.
    /// The device can now support multiple render targets. The number of render targets supported by a device is contained in the NumSimultaneousRTs member of [Caps]. See [Multiple Render Targets (Direct3D 9)].
    ///
    /// ### Returns
    ///
    /// * [D3DERR]::???             `render_target_index` > [Caps].NumSimultaneousRTs ?
    /// * Ok(Some([Surface]))       the render target bound to that index
    /// * Ok(None)                  no render target was bound to that index
    ///
    /// [Multiple Render Targets (Direct3D 9)]:         https://docs.microsoft.com/en-us/windows/win32/direct3d9/multiple-render-targets
    fn get_render_target(&self, render_target_index: u32) -> Result<Option<Surface>, MethodError> {
        // TODO: verify soundness before making pub
        let mut rt = null_mut();
        let hr = unsafe { self.as_winapi().GetRenderTarget(render_target_index, &mut rt) };
        if hr == D3DERR::NOTFOUND {
            Ok(None)
        } else {
            MethodError::check("IDirect3DDevice9::GetRenderTarget", hr)?;
            Ok(unsafe { Surface::from_raw_opt(rt) })
        }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getrendertargetdata)\]
    /// IDirect3DDevice9::GetRenderTargetData
    ///
    /// Copies the render-target data from device memory to system memory.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the source and destination formats do not match
    /// *   [D3DERR]::???           If the source and destination sizes do not match
    /// *   [D3DERR]::???           If the source is multisampled
    /// *   [D3DERR]::???           If the source is not a regular render target, or level thereof, created with [Pool::Default]
    /// *   [D3DERR]::???           If the destination is not an off-screen plain surface, or level of a texture created with [Pool::SystemMem]
    ///
    fn get_render_target_data(&self, render_target: &Surface, dest_surface: &Surface) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().GetRenderTargetData(render_target.as_raw(), dest_surface.as_raw()) };
        MethodError::check("IDirect3DDevice9::GetRenderTargetData", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getsoftwarevertexprocessing)\]
    /// IDirect3DDevice9::GetSoftwareVertexProcessing
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// let software_mode : bool = device.get_software_vertex_processing();
    /// let hardware_mode : bool = !software_mode;
    /// assert!(hardware_mode);
    /// ```
    ///
    /// ### Returns
    /// *   `true`      if in software processing mode
    /// *   `false`     if in hardware processing mode
    fn get_software_vertex_processing(&self) -> bool {
        unsafe { self.as_winapi().GetSoftwareVertexProcessing() != 0 }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getstreamsource)\]
    /// IDirect3DDevice9::GetStreamSource
    ///
    /// Retrieves a vertex buffer bound to the specified data stream.
    ///
    /// ### Example
    ///
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
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(Some([VertexBuffer]), `offset_in_bytes`, `stride`)
    /// *   Ok(`(None, 0, 0)`)
    fn get_stream_source(&self, stream_number: u32) -> Result<(Option<VertexBuffer>, u32, u32), MethodError> {
        let mut buffer = null_mut();
        let mut offset = 0;
        let mut stride = 0;
        let hr = unsafe { self.as_winapi().GetStreamSource(stream_number, &mut buffer, &mut offset, &mut stride) };
        MethodError::check("IDirect3DDevice9::GetStreamSource", hr)?;
        let buffer = unsafe { VertexBuffer::from_raw_opt(buffer) };
        Ok((buffer, offset, stride))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getstreamsourcefreq)\]
    /// IDirect3DDevice9::GetStreamSourceFreq
    ///
    /// Gets the stream source frequency divider value.
    ///
    /// ### Example
    ///
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
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([StreamSource])
    fn get_stream_source_freq(&self, stream_number: u32) -> Result<StreamSource, MethodError> {
        let mut freq = 0;
        let hr = unsafe { self.as_winapi().GetStreamSourceFreq(stream_number, &mut freq) };
        MethodError::check("IDirect3DDevice9::GetStreamSourceFreq", hr)?;
        Ok(StreamSource::from_unchecked(freq))
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getswapchain)\]
    /// IDirect3DDevice9::GetSwapChain
    ///
    /// Gets a pointer to a swap chain.
    ///
    /// ### Example
    /// ```rust
    /// # use dev::d3d9::*; let device = device_pure();
    /// for swap_chain in 0 .. device.get_number_of_swap_chains() {
    ///     let swap_chain = device.get_swap_chain(swap_chain).unwrap();
    /// }
    /// # for swap_chain in (device.get_number_of_swap_chains() .. 255).chain((8..32).map(|pow| 1<<pow)) {
    /// #   assert_eq!(device.get_swap_chain(swap_chain).err().unwrap().kind(), D3DERR::INVALIDCALL);
    /// # }
    /// ```
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok([SwapChain])
    fn get_swap_chain(&self, swap_chain: u32) -> Result<SwapChain, MethodError> {
        let mut sc = null_mut();
        let hr = unsafe { self.as_winapi().GetSwapChain(swap_chain, &mut sc) };
        MethodError::check("IDirect3DDevice9::GetSwapChain", hr)?;
        let swap_chain = unsafe { SwapChain::from_raw(sc) };
        Ok(swap_chain)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-gettexture)\]
    /// IDirect3DDevice9::GetTexture
    ///
    /// Retrieves a texture assigned to a stage for a device.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   This function will crash (or worse!) if `set_texture` was never called for `stage`!
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the device is a pure device?
    /// *   Ok(Some([BaseTexture]))     If a texture was bound to that stage
    /// *   Ok(None)                    If no texture was bound to that stage
    ///
    /// ### Example
    ///
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
    unsafe fn get_texture(&self, stage: u32) -> Result<Option<BaseTexture>, MethodError> {
        let mut texture = null_mut();
        let hr = unsafe { self.as_winapi().GetTexture(stage, &mut texture) };
        MethodError::check("IDirect3DDevice9::GetTexture", hr)?;
        Ok(unsafe { BaseTexture::from_raw_opt(texture) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getvertexshader)\]
    /// IDirect3DDevice9::GetVertexShader
    ///
    /// Gets the vertex shader currently bound to the device, if any.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If the device was created with D3DCREATE_PUREDEVICE?
    /// *   Ok(Some([VertexShader]))    If a vertex shader was bound
    /// *   Ok(None)                    If no vertex shader was bound
    fn get_vertex_shader(&self) -> Result<VertexShader, MethodError> {
        let mut shader = null_mut();
        let hr = unsafe { self.as_winapi().GetVertexShader(&mut shader) };
        MethodError::check("IDirect3DDevice9::GetVertexShader", hr)?;
        Ok(unsafe { VertexShader::from_raw(shader) })
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-getviewport)\]
    /// IDirect3DDevice9::GetViewport
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If this is a pure device?
    /// *   [D3DERR::INVALIDCALL]   If the viewport is invalid
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let viewport : Viewport = device.get_viewport().unwrap();
    /// ```
    fn get_viewport(&self) -> Result<Viewport, MethodError> {
        let mut viewport = Viewport::default();
        let hr = unsafe { self.as_winapi().GetViewport(&mut *viewport) };
        MethodError::check("IDirect3DDevice9::GetViewport", hr)?;
        Ok(viewport)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-lightenable)\]
    /// IDirect3DDevice9::LightEnable
    ///
    /// Enables or disables a set of lighting parameters within a device.
    ///
    /// For soundness, this limits `index` to [u16], instead of accepting [u32] like the underlying API does.
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.light_enable(0,  true).unwrap(); // Ok
    /// device.light_enable(1,  true).unwrap(); // Ok
    /// device.light_enable(!0, true).unwrap(); // Ok (16-bit)
    /// ```
    fn light_enable(&self, index: u16, enable: bool) -> Result<(), MethodError> {
        // Safe: `index` max == `u16::MAX` == `0xFFFF`, well bellow when GArrayT starts buffer overflowing (`0x1000_0000 - 8`)
        unsafe { self.light_enable_32_unchecked(index.into(), enable) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-lightenable)\]
    /// IDirect3DDevice9::LightEnable
    ///
    /// Enables or disables a set of lighting parameters within a device.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   This will buffer overflow and crash (or worse!) if `index` >= `0x1000_0000 - 8` on some systems!
    /// *   Other, smaller sizes may also crash on other system I haven't tested.
    /// *   Prefer [light_enable] (limits the index to [u16], and is thus 100% infalliable.)
    ///
    /// [light_enable]:                Self::light_enable
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// unsafe {
    ///     device.light_enable_32_unchecked(0, true).unwrap();
    ///     device.light_enable_32_unchecked(0, false).unwrap();
    ///     // device.light_enable_32_unchecked(!0, true).unwrap();
    ///     // XXX: Buffer overflow, crash, or worse!
    /// }
    /// ```
    unsafe fn light_enable_32_unchecked(&self, index: u32, enable: bool) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().LightEnable(index, enable.into()) };
        MethodError::check("IDirect3DDevice9::LightEnable", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-present)\]
    /// IDirect3DDevice9::Present
    ///
    /// Presents the contents of the next buffer in the sequence of back buffers owned by the device.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   It's likely unsound to use an invalid, non-null `hwnd`
    /// *   It's likely unsound to use a null `hwnd` if the original `presentation_parameters.hDeviceWindow` is an invalid, non-null HWND
    /// *   Out of bounds rects might also be an issue IDK?
    ///
    /// ### Arguments
    ///
    /// *   `source_rect`           - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire source surface is presented.)
    /// *   `dest_rect`             - "Must be `..`" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be `..` even then (the entire client area is filled.)
    /// *   `dest_window_override`  - The destination window to render to.  If null / `()`, the runtime uses the `hDeviceWindow` member of D3DPRESENT_PARAMETERS for the presentation.
    /// *   `dirty_region`          - "Must be [None]" unless the [SwapChain] was created with [SwapEffect::Copy].  Can still be [None] even then (the entire region will be considered dirty.)  The implementation is free to copy more than the exact dirty region.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::DEVICEREMOVED]     When you least expect it
    /// *   [D3DERR::DEVICELOST]        When switching into/out-of fullscreen, or when invoking `C:\Windows\System32\DXCap.exe -forcetdr`
    /// *   [D3DERR::INVALIDCALL]       If called within a [IDirect3DDevice9Ext::begin_scene] .. [IDirect3DDevice9Ext::end_scene] section, if the render target is the current render target.
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use std::ptr::null_mut;   let hwnd = null_mut();
    /// # use dev::d3d9::*;               let device = device_test();
    /// // Present the entire back buffer (should work with all swap chains, probably:)
    /// device.present(.., .., (), None).unwrap();
    /// // TODO: Handle D3DERR::DEVICEREMOVED
    ///
    /// // Or, with a SwapEffect::Copy swap chain, this should succeed (might succeed by simply ignoring the args, even for other SwapEffect s:)
    /// let hwnd = unsafe { SafeHWND::assert(&hwnd) };
    /// match device.present((0,0)..(100,100), Rect::from((0,0)..(100,100)), hwnd, None).map_err(|e| e.kind()) {
    ///     Ok(()) => {}, // Huzzah!
    ///     Err(D3DERR::DEVICEREMOVED   ) => { /* oooh, a removable GPU?  Nifty!  Might switch to the laptop's built-in Device (might have different caps!) */ },
    ///     Err(D3DERR::DEVICELOST      ) => { /* switching fullscreen modes? GPU driver crashed? might prompt the user before recreating the device to avoid hang loops */ },
    ///     Err(D3DERR::DEVICEHUNG      ) => { /* ...is it your fault? crazy shaders? might prompt the user before recreating the device to avoid hang loops */ },
    ///     Err(other                   ) => panic!("oh no something bad happened: {}", other),
    /// }
    /// ```
    fn present<'r>(&self, source_rect: impl IntoRectOrFull, dest_rect: impl IntoRectOrFull, dest_window_override: impl AsHWND, dirty_region: impl Into<Option<&'r RgnData>>) -> Result<(), MethodError> {
        let source_rect     = source_rect.into_rect();
        let dest_rect       = dest_rect.into_rect();
        let hwnd            = dest_window_override.as_hwnd();
        let dirty_region    = dirty_region.into();

        let source_rect     = source_rect   .map_or(null(), |r| &*r).cast();
        let dest_rect       = dest_rect     .map_or(null(), |r| &*r).cast();
        let dirty_region    = match dirty_region {
            None => null::<RGNDATA>(),
            Some(dr) => {
                if dr.rdh.dwSize as usize   != std::mem::size_of::<RGNDATAHEADER>() { return Err(MethodError("IDirect3DDevice9Ext::present", THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.iType             != RDH_RECTANGLES                       { return Err(MethodError("IDirect3DDevice9Ext::present", THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nCount as usize   > dr.buffer.len()                       { return Err(MethodError("IDirect3DDevice9Ext::present", THINERR::INVALID_STRUCT_FIELD)); }
                if dr.rdh.nRgnSize as usize > std::mem::size_of_val(dr)             { return Err(MethodError("IDirect3DDevice9Ext::present", THINERR::INVALID_STRUCT_FIELD)); }
                let dr : *const RgnData = dr;
                dr.cast()
            },
        };

        let hr = unsafe { self.as_winapi().Present(source_rect, dest_rect, hwnd, dirty_region) };
        MethodError::check("IDirect3DDevice9::Present", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-reset)\]
    /// IDirect3DDevice9::Reset
    ///
    /// Resets the type, size, and format of the swap chain.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   `presentation_parameters.hDeviceWindow` must be null or a valid window
    /// *   `presentation_parameters.*` in general probably needs certain "valid" values
    ///
    /// ### Returns
    ///
    /// * [D3DERR::DEVICELOST]
    /// * [D3DERR::DEVICEREMOVED]
    /// * [D3DERR::DRIVERINTERNALERROR]
    /// * [D3DERR::OUTOFVIDEOMEMORY]
    /// * Ok(())
    unsafe fn reset(&self, presentation_parameters: &mut D3DPRESENT_PARAMETERS) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().Reset(presentation_parameters) };
        MethodError::check("IDirect3DDevice9::Reset", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setdepthstencilsurface)\]
    /// IDirect3DDevice9::SetDepthStencilSurface
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]         if `depth_stencil_surface == Some(surface)` and `surface.usage() != Usage::DepthStencil`
    /// * `Ok(())`                      if the depth stencil was successfully (un)bound
    fn set_depth_stencil_surface(&self, depth_stencil_surface: Option<&Surface>) -> Result<(), MethodError> {
        let ds = depth_stencil_surface.map_or(null_mut(), |ds| ds.as_raw());
        let hr = unsafe { self.as_winapi().SetDepthStencilSurface(ds) };
        MethodError::check("IDirect3DDevice9::SetDepthStencilSurface", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setfvf)\]
    /// IDirect3DDevice9::SetFVF
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       "If the method fails (impossible via thindx?)
    /// *   Ok(())
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.set_fvf(FVF::None).unwrap();
    /// device.set_fvf(FVF::XYZ).unwrap();
    /// ```
    fn set_fvf(&self, fvf: impl Into<FVF>) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().SetFVF(fvf.into().into()) };
        MethodError::check("IDirect3DDevice9::SetFVF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setgammaramp)\]
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
    /// ### Returns
    ///
    /// *   `()`
    ///
    /// ### Example
    ///
    /// ```rust,no_run
    /// # use dev::d3d9::*; let device = device_test();
    /// let ramp = device.get_gamma_ramp(0);
    /// // ...modify ramp?..
    /// device.set_gamma_ramp(0, SGR::NoCalibration, &ramp);
    /// ```
    ///
    /// [Gamma (Direct3D 9)]:           https://docs.microsoft.com/en-us/windows/desktop/direct3d9/gamma
    fn set_gamma_ramp(&self, swap_chain: u32, flags: impl Into<SGR>, ramp: &D3DGAMMARAMP) {
        let _nohr : () = unsafe { self.as_winapi().SetGammaRamp(swap_chain, flags.into().into(), ramp) };
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setindices)\]
    /// IDirect3DDevice9::SetIndices
    ///
    /// Sets index data.
    ///
    /// ### Example
    ///
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
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       (perhaps only on an invalid [IndexBuffer] that thindx's API prevents?)
    /// *   [THINERR::DEVICE_MISMATCH]   If the [IndexBuffer] was created with a different [Device].
    /// *   Ok(())
    fn set_indices<'ib>(&self, index_data: impl Into<Option<&'ib IndexBuffer>>) -> Result<(), MethodError> {
        let ptr = match index_data.into() {
            None => null_mut(),
            Some(ib) => { ib.check_compatible_with(self, "Device::set_indices")?; ib.as_raw() }
            // Mixing index buffers between devices crashes on my computer, compatability check 100% necessary!
        };
        let hr = unsafe { self.as_winapi().SetIndices(ptr) };
        MethodError::check("IDirect3DDevice9::SetIndices", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setlight)\]
    /// IDirect3DDevice9::SetLight
    ///
    /// Assigns a set of lighting properties for this device.
    ///
    /// For soundness, this limits `index` to [u16], instead of accepting [u32] like the underlying API does.
    ///
    /// ### Returns
    ///
    /// *   <span class="inaccurate">[D3DERR::INVALIDCALL]</span>
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let mut light = Light::default();
    /// light.Type = LightType::Point.into();
    /// // ...
    /// device.set_light(0,  light).unwrap(); // Ok
    /// device.set_light(1,  light).unwrap(); // Ok
    /// device.set_light(!0, light).unwrap(); // Ok (16-bit)
    /// ```
    fn set_light(&self, index: u16, light: impl Into<Light>) -> Result<(), MethodError> {
        // Safe: `index` max == `u16::MAX` == `0xFFFF`, well bellow when GArrayT starts buffer overflowing (`0x1000_0000 - 8`)
        unsafe { self.set_light_32_unchecked(index.into(), light.into()) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setlight)\]
    /// IDirect3DDevice9::SetLight
    ///
    /// Assigns a set of lighting properties for this device.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
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
    ///
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
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
    unsafe fn set_light_32_unchecked(&self, index: u32, light: impl Into<Light>) -> Result<(), MethodError> {
        let light = light.into();
        let hr = unsafe { self.as_winapi().SetLight(index, &*light) };
        MethodError::check("IDirect3DDevice9::SetLight", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setmaterial)\]
    /// IDirect3DDevice9::SetMaterial
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the material is invalid
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let material = Material {
    ///     diffuse: ColorValue::default(),
    ///     .. Material::default()
    /// };
    /// device.set_material(material).unwrap();
    /// ```
    fn set_material(&self, material: impl Into<Material>) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().SetMaterial(&*material.into()) };
        MethodError::check("IDirect3DDevice9::SetMaterial", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-getnpatchmode)\]
    /// IDirect3DDevice9::SetNPatchMode
    ///
    /// Specifies the number of subdivision segments.
    /// If the number of segments is less than 1.0, N-patches are disabled.
    /// The default value is 0.0.
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.set_npatch_mode(0.0).unwrap();
    /// device.set_npatch_mode(1.0).unwrap();
    /// device.set_npatch_mode(9001.0).unwrap();
    /// ```
    fn set_npatch_mode(&self, mode: f32) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().SetNPatchMode(mode) };
        MethodError::check("IDirect3DDevice9::SetNPatchMode", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpaletteentries)\]
    /// IDirect3DDevice9::SetPaletteEntries
    ///
    /// Sets palette entries.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If D3DPTEXTURECAPS_ALPHAPALETTE is not set and any entries has an alpha other than 1.0.
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// let pal = [Color::argb(0xFF112233); 256];
    /// device.set_palette_entries(0, &pal).unwrap();
    /// ```
    fn set_palette_entries(&self, palette_number: u32, entries: &[Color; 256]) -> Result<(), MethodError> {
        // D3D9 uses PALETTEENTRYs but misuses the flags field.  D3DCOLORs are much better fits.
        let hr = unsafe { self.as_winapi().SetPaletteEntries(palette_number, entries.as_ptr().cast()) };
        MethodError::check("IDirect3DDevice9::SetPaletteEntries", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshader)\]
    /// IDirect3DDevice9::SetPixelShader
    ///
    /// Sets the pixel shader to render with.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If `pixel_shader` was created by another device?
    /// *   Ok(())
    fn set_pixel_shader<'sh>(&self, pixel_shader: impl Into<Option<&'sh PixelShader>>) -> Result<(), MethodError> {
        let pixel_shader = pixel_shader.into();
        let ps = pixel_shader.map_or(null_mut(), |ps| ps.as_raw());
        let hr = unsafe { self.as_winapi().SetPixelShader(ps) };
        MethodError::check("IDirect3DDevice9::SetPixelShader", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantb)\]
    /// IDirect3DDevice9::SetPixelShaderConstantB
    ///
    /// Sets boolean shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers?
    /// *   Ok(())
    fn set_pixel_shader_constant_b(&self, start_register: u32, constant_data: &[bool32]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_b", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetPixelShaderConstantB(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantB", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantf)\]
    /// IDirect3DDevice9::SetPixelShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    fn set_pixel_shader_constant_f(&self, start_register: u32, constant_data: &[f32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_pixel_shader_constant_f", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_f", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetPixelShaderConstantF(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstantf)\]
    /// IDirect3DDevice9::SetPixelShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    fn set_pixel_shader_constant_fv(&self, start_register: u32, constant_data: &[[f32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_fv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetPixelShaderConstantF(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstanti)\]
    /// IDirect3DDevice9::SetPixelShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    fn set_pixel_shader_constant_i(&self, start_register: u32, constant_data: &[i32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_pixel_shader_constant_i", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_i", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetPixelShaderConstantI(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantI", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setpixelshaderconstanti)\]
    /// IDirect3DDevice9::SetPixelShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    fn set_pixel_shader_constant_iv(&self, start_register: u32, constant_data: &[[i32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_pixel_shader_constant_iv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetPixelShaderConstantI(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetPixelShaderConstantI", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setrenderstate)\]
    /// IDirect3DDevice9::SetRenderState
    ///
    /// Sets a single device render-state parameter
    ///
    /// ### Examples
    ///
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
    ///
    /// ### Returns
    ///
    /// *   Ok(())              no matter what invalid parameters are used?
    fn set_render_state_untyped(&self, state: impl Into<d3d9::RenderStateType>, value: impl Into<u32>) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().SetRenderState(state.into().into(), value.into()) };
        MethodError::check("IDirect3DDevice9::SetRenderState", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setrendertarget)\]
    /// IDirect3DDevice9::SetRenderTarget
    ///
    /// ### Returns
    ///
    /// * [D3DERR::INVALIDCALL]         if `render_target_index == 0` and `render_target == None`
    /// * [D3DERR::INVALIDCALL]         if `render_target == Some(surface)` and `surface.usage() != Usage::RenderTarget`
    /// * `Ok(())`                      if the render target was successfully bound
    fn set_render_target(&self, render_target_index: u32, render_target: Option<&Surface>) -> Result<(), MethodError> {
        let rt = render_target.map_or(null_mut(), |rt| rt.as_raw());
        let hr = unsafe { self.as_winapi().SetRenderTarget(render_target_index, rt) };
        MethodError::check("IDirect3DDevice9::SetRenderTarget", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsamplerstate)\]
    /// IDirect3DDevice9::SetSamplerState
    ///
    /// Prefer [set_sampler_state](Self::set_sampler_state)
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// Known undefined behavior:
    /// *   This function may crash with out-of-bounds sampler state types! (possibly only if fed nonzero values?)
    ///
    /// Possible undefined behavior on older Runtimes/Drivers?
    /// *   This function may crash with out-of-bounds samplers?
    /// *   This function may crash with out-of-bounds sampler state type values?
    ///
    /// ### Example
    ///
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
    ///
    /// ### Returns
    ///
    /// *   Ok(())              no matter what invalid parameters are used?
    unsafe fn set_sampler_state_unchecked(&self, sampler: u32, ty: SamplerStateType, value: impl Into<u32>) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().SetSamplerState(sampler, ty.into(), value.into()) };
        MethodError::check("IDirect3DDevice9::SetSamplerState", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setsamplerstate)\]
    /// IDirect3DDevice9::SetSamplerState
    ///
    /// ### Example
    ///
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
    ///
    /// ### Returns
    ///
    /// *   Ok(())              no matter what invalid parameters are used?
    fn set_sampler_state(&self, sampler: u32, ssv: impl Into<SamplerStateValue>) -> Result<(), MethodError> {
        let (ty, value) = ssv.into().ty_value();
        unsafe { self.set_sampler_state_unchecked(sampler, ty, value) }
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsource)\]
    /// IDirect3DDevice9::SetStreamSource
    ///
    /// Binds a vertex buffer to a device data stream. For more information, see [Setting the Stream Source (Direct3D 9)].
    ///
    /// [Setting the Stream Source (Direct3D 9)]:       https://docs.microsoft.com/en-us/windows/desktop/direct3d9/setting-the-stream-source
    ///
    /// ### Example
    ///
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
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       if the [VertexBuffer] belongs to another device?
    /// *   [THINERR::DEVICE_MISMATCH]   If the [IndexBuffer] was created with a different [Device].
    /// *   Ok(`()`)
    fn set_stream_source<'b>(&self, stream_number: u32, stream_data: impl Into<Option<&'b VertexBuffer>>, offset_in_bytes: u32, stride: u32) -> Result<(), MethodError> {
        let stream_data = match stream_data.into() {
            None => null_mut(),
            Some(sd) => { sd.check_compatible_with(self, "Device::set_stream_source")?; sd.as_raw() },
            // XXX: Might be able to skip check_compatible_with with software vertex buffers?  Those might be safe?
            // They don't seem to crash for me, but I'm erring on the side of caution for now.
        };
        let hr = unsafe { self.as_winapi().SetStreamSource(stream_number, stream_data, offset_in_bytes, stride) };
        MethodError::check("IDirect3DDevice9::SetStreamSource", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setstreamsourcefreq)\]
    /// IDirect3DDevice9::SetStreamSourceFreq
    ///
    /// Sets the stream source frequency divider value. This may be used to draw several instances of geometry.
    ///
    /// ### Example
    ///
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
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(`()`)
    fn set_stream_source_freq(&self, stream_number: u32, setting: impl Into<StreamSource>) -> Result<(), MethodError> {
        let setting = setting.into().into();
        let hr = unsafe { self.as_winapi().SetStreamSourceFreq(stream_number, setting) };
        MethodError::check("IDirect3DDevice9::SetStreamSourceFreq", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-settexture)\]
    /// IDirect3DDevice9::SetTexture
    ///
    /// Assigns a texture to a stage for a device.
    ///
    /// ### ⚠️ Safety ⚠️
    ///
    /// *   This function will crash (or worse!) if `stage` is too large (> `MaxSimultaneousTextures`?)
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]
    /// *   Ok(())
    ///
    /// ### Example
    ///
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
    unsafe fn set_texture<'t>(&self, stage: u32, texture: impl Into<Option<&'t BaseTexture>>) -> Result<(), MethodError> {
        // TODO: make a more convenient trait for texture binding
        let texture = texture.into();
        let texture = texture.map_or(null_mut(), |t| t.as_raw());
        let hr = unsafe { self.as_winapi().SetTexture(stage, texture) };
        MethodError::check("IDirect3DDevice9::SetTexture", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-settransform)\]
    /// IDirect3DDevice9::SetTransform
    ///
    /// Sets a single device transform
    ///
    /// ### Examples
    ///
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
    ///
    /// ### Returns
    ///
    /// *   Ok(())
    fn set_transform(&self, ts: impl Into<TransformStateType>, matrix: impl Into<Matrix>) -> Result<(), MethodError> {
        let hr = unsafe { self.as_winapi().SetTransform(ts.into().into(), &matrix.into().into()) };
        MethodError::check("IDirect3DDevice9::SetTransform", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9/nf-d3d9-idirect3ddevice9-setvertexdeclaration)\]
    /// IDirect3DDevice9::SetVertexDeclaration
    ///
    /// Describes the layout of vertexes for rendering.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If `decl` was created by another device?
    /// *   Ok(())
    fn set_vertex_declaration<'d>(&self, decl: impl Into<Option<&'d VertexDeclaration>>) -> Result<(), MethodError> {
        let decl = decl.into();
        let decl = decl.map_or(null_mut(), |d| d.as_raw());
        let hr = unsafe { self.as_winapi().SetVertexDeclaration(decl) };
        MethodError::check("IDirect3DDevice9::SetVertexDeclaration", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshader)\]
    /// IDirect3DDevice9::SetVertexShader
    ///
    /// Sets the vertex shader to render with.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If `vertex_shader` was created by another device?
    /// *   Ok(())
    fn set_vertex_shader<'sh>(&self, vertex_shader: impl Into<Option<&'sh VertexShader>>) -> Result<(), MethodError> {
        let vertex_shader = vertex_shader.into();
        let ps = vertex_shader.map_or(null_mut(), |ps| ps.as_raw());
        let hr = unsafe { self.as_winapi().SetVertexShader(ps) };
        MethodError::check("IDirect3DDevice9::SetVertexShader", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantb)\]
    /// IDirect3DDevice9::SetVertexShaderConstantB
    ///
    /// Sets boolean shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max boolean registers?
    /// *   Ok(())
    fn set_vertex_shader_constant_b(&self, start_register: u32, constant_data: &[bool32]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_b", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetVertexShaderConstantB(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantB", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantf)\]
    /// IDirect3DDevice9::SetVertexShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    fn set_vertex_shader_constant_f(&self, start_register: u32, constant_data: &[f32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_vertex_shader_constant_f", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_f", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetVertexShaderConstantF(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstantf)\]
    /// IDirect3DDevice9::SetVertexShaderConstantF
    ///
    /// Sets floating-point shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    fn set_vertex_shader_constant_fv(&self, start_register: u32, constant_data: &[[f32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_fv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetVertexShaderConstantF(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantF", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstanti)\]
    /// IDirect3DDevice9::SetVertexShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() / 4` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len() % 4` != `0` (not a multiple of vectors)
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()/4` > max floating point vector registers?
    /// *   Ok(())
    fn set_vertex_shader_constant_i(&self, start_register: u32, constant_data: &[i32]) -> Result<(), MethodError> {
        let n = constant_data.len();
        if (n % 4) != 0 { return Err(MethodError("Device::set_vertex_shader_constant_i", D3DERR::INVALIDCALL)); }
        let n : u32 = (n/4).try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_i", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetVertexShaderConstantI(start_register, constant_data.as_ptr(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantI", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setvertexshaderconstanti)\]
    /// IDirect3DDevice9::SetVertexShaderConstantI
    ///
    /// Sets integer shader constants.
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]       If `constant_data.len()` > `u32::MAX`
    /// *   [D3DERR::INVALIDCALL]       If `start_register + constant_data.len()` > max floating point vector registers?
    /// *   Ok(())
    fn set_vertex_shader_constant_iv(&self, start_register: u32, constant_data: &[[i32; 4]]) -> Result<(), MethodError> {
        let n : u32 = constant_data.len().try_into().map_err(|_| MethodError("Device::set_vertex_shader_constant_iv", D3DERR::INVALIDCALL))?;
        let hr = unsafe { self.as_winapi().SetVertexShaderConstantI(start_register, constant_data.as_ptr().cast(), n) };
        MethodError::check("IDirect3DDevice9::SetVertexShaderConstantI", hr)
    }

    /// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/d3d9helper/nf-d3d9helper-idirect3ddevice9-setviewport)\]
    /// IDirect3DDevice9::SetViewport
    ///
    /// ### Returns
    ///
    /// *   [D3DERR::INVALIDCALL]   If the viewport is invalid / describes a region that cannot exist within the render target surface.
    /// *   Ok(`()`)
    ///
    /// ### Example
    ///
    /// ```rust
    /// # use dev::d3d9::*; let device = device_test();
    /// device.set_viewport(Viewport{ x: 0, y: 0, width: 100, height: 100, min_z: 0.0, max_z: 1.0 }).unwrap();
    /// ```
    fn set_viewport(&self, viewport: impl Into<Viewport>) -> Result<(), MethodError> {
        let viewport = viewport.into();
        let hr = unsafe { self.as_winapi().SetViewport(&*viewport) };
        MethodError::check("IDirect3DDevice9::SetViewport", hr)
    }
}

impl<T: AsSafe<IDirect3DDevice9>> IDirect3DDevice9Ext for T {}



/// \[[docs.microsoft.com](https://docs.microsoft.com/en-us/windows/win32/api/wingdi/ns-wingdi-rgndata)\]
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
        let device = device_test_pp(false, |pp, _| pp.SwapEffect = SwapEffect::Copy.into()).unwrap();
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
