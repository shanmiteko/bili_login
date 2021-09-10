[![publish-release](https://github.com/shanmiteko/bili_login/actions/workflows/build.yml/badge.svg)](https://github.com/shanmiteko/bili_login/actions/workflows/build.yml)

### 开发依赖
[tauri-setup-linux](https://tauri.studio/en/docs/getting-started/setup-linux/)

**openSUSE**
```
$ sudo zypper install -t pattern devel_C_C++
$ sudo zypper dup && sudo zypper in webkit2gtk3-devel \
    openssl-devel \
    curl \
    wget \
    libappindicator3-devel \
    patchelf \
    librsvg2-devel
```

>fatal error: vips/vips8: No such file or directory
>
>`$ sudo zypper in libvips-devel`


