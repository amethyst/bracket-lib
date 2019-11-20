REM @ECHO OFF
REM Build the actual WASM files and helpers
CALL :StageExample ex01-helloworld, ex01
CALL :StageExample ex02-sparse, ex02
CALL :StageExample ex03-walking, ex03
CALL :StageExample ex04-fov, ex04
CALL :StageExample ex05-dijkstra, ex05
CALL :StageExample ex06-astar-mouse, ex06
CALL :StageExample ex07-tiles, ex07
CALL :StageExample ex08-rex, ex08
CALL :StageExample ex09-offsets, ex09
CALL :StageExample ex10-postprocess, ex10
CALL :StageExample ex11-random, ex11
CALL :StageExample ex12-simplex, ex12
CALL :StageExample ex13-textblock, ex13
CALL :StageExample ex14-dwarfmap, ex14
CALL :StageExample ex15-specs, ex15
CALL :StageExample ex16-keyboard, ex16
CALL :Example17Builder

REM Duplicate example 1 into the root for compatibility with links I've already shared
copy .\wasm_help\staging\ex01\* .\wasm_help\staging

REM Submit to server
cd wasm_help\staging
pscp -r * herbert@172.16.10.193:/var/www/bfnightly/wasmtest
cd ..\..

REM Finish
EXIT /B 0

REM Usage: StageExample EXAMPLE
:StageExample
echo Building example %~1
cargo build --example %~1 --target wasm32-unknown-unknown --release
echo wasm-gc .\target\wasm32-unknown-unknown\release\examples\%~1.wasm
mkdir .\wasm_help\staging\%~2
wasm-bindgen .\target\wasm32-unknown-unknown\release\examples\%~1.wasm --out-dir .\wasm_help\staging\%~2 --no-modules --no-typescript
copy .\wasm_help\index.html .\wasm_help\staging\%~2
move .\wasm_help\staging\%~2\%~1_bg.wasm .\wasm_help\staging\%~2\myblob_bg.wasm
move .\wasm_help\staging\%~2\%~1.js .\wasm_help\staging\%~2\myblob.js
EXIT /B 0

REM This is for the "special" example, 17 - uses its own HTML elements
:Example17Builder
echo Special-Build for Example 17
cargo build --example ex17-wasm-external --target wasm32-unknown-unknown --release
mkdir .\wasm_help\staging\ex17
wasm-bindgen .\target\wasm32-unknown-unknown\release\examples\ex17-wasm-external.wasm --out-dir .\wasm_help\staging\ex17 --no-modules --no-typescript
copy .\wasm_help\index-ex17.html .\wasm_help\staging\ex17\index.html
move .\wasm_help\staging\ex17\ex17-wasm-external_bg.wasm .\wasm_help\staging\ex17\myblob_bg.wasm
move .\wasm_help\staging\ex17\ex17-wasm-external.js .\wasm_help\staging\ex17\myblob.js
