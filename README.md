<div align="center">
  <a href="https://aviso.bz/" target="_blank"><img src="https://aviso.bz/statica/pictures/A-468.gif" style="width:468px;heigh:60px" alt="aviso | рекламный сервис"></a>
</div>

        
**A website that pays for micro tasks like watching YouTube videos or browsing other websites. If you have any questions, you can call me [here](https://t.me/westsidedev)**

<h2>DOWNLOAD:</h2>

+ **Linux** *(requires Ubuntu 24 or GLIBC_2.39)*

  Make sure the browser and driver binaries are located in /bin, the software will look for them in this location. The browser name must only match its name, otherwise you must rename it, the software only supports the brave browser, if you don't have it on your machine you must install it.

  Ex: *brave-browser* for *brave* only
  
  **Go to the release page, or click [here](https://github.com/westsidedev/aviso-bz/releases)**

+ **Termux** *(arm64)*

  **Go to the release page, or click [here](https://github.com/westsidedev/aviso-bz/releases)**

+ **Windows**

  **Go to the release page, or click [here](https://github.com/westsidedev/aviso-bz/releases)**
  
<h2>BUILD:</h2>

**If your distribution is different from Ubuntu 24 or lower GLIBC_2.39 consider building from source**

+ **1. Install the latest version of [Rust](https://www.rust-lang.org/tools/install)**
+ **2. Clone this repository**
+ **3. run `cargo build --release`**

<h2>INSTALATION:</h2>

After downloading, create a folder and extract the zip inside it, then give it permission, in termux you must put it in your HOME

+ **Linux**

```bash
mkdir avisobz
mv avisobz-linux.zip avisobz
cd avisobz
unzip avisobz-linux.zip
chmod +x aviso-bz
./aviso-bz
```

+ **Termux**

```bash
pkg update 
pkg upgrade 
pkg install x11-repo 
pkg install tur-repo 
pkg install chromium
pkg install zip
termux-setup-storage
mv /sdcard/Download/avisobz-termux.zip $HOME
mkdir avisobz
mv avisobz-termux.zip avisobz
cd avisobz
unzip avisobz-termux.zip
chmod +x aviso-bz
./aviso-bz
```

<h2>COMMANDS:</h2>

```
--email      Email used for login in avisobz
--passw      Password used for login in avisobz
--start      Start software after first execution
--headless   Active headless mode (OPTIONAL)
--help       Show this message
--YT         Youtube mode
--SF         Surfing mode
--All        Youtube and surfing mode
```

<h2>USAGE:</h5>

**FIRST EXECUTION:**

```bash
./aviso-bz --email xxxx@xxxx --passw 123456 --YT --headless
```

**START**

```bash
./aviso-bz --start --YT --headless
```

<h2 align="center"><a href="LICENSE.txt">Apache License</a></h2>
<div align="center"><b>Copyright &copy; 2025 westsidedev. All rights reserved.</b></div>

