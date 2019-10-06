(
cd vendor/submodules/fmt && 
mkdir build && 
cd build && 
cmake .. -DFMT_TEST=false &&
make
)
