# Warp
Warp lets you create self-contained single binary applications making it simpler and more ergonomic to deliver your application to your customers. A self-contained binary is specially convenient when the technology you use, such as Node.js, .NET Core, Java and others, contain many dependencies that must be shipped alongside your application.

Warp is written in Rust and is supported on Linux, Windows and macOS.

### Table of Content
  * [Quickstart with Node.js](#quickstart-with-nodejs)
    + [Linux](#linux)
    + [macOS](#macos)
    + [Windows](#windows)
  * [Quickstart with .NET Core](#quickstart-with-net-core)
    + [Linux](#linux-1)
    + [macOS](#macos-1)
    + [Windows](#windows-1)
  * [Quickstart with Java](#quickstart-with-java)
  * [How it works](#how-it-works)
    + [Performance](#performance)
    + [Packages cache location](#packages-cache-location)
    + [Runners cache location](#runners-cache-location)
  * [Authors](#authors)
  * [License](#license)


## Quickstart with Node.js
### Linux
**Create the directory for the application**
```sh
dgiagio@X1:~/Devel$ mkdir myapp
dgiagio@X1:~/Devel/myapp$ cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```sh
dgiagio@X1:~/Devel/myapp$ wget https://nodejs.org/dist/v8.12.0/node-v8.12.0-linux-x64.tar.xz
dgiagio@X1:~/Devel/myapp$ xz -dc node-v8.12.0-linux-x64.tar.xz | tar xvf -
```

**Install dependencies**
```sh
dgiagio@X1:~/Devel/myapp$ node-v8.12.0-linux-x64/bin/npm install lodash
```

**Remove unneeded files**
```sh
dgiagio@X1:~/Devel/myapp$ rm -r node-v8.12.0-linux-x64/include node-v8.12.0-linux-x64/share node-v8.12.0-linux-x64/lib
dgiagio@X1:~/Devel/myapp$ rm node-v8.12.0-linux-x64/bin/npm node-v8.12.0-linux-x64/bin/npx
```

**Create launcher script** - `launch`
```sh
#!/bin/sh

NODE_DIST=node-v8.12.0-linux-x64
APP_MAIN_JS=app.js

DIR="$(cd "$(dirname "$0")" ; pwd -P)"
NODE_EXE=$DIR/$NODE_DIST/bin/node
NODE_PATH=$DIR/node_modules
APP_MAIN_JS_PATH=$DIR/$APP_MAIN_JS

exec $NODE_EXE $APP_MAIN_JS_PATH $@
```

**Make the launcher script executable**
```sh
dgiagio@X1:~/Devel/myapp$ chmod +x launch
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
dgiagio@X1:~/Devel/myapp$ cd ..
dgiagio@X1:~/Devel$ curl -Lo warp-packer https://github.com/dgiagio/warp/releases/download/v0.3.0/linux-x64.warp-packer
dgiagio@X1:~/Devel$ chmod +x warp-packer
```

**Create your self-contained application**

```sh
dgiagio@X1:~/Devel$ ./warp-packer --arch linux-x64 --input_dir myapp --exec launch --output myapp.bin
dgiagio@X1:~/Devel$ chmod +x myapp.bin
```

**Run your self-contained application**

```sh
dgiagio@X1:~/Devel$ ./myapp.bin
[ 2, 3 ]
dgiagio@X1:~/Devel$
```

**More information about your self-contained application**

```sh
dgiagio@X1:~/Devel/myapp$ file myapp.bin
myapp.bin: ELF 64-bit LSB shared object, x86-64, version 1 (SYSV), dynamically linked, interpreter /lib64/ld-linux-x86-64.so.2, for GNU/Linux 3.2.0, BuildID[sha1]=aa53b01be2cde5e0b64450870b1af13b52d5cffb, with debug_info, not stripped

dgiagio@X1:~/Devel/myapp$ du -hs myapp.bin
17M     myapp.bin
```

### macOS
**Create the directory for the application**
```sh
Diegos-iMac:Devel dgiagio$ mkdir myapp
Diegos-iMac:Devel dgiagio$ cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```sh
Diegos-iMac:myapp dgiagio$ curl -Lo node-v8.12.0-darwin-x64.tar.gz https://nodejs.org/dist/v8.12.0/node-v8.12.0-darwin-x64.tar.gz
Diegos-iMac:myapp dgiagio$ tar xvfz node-v8.12.0-darwin-x64.tar.gz
```

**Install dependencies**
```sh
Diegos-iMac:myapp dgiagio$ PATH=node-v8.12.0-darwin-x64/bin npm install lodash
```

**Remove unneeded files**
```sh
Diegos-iMac:myapp dgiagio$ rm -r node-v8.12.0-darwin-x64/include node-v8.12.0-darwin-x64/share node-v8.12.0-darwin-x64/lib
Diegos-iMac:myapp dgiagio$ rm node-v8.12.0-darwin-x64/bin/npm node-v8.12.0-darwin-x64/bin/npx
```

**Create launcher script*** - `launch`
```sh
#!/bin/sh

NODE_DIST=node-v8.12.0-darwin-x64
APP_MAIN_JS=app.js

DIR="$(cd "$(dirname "$0")" ; pwd -P)"
NODE_EXE=$DIR/$NODE_DIST/bin/node
NODE_PATH=$DIR/node_modules
APP_MAIN_JS_PATH=$DIR/$APP_MAIN_JS

exec "$NODE_EXE" "$APP_MAIN_JS_PATH" $@
```

**Make the launcher script executable**
```sh
Diegos-iMac:myapp dgiagio$ chmod +x launch
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```sh
Diegos-iMac:myapp dgiagio$ cd ..
Diegos-iMac:Devel dgiagio$ curl -Lo warp-packer https://github.com/dgiagio/warp/releases/download/v0.3.0/macos-x64.warp-packer
Diegos-iMac:Devel dgiagio$ chmod +x warp-packer
```

**Create your self-contained application**

```sh
Diegos-iMac:Devel dgiagio$ ./warp-packer --arch macos-x64 --input_dir myapp --exec launch --output myapp.bin
Diegos-iMac:Devel dgiagio$ chmod +x myapp.bin
```

**Run your self-contained application**

```sh
Diegos-iMac:Devel dgiagio$ ./myapp.bin
[ 2, 3 ]
Diegos-iMac:Devel dgiagio$
```

**More information about your self-contained application**

```sh
Diegos-iMac:Devel dgiagio$ file myapp.bin
myapp.bin: Mach-O 64-bit executable x86_64

Diegos-iMac:Devel dgiagio$ du -hs myapp.bin
26M     myapp.bin
```

### Windows
**Create the directory for the application**
```powershell
PS C:\Users\Diego\Devel> mkdir myapp
PS C:\Users\Diego\Devel> cd myapp
```

**Create main application** - `app.js`
```javascript
var lodash = require('lodash');
var output = lodash.without([1, 2, 3], 1);
console.log(output);
```

**Download Node.js distribution**
```powershell
PS C:\Users\Diego\Devel\myapp> curl https://nodejs.org/dist/v8.12.0/node-v8.12.0-win-x64.zip -OutFile node-v8.12.0-win-x64.zip
PS C:\Users\Diego\Devel\myapp> Expand-Archive .\node-v8.12.0-win-x64.zip -DestinationPath .\
```

**Install dependencies**
```powershell
PS C:\Users\Diego\Devel\myapp> .\node-v8.12.0-win-x64\npm install lodash
```

**Remove unneeded files**
```powershell
PS C:\Users\Diego\Devel\myapp> rmdir -Recurse .\node-v8.12.0-win-x64\node_modules\npm
```

**Create launcher script*** - `launch.cmd`
```bat
@ECHO OFF

SETLOCAL

SET "NODE_DIST=node-v8.12.0-win-x64"
SET "APP_MAIN_JS=app.js"

SET "NODE_EXE=%~dp0\%NODE_DIST%\node.exe"
SET "NODE_PATH=%~dp0\%NODE_DIST%\node_modules"
SET "APP_MAIN_JS_PATH=%~dp0\%APP_MAIN_JS%"

CALL %NODE_EXE% %APP_MAIN_JS_PATH% %*
EXIT /B %ERRORLEVEL%
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.
```powershell
PS C:\Users\Diego\Devel\myapp> cd ..
PS C:\Users\Diego\Devel> [Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls" ; Invoke-WebRequest https://github.com/dgiagio/warp/releases/download/v0.3.0/windows-x64.warp-packer.exe -OutFile warp-packer.exe
```

**Create your self-contained application**

```powershell
PS C:\Users\Diego\Devel> .\warp-packer --arch windows-x64 --input_dir .\myapp\ --exec launch.cmd --output myapp.exe
```

**Run your self-contained application**

```powershell
PS C:\Users\Diego\Devel> .\myapp.exe
[ 2, 3 ]
PS C:\Users\Diego\Devel>
```

**More information about your self-contained application**

```powershell
PS C:\Users\Diego\Devel> "{0:N2} MB" -f ((Get-Item myapp.exe).Length / 1MB)
9.15 MB
```

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
dgiagio@X1:~/Devel/myapp$ curl -Lo warp-packer https://github.com/dgiagio/warp/releases/download/v0.3.0/linux-x64.warp-packer
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
Diegos-iMac:myapp dgiagio$ curl -Lo warp-packer https://github.com/dgiagio/warp/releases/download/v0.3.0/macos-x64.warp-packer
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
PS C:\Users\Diego\Devel\myapp> [Net.ServicePointManager]::SecurityProtocol = "tls12, tls11, tls" ; Invoke-WebRequest https://github.com/dgiagio/warp/releases/download/v0.3.0/windows-x64.warp-packer.exe -OutFile warp-packer.exe
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
### Linux

**Create a Hello World application**

Create `HelloWorld.java`: 

```java
// HelloWorld.java
public final class HelloWorld {
  public static void main(final String[] args) {
    System.out.println("Hello, world. ");
  }
}
```

Test that it works: 

```bash
$ javac HelloWorld.java
$ java HelloWorld
Hello, world.
```

We need to bundle this as a `.jar`:

```bash
$ jar cvfe app.jar HelloWorld HelloWorld.class
added manifest
adding: HelloWorld.class(in = 428) (out= 290)(deflated 32%)
$ java -jar app.jar
Hello, world.
```

**Download a JRE**

There are prebuilt JREs over on [AdoptOpenJDK](https://adoptopenjdk.net). 

Here we use JRE 8:

```
wget -N https://github.com/AdoptOpenJDK/openjdk8-binaries/releases/download/jdk8u202-b08/OpenJDK8U-jre_x64_linux_hotspot_8u202b08.tar.gz
```

Unpack it: 

```
tar -xvf OpenJDK8U-jre_x64_linux_hotspot_8u202b08.tar.gz
```

**Create a bundle**

We need to create a folder containing: our compiled code, the JRE and a launch script. 

```
mkdir bundle
cp -r ./jdk8u202-b08-jre ./bundle/jre
cp app.jar ./bundle/app.jar
touch bundle/run.sh
chmod +x ./bundle/run.sh 
```

Finally, we to write `run.sh`. This script will run our `.jar` using the bundled JRE.

Here are the contents of `./bundle/run.sh`:

```bash
#!/usr/bin/env bash

HERE=${BASH_SOURCE%/*}

"$HERE/jre/bin/java" -jar "$HERE/app.jar" "$@"
```

Test the bundle: 

```bash
$ ./bundle/run.sh 
Hello, world. 
```

**Download `warp-packer`**

If you save `warp-packer` in a directory in your PATH, you only need to download it once.

```bash
$ wget -O warp-packer https://github.com/dgiagio/warp/releases/download/v0.3.0/linux-x64.warp-packer
$ chmod +x ./warp-packer
```

**Create your self-contained application**

```bash
$ ./warp-packer --arch linux-x64 --input_dir bundle --exec run.sh --output app.bin
$ chmod +x app.bin
```

**Run your self-contained application**

```bash
$ ./app.bin 
Hello, world. 
```

## How it works
Warp is a multi-platform tool written in Rust and is comprised of two programs: `warp-runner` and `warp-packer`.

The final self-contained single binary application consists of two parts: 1) runner and 2) the compressed target application executable and dependencies.

<img src="https://image.ibb.co/bBe669/warp_app_binary.png" width="272">

`warp-runner` is a stub application that knows how to find the compressed payload within its own binary, perform extraction to a local cache and execute the target application.

The extraction process only happens the first time the application is ran, or when the self-contained application binary is updated.

`warp-packer` is a CLI application that's used to create the self-contained application binary by downloading the matching `warp-runner` for the chosen platform, compressing the target application and its dependencies, and generating the final self-contained binary.

### Performance
The performance characteristics of the generated self-contained application is roughly the same of original application, except for the first time it's ran as the target application and its dependencies have to be decompressed to a local cache.

### Packages cache location
- Linux: `$HOME/.local/share/warp/packages`
- macOS: `$HOME/Library/Application Support/warp/packges`
- Windows: `%LOCALAPPDATA%\warp\packages`

### Runners cache location
- Linux: `$HOME/.local/share/warp/runners`
- macOS: `$HOME/Library/Application Support/warp/runners`
- Windows: `%LOCALAPPDATA%\warp\runners`

## Authors
- Diego Giagio `<diego@giagio.com>`

## License
This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details

## Who is Using Warp? 

 * Buckaroo, the C++ package manager https://github.com/loopperfect/buckaroo
 * Buck, the build system https://github.com/njlr/buck-warp
