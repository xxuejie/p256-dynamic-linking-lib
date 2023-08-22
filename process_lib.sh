#!/bin/bash
set -ex

# This bash script extract objects from an archive file, then convert all functions
# in the archive file to local except for certain exported symbols.
# It is used to workaround this limitation of Rust: https://github.com/rust-lang/rust/issues/18541,
# so we can build a minimal yet dynamic linking library.

ARCHIVE_FILE=$(realpath $1)
SYMBOL_FILE=$(realpath $2)
OUTPUT_PATH=$3
AR_COMMAND=$4
OBJCOPY_COMMAND=$5

rm -rf $OUTPUT_PATH
mkdir -p $OUTPUT_PATH

cd $OUTPUT_PATH
$AR_COMMAND x $ARCHIVE_FILE

# FIXME: unused symbols shall be kept HIDDEN but GLOBAL, making them LOCAL would break the linking process.

# for o in $(find . -name "*.o"); do
#   for s in $(grep -oP "[a-zA-Z_][a-zA-Z_0-9]*" $SYMBOL_FILE); do
#     $OBJCOPY_COMMAND --keep-global-symbol=$s $o $o
#   done
# done

echo "All done!"
