# 参与开发

代码风格和提交约定见 [AGENTS.md](AGENTS.md)（和 CLAUDE.md 内容相同），动手前先读 [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)。

## 从源码构建

macOS 需要 Xcode Command Line Tools；Windows 需要 Visual Studio 2022 C++ Build Tools 和 Windows SDK。Rust 版本见 `rust-toolchain.toml`，rustup 会自动选择。

```sh
cargo build --release --locked
./target/release/quiet_md                          # 默认用 ~/Documents/Quiet.md
QUIET_MD_HOME=/tmp/lib ./target/release/quiet_md   # 换一个文档库
```

开发时请用 `QUIET_MD_HOME` 指向临时目录，别拿自己的真实文档库测试。

仓库是 Cargo workspace：`crates/core` 是不依赖 GPUI 的文档引擎（`fdl_core`），根目录是应用本体。GPUI 和 taffy 用的是 `vendor/` 里的本地补丁版本，改动见 [vendor/PATCHES.md](vendor/PATCHES.md)。`target/debug` 会长到十几 GB，日常只用 release 时可以 `cargo clean --profile dev` 清掉。

## 测试

```sh
scripts/check.sh                                        # 测试 + clippy 零警告，提交前跑
cargo test --release -p fdl_core                        # 只测文档引擎，不编 GPUI
FDL_FUZZ_SEEDS=2000 cargo test --release random_edits   # 更多随机编辑，查编辑内核
```

- 编辑内核有一个随机测试：从一篇包含所有块类型的文档出发，随机做选区、输入、结构、样式、表格和撤销，每一步检查模型不变量，每 25 步存盘再读回。
- 文档格式有回归测试：`crates/core/tests/fixtures/all-blocks.xml` 读入再写出必须逐字节不变。
- 界面截图和自动化脚本见 [tools/harness/README.md](tools/harness/README.md)。

想每次提交自动检查：`git config core.hooksPath scripts/hooks`。

## 文档

| 文档 | 内容 |
|---|---|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | 分层、编辑流程、不变量、撤销、渲染、模块地图 |
| [docs/FORMAT.md](docs/FORMAT.md) | 文档库目录、索引恢复、DocxXML 读写规则和本地扩展 |
| [docs/THEMES.md](docs/THEMES.md) | 文档主题的 CSS 写法：选择器、属性、命名颜色 |
| [AGENTS.md](AGENTS.md) | 代码风格、提交前检查、测试约定 |

## 构建分发包

发布前需完成 `scripts/check.sh`，并确认对应提交的 CI 通过；打包后验证签名、公证（macOS）和下载文件的 SHA-256。Windows 构建 CI 不替代安装、卸载及真实编辑验收。

macOS 最低系统声明为 13，尚未完成该版本实机验证。Intel Mac 暂无公开安装包。

### macOS

`scripts/release.sh` 在本机架构上构建，生成应用图标，使用 Developer ID Application 证书签名，提交 Apple 公证，装订票据并执行 Gatekeeper 校验，最后输出 ZIP 和 SHA-256。它不会上传 GitHub，也不会启动应用。

先在登录钥匙串中安装包含私钥的签名证书，并配置 notarytool 钥匙串 profile，再运行：

```sh
SIGNING_IDENTITY="Developer ID Application: …" NOTARY_PROFILE="quiet-md" scripts/release.sh
```

脚本遇到签名或公证失败会停止，失败目录里的中间产物不能作为正式版本分发。

### Windows

使用 Inno Setup 7。在 Visual Studio x64 开发者 PowerShell 中运行：

```powershell
# 本地验收用的未签名安装包，不会上传或发布
.\scripts\package-windows.ps1 -Iscc 'C:\path\to\ISCC.exe'

# 证书签发并通过硬件令牌或云签名客户端连接后，按指纹选择当前用户的证书
.\scripts\package-windows.ps1 -Iscc 'C:\path\to\ISCC.exe' `
  -CertificateThumbprint '<40-character certificate thumbprint>' `
  -TimestampUrl 'http://time.certum.pl'
```

签名使用 Windows SDK 的 `signtool.exe`，可用 `-SignTool` 指定路径。时间戳地址应使用证书机构提供的 RFC 3161 服务，示例为 Certum。脚本分别对程序、安装器和卸载器签名，检查发布者、可信签名和时间戳，失败即停止。最终安装包和 SHA-256 输出到 `dist/windows/signed/` 或 `dist/windows/unsigned/`，中间目录不用于发布。

安装器支持选择位置和可选的桌面快捷方式，默认只安装到当前用户的应用目录。卸载不删除文档。Windows 的文档位置是系统 Documents 目录下的 `Quiet.md`。

无论哪个平台，凭据、证书私钥和证书文件都不要提交到仓库。

### 上传和自动更新

`scripts/publish-downloads.sh <包>…` 把包上传到 `download.quiet.md` 的 `v<版本>/` 和 `latest/`，用 minisign 签名，最后写 `latest/update-<平台>.json`（`macos-arm64`、`windows-x64`）。应用读的就是这份清单：启动 10 秒后和之后每 12 小时检查一次，窗口回到前台时如果距上次检查超过一小时也会再查（设置 › 通用 可关），菜单「检查更新…」立即检查；下载要用户点，校验 minisign 签名（trusted comment 必须是包的文件名），macOS 还要求新包的签名团队和当前应用一致，然后退出、换包、重新打开。实现见 `src/update.rs`。

更新签名密钥只生成一次（`brew install minisign`），私钥放在仓库外，公钥提交：

```sh
minisign -G -f -p packaging/update-key.pub -s ~/.minisign/quiet-md.key
```

脚本会用 `packaging/update-key.pub` 校验刚签的包，公钥和私钥对不上就停止。私钥丢了只能换公钥，已安装的旧版本就收不到之后的更新了，请备份。

本地试更新流程：用 `QUIET_MD_UPDATE_URL=http://127.0.0.1:<端口>` 指向一个放着清单和包的目录，再用 `QUIET_MD_UPDATE_PUBKEY=<测试公钥>` 编译；设了这个变量时，未签名的构建也会检查更新。

发布新版本时，两个 `Cargo.toml` 的版本号一起更新，并在 `src/changelog.rs` 写下这个版本的更新要点：老用户更新后，侧栏底部会显示一次。
