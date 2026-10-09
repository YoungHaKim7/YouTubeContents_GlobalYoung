# Result

- `clang`

```bash
# 젤 먼저
cargo build --release

$ clang main.c -std=c23 \
          -I../../include \
          -L../../target/release \
          -labi_rust \
          -o app

$ ls
app*       main.c     README.md

$ ./app
30
  
```
- gcc

```bash
$ gcc main.c \
      -I../../include \
      -L../../target/release \
      -labi_rust \
       -o app

$ LD_LIBRARY_PATH=../../target/release ./app
30
```

# WindowsOS test(261009)
- Windows환경에서는 실행파일을 실행할때 `.dll` 파일이 필요하다.
- 파워셀에서 리눅스의 `\` 이 기능은 백틱`이다. 

```pwsh
PS C:\abi_rust\examples\c> Get-ChildItem ../../target/release/abi_rust*

    디렉터리: C:\abi_rust\target\release

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---        2026-10-09  오후 1:31            172 abi_rust.d
-a---        2026-10-09  오후 1:31           9728 abi_rust.dll
-a---        2026-10-09  오후 1:31           1200 abi_rust.dll.exp
-a---        2026-10-09  오후 1:31           2106 abi_rust.dll.lib
-a---        2026-10-09  오후 1:31         962560 abi_rust.pdb

PS C:\abi_rust\examples\c> clang main.c -std=c23 `
>>     -I../../include `
>>     ../../target/release/abi_rust.dll.lib `
>>     -o app.exe
PS C:\abi_rust\examples\c> ls

    디렉터리: C:\abi_rust\examples\c

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---        2026-10-09  오후 1:30              5 .gitignore
-a---        2026-10-09  오후 1:44         144384 app.exe
-a---        2026-10-09  오후 1:30            124 main.c
-a---        2026-10-09  오후 1:30            455 README.md

PS C:\abi_rust\examples\c> .\app.exe
PS C:\abi_rust\examples\c> cp -Force ..\..\target\release\abi_rust.dll ./.
PS C:\abi_rust\examples\c> ls

    디렉터리: C:\abi_rust\examples\c

Mode                 LastWriteTime         Length Name
----                 -------------         ------ ----
-a---        2026-10-09  오후 1:30              5 .gitignore
-a---        2026-10-09  오후 1:31           9728 abi_rust.dll
-a---        2026-10-09  오후 1:44         144384 app.exe
-a---        2026-10-09  오후 1:30            124 main.c
-a---        2026-10-09  오후 1:30            455 README.md

$ .\app.exe
30

```
