#!/usr/bin/env sh

GL_XML="https://raw.githubusercontent.com/KhronosGroup/OpenGL-Registry/master/xml/gl.xml"

SCRIPT_PATH=$(readlink -f "$0")
SCRIPT_DIR=$(dirname "$SCRIPT_PATH")
REGISTRY=$SCRIPT_DIR/registry
[ -d "$REGISTRY" ] || mkdir -p "$REGISTRY"

echo "Acquiring Khronos XML registry... " && \
  echo "+ Downloading 'gl.xml'" && \
  curl -sSf -o "$REGISTRY/gl.xml" "$GL_XML" && \
  echo "Done!"
