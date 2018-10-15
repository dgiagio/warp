# Warp
Warp lets you create self-contained single binary applications making it simpler and more ergonomic to deliver your application to your customers. A self-contained binary is specially convenient when the technology you use, such as .NET Core, Java and others, contain many dependencies that must be shipped alongside your application.

Warp is written in Rust and is supported on Linux, Windows and macOS.

## Quickstart with .NET Core
### Linux
**Create a simple console application**

```sh
dgiagio@X1:~/Devel$ mkdir myapp
dgiagio@X1:~/Devel$ cd myapp
dgiagio@X1:~/Devel/myapp$ dotnet new console
dgiagio@X1:~/Devel/myapp$ dotnet run
Hello World!
dgiagio@X1:~/Devel/myapp$
```

**Publish the application with native installer for `linux-x64` runtime**

```sh
dgiagio@X1:~/Devel/myapp$ dotnet publish -c Release -r linux-x64
```
The application should be published to `bin/Release/netcoreapp2.1/linux-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
dgiagio@X1:~/Devel/myapp$ curl -Lo warp-packer https://github.com/dgiagio/warp/releases/download/v0.2.1/linux-x64.warp-packer
dgiagio@X1:~/Devel/myapp$ chmod +x warp-packer
```

**Create your self-contained application**

```sh
dgiagio@X1:~/Devel/myapp$ ./warp-packer --arch linux-x64 --input_dir bin/Release/netcoreapp2.1/linux-x64/publish --exec myapp --output myapp
dgiagio@X1:~/Devel/myapp$ chmod +x myapp
```

**Run your self-contained application**

```sh
dgiagio@X1:~/Devel/myapp$ ./myapp
Hello World!
dgiagio@X1:~/Devel/myapp$
```

**More information about your self-contained application**

```sh
dgiagio@X1:~/Devel/myapp$ file myapp
myapp: ELF 64-bit LSB executable, x86-64, version 1 (GNU/Linux), statically linked, BuildID[sha1]=13b12e71a63ca1de8537ad7e90c83241f9f87f6c, with debug_info, not stripped

dgiagio@X1:~/Devel/myapp$ du -hs myapp
34M     myapp
```

### macOS
**Create a simple console application**

```sh
Diegos-iMac:Devel dgiagio$ mkdir myapp
Diegos-iMac:Devel dgiagio$ cd myapp
Diegos-iMac:myapp dgiagio$ dotnet new console
Diegos-iMac:myapp dgiagio$ dotnet run
Hello World!
Diegos-iMac:myapp dgiagio$
```

**Publish the application with native installer for `osx-x64` runtime**

```sh
Diegos-iMac:myapp dgiagio$ dotnet publish -c Release -r osx-x64
```
The application should be published to `bin/Release/netcoreapp2.1/osx-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
Diegos-iMac:myapp dgiagio$ curl -Lo warp-packer https://github.com/dgiagio/warp/releases/download/v0.2.1/macos-x64.warp-packer
Diegos-iMac:myapp dgiagio$ chmod +x warp-packer
```

**Create your self-contained application**

```sh
Diegos-iMac:myapp dgiagio$ ./warp-packer --arch macos-x64 --input_dir bin/Release/netcoreapp2.1/osx-x64/publish --exec myapp --output myapp
Diegos-iMac:myapp dgiagio$ chmod +x myapp
```

**Run your self-contained application**

```sh
Diegos-iMac:myapp dgiagio$ ./myapp
Hello World!
Diegos-iMac:myapp dgiagio$
```

**More information about your self-contained application**

```sh
Diegos-iMac:myapp dgiagio$ file myapp
myapp: Mach-O 64-bit executable x86_64

Diegos-iMac:myapp dgiagio$ du -hs myapp
 27M    myapp
```

### Windows
**Create a simple console application**

```powershell
PS C:\Users\Diego\Devel> mkdir myapp
PS C:\Users\Diego\Devel> cd myapp
PS C:\Users\Diego\Devel\myapp> dotnet new console
PS C:\Users\Diego\Devel\myapp> dotnet run
Hello World!
PS C:\Users\Diego\Devel\myapp>
```

**Publish the application with native installer for `win10-x64` runtime**

```powershell
PS C:\Users\Diego\Devel\myapp> dotnet publish -c Release -r win10-x64
```
The application should be published to `bin/Release/netcoreapp2.1/win10-x64/publish/`

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```powershell
PS C:\Users\Diego\Devel\myapp> [Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls" ; Invoke-WebRequest https://github.com/dgiagio/warp/releases/download/v0.2.1/windows-x64.warp-packer.exe -OutFile warp-packer.exe
```

**Create your self-contained application**

```powershell
PS C:\Users\Diego\Devel\myapp> .\warp-packer --arch windows-x64 --input_dir bin/Release/netcoreapp2.1/win10-x64/publish --exec myapp.exe --output myapp.exe
```

**Run your self-contained application**

```powershell
PS C:\Users\Diego\Devel\myapp> .\myapp.exe
Hello World!
PS C:\Users\Diego\Devel\myapp>
```

**More information about your self-contained application**

```powershell
PS C:\Users\Diego\Devel\myapp> "{0:N2} MB" -f ((Get-Item myapp.exe).Length / 1MB)
28.51 MB
```

## Quickstart with Java
_In progress_

## How it works
Warp is a multi-platform tool written in Rust and is comprised of two programs: `warp-runner` and `warp-packer`.

The final self-contained single binary application consists of two parts: 1) runner and 2) the compressed target application executable and dependencies.

<img src="https://image.ibb.co/bBe669/warp_app_binary.png" width="272">

`warp-runner` is a stub application that knows how to find the compressed payload within its own binary, perform extraction to a local cache and execute the target application.

The extraction process only happens the first time the application is ran, or when the self-contained application binary is updated.

`warp-packer` is a CLI application that's used to create the self-contained application binary by downloading the matching `warp-runner` for the chosen platform, compressing the target application and its dependencies, and generating the final self-contained binary.

### Performance
The performance characteristics of the generated self-contained application is roughly the same of original application, except for the first time it's ran as the target application and its dependencies have to be decompressed to a local cache.

### Cache location
- Linux: `$HOME/.local/share/warp`
- macOS: `$HOME/Library/Application Support/warp`
- Windows: `%LOCALAPPDATA%\warp`

## Authors
- Diego Giagio `<diego@giagio.com>`

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details
