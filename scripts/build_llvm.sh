#!/usr/bin/env sh

# A script to build LLVM from sources.
#
# Uses Apple Clang builds documented in LLVM: https://llvm.org/docs/AdvancedBuilds.html#apple-clang-builds-a-more-complex-bootstrap
#
# Before running you need:
# * MacOS:
#   ```
#   brew install bzip2 cmake coreutils git lz4 make ninja xz zlib zstd lld
#   ```
#

set -eux

LLVM_PROJ_DIR=/tmp/llvm-project-$1
LLVM_TAG="llvmorg-$1"
OUTPUT_DIR="$2"
MAJOR_VERSION=$(cut -d '.' -f 1 <<< $1)

REPO_URL="https://github.com/llvm/llvm-project.git"

# Color definitions
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
RESET='\033[0m'

# Check if the system is macOS
if [[ "$(uname)" != "Darwin" ]]; then
  echo "${RED}❌ This script is intended to run on macOS only. Exiting.${RESET}"
  exit 1
fi


# Check if the repository is already cloned
if [ ! -d "$LLVM_PROJ_DIR/.git" ]; then
    echo "${CYAN}🛠️ Cloning repository into $LLVM_PROJ_DIR...${RESET}"
    git clone "$REPO_URL" --branch $LLVM_TAG --depth 1 "$LLVM_PROJ_DIR"
else
    echo "${YELLOW}🔄 Repository already cloned. Fetching latest updates...${RESET}"
    git -C "$LLVM_PROJ_DIR" fetch --depth=1 origin $LLVM_TAG
fi

# Checkout the desired tag
echo "${BLUE}📌 Checking out tag $LLVM_TAG...${RESET}"
git -C "$LLVM_PROJ_DIR" checkout "tags/$LLVM_TAG" -B "branch-$LLVM_TAG"
git -C "$LLVM_PROJ_DIR" reset --hard "tags/$LLVM_TAG"

pushd $LLVM_PROJ_DIR

# Apply patch
# echo "${PURPLE}🩹 Applying patch...${RESET}"
# curl https://raw.githubusercontent.com/scasagrande/build_macos_llvm/refs/heads/main/update-DistributionExample.cmake.patch | git apply -v

# Now CMake time
LLD_LINKER=""
if command -v lld >/dev/null 2>&1; then
    echo "${GREEN}✅ lld is available.${RESET}"
    LLD_LINKER="-DLLVM_USE_LINKER=lld"
fi

echo "${CYAN}🏗️ Configuring build with CMake...${RESET}"
rm -f build/CMakeCache.txt

# # Apple clang - bootstrap build: https://llvm.org/docs/AdvancedBuilds.html#apple-clang-builds-a-more-complex-bootstrap
# FIXME: This build is failing in MacOS Sequoia: https://github.com/llvm/llvm-project/issues/109549
# cmake -S llvm -B build \
#     -G Ninja \
#     -C clang/cmake/caches/Apple-stage1.cmake \
#     $LLD_LINKER \
#     -DCMAKE_INSTALL_PREFIX="$OUTPUT_DIR" \
#     -DCMAKE_BUILD_TYPE=Release \
#     -DLLVM_TARGETS_TO_BUILD=X86 \
#     -DLLVM_ENABLE_RUNTIMES="compiler-rt;libunwind;libcxx;libcxxabi" \
#     -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;lld"
# echo "${BLUE}🔨 Building LLVM...${RESET}"
# ninja -C build stage2-distribution

# Regular CMake build
cmake -S llvm -B build \
    -G Ninja \
    $LLD_LINKER \
    -DCMAKE_INSTALL_PREFIX="$OUTPUT_DIR" \
    -DCMAKE_BUILD_TYPE=Release \
    -DLLVM_TARGETS_TO_BUILD=X86 \
    -DLLVM_ENABLE_RUNTIMES="compiler-rt;libcxx;libcxxabi;libunwind" \
    -DLLVM_ENABLE_PROJECTS="clang;clang-tools-extra;lld"
echo "${BLUE}🔨 Building LLVM...${RESET}"
ninja -C build install

popd

echo "${GREEN}📦 Bazelify the install folder...${RESET}"
echo "module(name = 'llvm', version = '$1')\n" > $OUTPUT_DIR/MODULE.bazel
wget -O $OUTPUT_DIR/BUILD.bazel https://raw.githubusercontent.com/bazel-contrib/toolchains_llvm/refs/tags/v1.3.0/toolchain/BUILD.llvm_repo

# cd $LLVM_PROJ_DIR

# git apply update-DistributionExample.cmake.patch

# mkdir build && cd build
# cmake -G Ninja -C ../clang/cmake/caches/DistributionExample.cmake ../llvm
# ninja stage2-distribution

# cd tools/clang/stage2-bins
# find lib bin include -type d -name "CMakeFiles" -prune -exec rm -r {} +
# find lib bin include -name cmake_install.cmake -delete
# rm lib/libLLVM*.a lib/libclang*.a lib/liblld*.a
# rm bin/{clang,clang++,clang-cl,clang-cpp}
# rm bin/{ld.lld,ld64.lld,lld-link,wasm-ld}
# rm bin/llvm-ranlib

# pushd bin

# ln -s clang-${MAJOR_VERSION} clang
# ln -s clang-${MAJOR_VERSION} clang++
# ln -s clang-${MAJOR_VERSION} clang-cl
# ln -s clang-${MAJOR_VERSION} clang-cpp

# ln -s lld ld.lld
# ln -s lld ld64.lld
# ln -s lld lld-link
# ln -s lld wasm-ld

# ln -s llvm-ar llvm-ranlib

# popd

# XZ_OPT="-9e -T0" tar -cJf ${OUTPUT_PATH} bin include lib
