/*******************************************************************************************
 *
 *   raylib [shaders] example - normalmap rendering
 *
 *   Example complexity rating: [★★★★] 4/4
 *
 *   NOTE: This example requires raylib OpenGL 3.3 or ES2 versions for shaders
 * support, OpenGL 1.1 does not support shaders, recompile raylib to OpenGL 3.3
 * version
 *
 *   Example originally created with raylib 5.6, last time updated with
 * raylib 5.6
 *
 *   Example contributed by Jeremy Montgomery (@Sir_Irk) and reviewed by Ramon
 * Santamaria (@raysan5)
 *
 *   Example licensed under an unmodified zlib/libpng license, which is an
 * OSI-certified, BSD-like license that allows static linking with closed source
 * software
 *
 *   Copyright (c) 2025 Jeremy Montgomery (@Sir_Irk) and Ramon Santamaria
 * (@raysan5)
 *
 ********************************************************************************************/

#include "raylib.h"

#include "raymath.h"

#define GLSL_VERSION 330

//------------------------------------------------------------------------------------
// Program main entry point
//------------------------------------------------------------------------------------
int main(void) {
  // Initialization
  //--------------------------------------------------------------------------------------
  const int screenWidth = 800;
  const int screenHeight = 450;

  SetConfigFlags(FLAG_MSAA_4X_HINT);
  InitWindow(screenWidth, screenHeight,
             "raylib [shaders] example - normalmap rendering");

  Camera camera = {0};
  camera.position = (Vector3){-10.0f, 10.0f, -10.0f};
  camera.target = (Vector3){.0f, .0f, .0f};
  camera.up = (Vector3){0.0f, 1.0f, 0.0f};
  camera.fovy = 2.5f;
  camera.projection = CAMERA_ORTHOGRAPHIC;

  // Load basic normal map lighting shader
  Shader shader =
      LoadShader(TextFormat("resources/shaders/glsl%i/normalmap_directional.vs",
                            GLSL_VERSION),
                 TextFormat("resources/shaders/glsl%i/normalmap_directional.fs",
                            GLSL_VERSION));

  // Get some required shader locations
  shader.locs[SHADER_LOC_MAP_NORMAL] = GetShaderLocation(shader, "normalMap");
  shader.locs[SHADER_LOC_VECTOR_VIEW] = GetShaderLocation(shader, "viewPos");

  // NOTE: "matModel" location name is automatically assigned on shader loading,
  // no need to get the location again if using that uniform name
  // shader.locs[SHADER_LOC_MATRIX_MODEL] = GetShaderLocation(shader,
  // "matModel");

  // This example uses just 1 point light
  Vector3 lightDir = {3.0f, -3., -5.0f};
  int lightDirectionLoc = GetShaderLocation(shader, "lightDirection");

  // Load a plane model that has proper normals and tangents
  Model plane = LoadModel("resources/models/plane.glb");

  // Set the plane model's shader and texture maps
  plane.materials[0].shader = shader;
  plane.materials[0].maps[MATERIAL_MAP_DIFFUSE].texture =
      LoadTexture("resources/tiles_diffuse.png");
  plane.materials[0].maps[MATERIAL_MAP_NORMAL].texture =
      LoadTexture("resources/tiles_normal.png");

  Mesh cubeMesh = GenMeshCube(0.5, 0.5, 0.5);
  GenMeshTangents(&cubeMesh);
  Model cube = LoadModelFromMesh(cubeMesh);

  cube.materials[0].shader = shader;
  cube.materials[0].maps[MATERIAL_MAP_DIFFUSE].texture =
      LoadTexture("resources/tiles_diffuse_single.png");
  cube.materials[0].maps[MATERIAL_MAP_NORMAL].texture =
      LoadTexture("resources/tiles_normal_single.png");

  // Generate Mipmaps and use TRILINEAR filtering to help with texture aliasing
  GenTextureMipmaps(&plane.materials[0].maps[MATERIAL_MAP_DIFFUSE].texture);
  GenTextureMipmaps(&plane.materials[0].maps[MATERIAL_MAP_NORMAL].texture);

  SetTextureFilter(plane.materials[0].maps[MATERIAL_MAP_DIFFUSE].texture,
                   TEXTURE_FILTER_TRILINEAR);
  SetTextureFilter(plane.materials[0].maps[MATERIAL_MAP_NORMAL].texture,
                   TEXTURE_FILTER_TRILINEAR);

  GenTextureMipmaps(&cube.materials[0].maps[MATERIAL_MAP_DIFFUSE].texture);
  GenTextureMipmaps(&cube.materials[0].maps[MATERIAL_MAP_NORMAL].texture);

  SetTextureFilter(cube.materials[0].maps[MATERIAL_MAP_DIFFUSE].texture,
                   TEXTURE_FILTER_TRILINEAR);
  SetTextureFilter(cube.materials[0].maps[MATERIAL_MAP_NORMAL].texture,
                   TEXTURE_FILTER_TRILINEAR);

  // Specular exponent AKA shininess of the material
  float specularExponent = 3.6f;
  int specularExponentLoc = GetShaderLocation(shader, "specularExponent");

  // Allow toggling the normal map on and off for comparison purposes
  int useNormalMap = 1;
  int useNormalMapLoc = GetShaderLocation(shader, "useNormalMap");

  SetTargetFPS(60); // Set our game to run at 60 frames-per-second
  //--------------------------------------------------------------------------------------

  // Main game loop
  while (!WindowShouldClose()) // Detect window close button or ESC key
  {
    // Update
    //----------------------------------------------------------------------------------
    // Increase/Decrease the specular exponent(shininess)
    if (IsKeyDown(KEY_UP))
      specularExponent =
          Clamp(specularExponent + 40.0f * GetFrameTime(), 2.0f, 128.0f);
    if (IsKeyDown(KEY_DOWN))
      specularExponent =
          Clamp(specularExponent - 40.0f * GetFrameTime(), 2.0f, 128.0f);

    // Toggle normal map on and off
    if (IsKeyPressed(KEY_N))
      useNormalMap = !useNormalMap;

    // Spin plane model at a constant rate
    plane.transform = MatrixRotateY(PI);

    // Update shader values
    float lightDirection[3] = {lightDir.x, lightDir.y, lightDir.z};
    SetShaderValue(shader, lightDirectionLoc, lightDirection,
                   SHADER_UNIFORM_VEC3);

    float camPos[3] = {camera.position.x, camera.position.y, camera.position.z};
    SetShaderValue(shader, shader.locs[SHADER_LOC_VECTOR_VIEW], camPos,
                   SHADER_UNIFORM_VEC3);

    SetShaderValue(shader, specularExponentLoc, &specularExponent,
                   SHADER_UNIFORM_FLOAT);

    //--------------------------------------------------------------------------------------

    // Draw
    //----------------------------------------------------------------------------------
    BeginDrawing();

    ClearBackground(RAYWHITE);

    BeginMode3D(camera);

    BeginShaderMode(shader);

    SetShaderValue(shader, useNormalMapLoc, &useNormalMap, SHADER_UNIFORM_INT);
    DrawModel(plane, Vector3Zero(), 2.0f, DARKGREEN);
    // SetShaderValue(shader, useNormalMapLoc, &(int){0}, SHADER_UNIFORM_INT);
    DrawModel(cube, (Vector3){0.5, 0.5, 0.5}, 1., DARKGREEN);

    EndShaderMode();

    // Draw sphere to show light position

    EndMode3D();

    Color textColor = (useNormalMap) ? DARKGREEN : RED;
    const char *toggleStr = (useNormalMap) ? "On" : "Off";
    DrawText(TextFormat("Use key [N] to toggle normal map: %s", toggleStr), 10,
             10, 10, textColor);

    int yOffset = 24;
    DrawText("Use keys [Up][Down] to change specular exponent", 10,
             10 + yOffset * 1, 10, BLACK);
    DrawText(TextFormat("Specular Exponent: %.2f", specularExponent), 10,
             10 + yOffset * 2, 10, BLUE);

    DrawFPS(screenWidth - 90, 10);

    EndDrawing();
    //--------------------------------------------------------------------------------------
  }

  // De-Initialization
  //--------------------------------------------------------------------------------------
  UnloadShader(shader);
  UnloadModel(plane);

  CloseWindow(); // Close window and OpenGL context
  //--------------------------------------------------------------------------------------

  return 0;
}
