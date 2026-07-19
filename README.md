# ExtraVR

透過 Wi-Fi 把 PC 上的 VR 內容串流到獨立式頭戴裝置。

ExtraVR 是 [ALVR](https://github.com/alvr-org/ALVR) 的衍生作品（MIT 授權），在其基礎上加入繁體中文介面、重新設計的儀表板、瀏覽器監控頁與 Unity 整合用的 TCP 通道，並以單一 Windows 安裝檔散佈。

## 與上游 ALVR 的差異

| 項目 | 說明 |
| --- | --- |
| 介面 | 深色 tech 風格主題（電光青 accent）、緊湊化的分頁配置 |
| 語言 | 儀表板文字繁體中文化，於 render layer 套用，不動 `settings.rs` 與 preset 比對邏輯 |
| 字型 | 自動載入系統 CJK 字型，中文不再顯示為方框 |
| 監控頁 | 串流端內建網頁監控，任何裝置皆可開啟 `http://<PC IP>:<web_server_port>/monitor` |
| Unity 整合 | TCP passthrough 通道，連接埠可於設定中調整（預設 9001） |
| 散佈方式 | Inno Setup 安裝檔，安裝時自動註冊 SteamVR 驅動、設定防火牆與寫入權限 |

## 相容性

| 頭戴裝置 | 支援 |
| :--- | :---: |
| Quest 1/2/3/3S/Pro | ✔ |
| Pico Neo 3/4/4 Ultra | ✔ |
| Play For Dream YVR 1/2/MR | ✔ |
| Vive Focus 3/Vision/XR Elite | ✔ |
| Lynx R1 | ✔ |
| Apple Vision Pro | ✔ |
| Oculus Go | ✘ |

| PC 作業系統 | 支援 |
| :--- | :---: |
| Windows 10/11 | ✔ |
| Linux | 未測試（上游為 beta） |
| macOS | ✘ |

裝置支援狀況繼承自上游 ALVR；ExtraVR 目前只針對 Windows 串流端與 Android 用戶端進行建置與測試。

### 硬體需求

- 支援的獨立式 VR 頭戴裝置
- SteamVR
- 支援 NVENC 的 NVIDIA GPU（GTX 1000 系列以上）或支援 AMF VCE 的 AMD GPU，並安裝最新驅動
  - 筆電請將 ExtraVR 與 SteamVR 指定為使用獨立顯卡
- 802.11ac 5GHz 無線網路；PC 端建議使用有線連接，且與頭戴裝置位於同一個路由器下

## 安裝

從 Releases 頁面下載 `ExtraVR-Setup-<版本>.exe` 並執行。安裝程式會自動：

- 將程式安裝到 `Program Files\ExtraVR`
- 授予安裝資料夾寫入權限（`session.json` 與 log 存放於此，SteamVR 以非提權身分執行）
- 為 SteamVR 的 `vrserver.exe` 加入輸入防火牆規則
- 以 `vrpathreg` 註冊 SteamVR 驅動

解除安裝時會自動反註冊驅動並移除防火牆規則。

## 從原始碼建置

需要 Windows、Rust stable 與系統管理員權限：

```powershell
.\build_streamer.ps1     # PC 串流端（SteamVR 驅動 + 儀表板）
.\build_android.ps1      # Android 用戶端
```

產物位於 `build\alvr_streamer_windows`。要打包安裝檔：

```powershell
& "C:\Program Files (x86)\Inno Setup 6\ISCC.exe" /DAppVersion=1.0.0 installer\ExtraVR.iss
```

發佈流程另有 GitHub Actions workflow（`Create release`，手動觸發），會產生草稿 release 並附上安裝檔與 APK。

## 與上游同步

```bash
git fetch upstream
git merge upstream/master
```

`upstream` 指向 [alvr-org/ALVR](https://github.com/alvr-org/ALVR)，`origin` 為本專案。

## 授權

ExtraVR 以 [MIT License](LICENSE) 授權，並保留上游 ALVR（polygraphene、alvr-org）的版權聲明。

## 隱私

ExtraVR 不主動蒐集任何資料。
