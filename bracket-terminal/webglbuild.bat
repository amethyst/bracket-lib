@ECHO OFF
REM Build the actual WASM files and helpers
REM CALL :StageExample hello_minimal, hello_minimal
REM CALL :StageExample hello_terminal, hello_terminal
REM CALL :StageExample astar_mouse, astar_mouse
REM CALL :StageExample dwarfmap, dwarfmap
REM CALL :StageExample postprocess, postprocess
REM CALL :StageExample rex, rex
REM CALL :StageExample sparse, sparse
REM CALL :StageExample textblock, textblock
REM CALL :StageExample textsprites, textsprites
REM CALL :StageExample tiles, tiles
REM CALL :StageExample walking, walking
REM CALL :StageExample no_cls, no_cls
REM CALL :StageExample native_gl, native_gl
REM CALL :StageExample keyboard, keyboard
REM CALL :StageExample input_harness, input_harness
REM CALL :StageExample virtual_console, virtual_console
REM CALL :StageExample alpha, alpha
REM CALL :StageExample colorfont, colorfont
REM CALL :StageExample fontswitch, fontswitch
CALL :StageExample fancy, fancy
CALL :StageExample sprites, sprites


REM Submit to server
cd wasm_help\staging
pscp -r * herbert@vps.bracketproductions.com:/var/www/bfnightly/wasmtest
cd ..\..

REM Finish
EXIT /B 0

REM Usage: StageExample EXAMPLE
:StageExample
echo Building example %~1
cargo build --example %~1 --target wasm32-unknown-unknown --release --features=opengl
echo wasm-gc ..\target\wasm32-unknown-unknown\release\examples\%~1.wasm
mkdir .\wasm_help\staging\%~2
wasm-bindgen ..\target\wasm32-unknown-unknown\release\examples\%~1.wasm --out-dir .\wasm_help\staging\%~2 --no-modules --no-typescript
copy .\wasm_help\index.html .\wasm_help\staging\%~2
move .\wasm_help\staging\%~2\%~1_bg.wasm .\wasm_help\staging\%~2\myblob_bg.wasm
move .\wasm_help\staging\%~2\%~1.js .\wasm_help\staging\%~2\myblob.js
EXIT /B 0
