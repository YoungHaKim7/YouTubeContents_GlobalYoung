# Result

- c++26

```bash
$ clang++ main.cpp -std=c++26 \
          -I../../include \
          -L../../target/release \
          -labi_rust \
          -o app

$ ls
app*       main.cpp   README.md

LD_LIBRARY_PATH=../../target/release ./app
300
```

## WinOS 11 test(261009)

```pwsh
clang++ main.cpp -std=c++26 `
>> -I../../include `
>> -L../../target/release `
>> -labi_rust `
>> -o app.exe
```
