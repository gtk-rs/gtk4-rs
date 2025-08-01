// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

#include "manual.h"
#include <stdio.h>

#define PRINT_CONSTANT(CONSTANT_NAME) \
    printf("%s;", #CONSTANT_NAME); \
    printf(_Generic((CONSTANT_NAME), \
                    char *: "%s", \
                    const char *: "%s", \
                    char: "%c", \
                    signed char: "%hhd", \
                    unsigned char: "%hhu", \
                    short int: "%hd", \
                    unsigned short int: "%hu", \
                    int: "%d", \
                    unsigned int: "%u", \
                    long: "%ld", \
                    unsigned long: "%lu", \
                    long long: "%lld", \
                    unsigned long long: "%llu", \
                    float: "%f", \
                    double: "%f", \
                    long double: "%ld"), \
           CONSTANT_NAME); \
    printf("\n");

int main() {
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_COLOR);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_COLOR_BURN);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_COLOR_DODGE);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_DARKEN);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_DEFAULT);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_DIFFERENCE);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_EXCLUSION);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_HARD_LIGHT);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_HUE);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_LIGHTEN);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_LUMINOSITY);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_MULTIPLY);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_OVERLAY);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_SATURATION);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_SCREEN);
    PRINT_CONSTANT((gint) GSK_BLEND_MODE_SOFT_LIGHT);
    PRINT_CONSTANT((gint) GSK_BLEND_NODE);
    PRINT_CONSTANT((gint) GSK_BLUR_NODE);
    PRINT_CONSTANT((gint) GSK_BORDER_NODE);
    PRINT_CONSTANT((gint) GSK_CAIRO_NODE);
    PRINT_CONSTANT((gint) GSK_CLIP_NODE);
    PRINT_CONSTANT((gint) GSK_COLOR_MATRIX_NODE);
    PRINT_CONSTANT((gint) GSK_COLOR_NODE);
    PRINT_CONSTANT((gint) GSK_CONIC_GRADIENT_NODE);
    PRINT_CONSTANT((gint) GSK_CONTAINER_NODE);
    PRINT_CONSTANT((gint) GSK_CORNER_BOTTOM_LEFT);
    PRINT_CONSTANT((gint) GSK_CORNER_BOTTOM_RIGHT);
    PRINT_CONSTANT((gint) GSK_CORNER_TOP_LEFT);
    PRINT_CONSTANT((gint) GSK_CORNER_TOP_RIGHT);
    PRINT_CONSTANT((gint) GSK_CROSS_FADE_NODE);
    PRINT_CONSTANT((gint) GSK_DEBUG_NODE);
    PRINT_CONSTANT((gint) GSK_FILL_NODE);
    PRINT_CONSTANT((gint) GSK_FILL_RULE_EVEN_ODD);
    PRINT_CONSTANT((gint) GSK_FILL_RULE_WINDING);
    PRINT_CONSTANT((gint) GSK_GL_SHADER_NODE);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_BOOL);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_FLOAT);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_INT);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_NONE);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_UINT);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_VEC2);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_VEC3);
    PRINT_CONSTANT((gint) GSK_GL_UNIFORM_TYPE_VEC4);
    PRINT_CONSTANT((gint) GSK_INSET_SHADOW_NODE);
    PRINT_CONSTANT((gint) GSK_LINEAR_GRADIENT_NODE);
    PRINT_CONSTANT((gint) GSK_LINE_CAP_BUTT);
    PRINT_CONSTANT((gint) GSK_LINE_CAP_ROUND);
    PRINT_CONSTANT((gint) GSK_LINE_CAP_SQUARE);
    PRINT_CONSTANT((gint) GSK_LINE_JOIN_BEVEL);
    PRINT_CONSTANT((gint) GSK_LINE_JOIN_MITER);
    PRINT_CONSTANT((gint) GSK_LINE_JOIN_ROUND);
    PRINT_CONSTANT((gint) GSK_MASK_MODE_ALPHA);
    PRINT_CONSTANT((gint) GSK_MASK_MODE_INVERTED_ALPHA);
    PRINT_CONSTANT((gint) GSK_MASK_MODE_INVERTED_LUMINANCE);
    PRINT_CONSTANT((gint) GSK_MASK_MODE_LUMINANCE);
    PRINT_CONSTANT((gint) GSK_MASK_NODE);
    PRINT_CONSTANT((gint) GSK_NOT_A_RENDER_NODE);
    PRINT_CONSTANT((gint) GSK_OPACITY_NODE);
    PRINT_CONSTANT((gint) GSK_OUTSET_SHADOW_NODE);
    PRINT_CONSTANT((gint) GSK_PATH_CLOSE);
    PRINT_CONSTANT((gint) GSK_PATH_CONIC);
    PRINT_CONSTANT((gint) GSK_PATH_CUBIC);
    PRINT_CONSTANT((guint) GSK_PATH_FOREACH_ALLOW_CONIC);
    PRINT_CONSTANT((guint) GSK_PATH_FOREACH_ALLOW_CUBIC);
    PRINT_CONSTANT((guint) GSK_PATH_FOREACH_ALLOW_ONLY_LINES);
    PRINT_CONSTANT((guint) GSK_PATH_FOREACH_ALLOW_QUAD);
    PRINT_CONSTANT((gint) GSK_PATH_FROM_END);
    PRINT_CONSTANT((gint) GSK_PATH_FROM_START);
    PRINT_CONSTANT((gint) GSK_PATH_INTERSECTION_END);
    PRINT_CONSTANT((gint) GSK_PATH_INTERSECTION_NONE);
    PRINT_CONSTANT((gint) GSK_PATH_INTERSECTION_NORMAL);
    PRINT_CONSTANT((gint) GSK_PATH_INTERSECTION_START);
    PRINT_CONSTANT((gint) GSK_PATH_LINE);
    PRINT_CONSTANT((gint) GSK_PATH_MOVE);
    PRINT_CONSTANT((gint) GSK_PATH_QUAD);
    PRINT_CONSTANT((gint) GSK_PATH_TO_END);
    PRINT_CONSTANT((gint) GSK_PATH_TO_START);
    PRINT_CONSTANT((gint) GSK_RADIAL_GRADIENT_NODE);
    PRINT_CONSTANT((gint) GSK_REPEATING_LINEAR_GRADIENT_NODE);
    PRINT_CONSTANT((gint) GSK_REPEATING_RADIAL_GRADIENT_NODE);
    PRINT_CONSTANT((gint) GSK_REPEAT_NODE);
    PRINT_CONSTANT((gint) GSK_ROUNDED_CLIP_NODE);
    PRINT_CONSTANT((gint) GSK_SCALING_FILTER_LINEAR);
    PRINT_CONSTANT((gint) GSK_SCALING_FILTER_NEAREST);
    PRINT_CONSTANT((gint) GSK_SCALING_FILTER_TRILINEAR);
    PRINT_CONSTANT((gint) GSK_SERIALIZATION_INVALID_DATA);
    PRINT_CONSTANT((gint) GSK_SERIALIZATION_UNSUPPORTED_FORMAT);
    PRINT_CONSTANT((gint) GSK_SERIALIZATION_UNSUPPORTED_VERSION);
    PRINT_CONSTANT((gint) GSK_SHADOW_NODE);
    PRINT_CONSTANT((gint) GSK_STROKE_NODE);
    PRINT_CONSTANT((gint) GSK_SUBSURFACE_NODE);
    PRINT_CONSTANT((gint) GSK_TEXTURE_NODE);
    PRINT_CONSTANT((gint) GSK_TEXTURE_SCALE_NODE);
    PRINT_CONSTANT((gint) GSK_TEXT_NODE);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_CATEGORY_2D);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_CATEGORY_2D_AFFINE);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_CATEGORY_2D_TRANSLATE);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_CATEGORY_3D);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_CATEGORY_ANY);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_CATEGORY_IDENTITY);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_CATEGORY_UNKNOWN);
    PRINT_CONSTANT((gint) GSK_TRANSFORM_NODE);
    return 0;
}
