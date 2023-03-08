# pastebin

在线剪贴板

## 开发

前置要求

+ Node.js
+ Rust
+ Nginx
+ Redis
+ [just](https://github.com/casey/just)

下载后端依赖

```bash
cd pastebin-server
cargo fetch
```

下载前端依赖

```bash
cd pastebin-front
npm install
```

启用 nginx 配置文件

```bash
sudo ln -s $PWD/pastebin.nginx.conf /etc/nginx/sites-enabled/pastebin
sudo nginx -t
sudo nginx -s reload
```

启动后端

```bash
cd pastebin-server
cargo run --release
```

启动前端开发服务器

```bash
cd pastebin-front
npm run dev
```

打开页面 <http://localhost>

## 部署

编译并打包前端与后端

```bash
just dist
```

将 dist 目录下的最新压缩包上传至服务器，解压并修改配置，自行部署

## 其他

删除生成文件，释放空间

```bash
just clean
```
