# Taiwan Gov Data AI Automation Pipeline (TGD-AI)

## 1. 專案目標 (Project Objectives)
本專案旨在建立一個全自動化的數據驅動管線，結合台灣政府開放資料 (Taiwan Gov Open Data) 與先進的量子-經典混合運算 (Quantum-Classical Hybrid Computing) 技術。
* **自動化資料工程：** 透過 GitHub Actions 實現零人工干預的資料擷取與清洗，並以輕量化的 **Toon** 格式進行儲存與版本控制。
* **高效能運算：** 利用 **Rust** 的記憶體安全性與並發優勢，結合 **holyQASM** 進行量子啟發式演算法或電路模擬。
* **低延遲通訊與預測告警：** 捨棄傳統 REST 方案，全面採用 **gRPC** 框架進行微服務通訊，確保高吞吐量與低延遲，並針對異常指標實作即時自動化告警。

---

## 2. 系統架構 (System Structure)

```text
.
├── .github/
│   └── workflows/              # GitHub Actions 定期任務 (Cron 1 & 2)
├── proto/
│   └── inference.proto         # gRPC 服務定義檔 (Protocol Buffers)
├── src/
│   ├── main.rs                 # Rust 後端主程式 (gRPC Server/Client)
│   ├── data_engine/            # 資料處理與 Toon 格式解析模組
│   └── quantum_bridge/         # Rust 與 holyQASM 呼叫介面
├── qasm/
│   └── logic.qasm              # holyQASM 量子邏輯電路定義
├── models/                     # 模型配置與量化腳本
├── Cargo.toml                  # Rust 專案配置 (包含 tonic, prost 等依賴)
└── build.rs                    # Rust 編譯腳本 (用於編譯 .proto 檔)




階段一：資料獲取與雲端同步 (ETL Pipeline)
• 觸發： GitHub Action (Cron 1: 每日執行)。
• 執行：
• 由 Rust 撰寫的 data_engine 呼叫政府 API。
• 進行資料清洗、格式校驗，並將結構化資料序列化為 Toon 格式。
• 輸出： 將 .toon 資料檔推送至 Hugging Face Datasets 進行集中管理與版本控制。
階段二：混合動力訓練 (Hybrid Training)
• 執行環境： Kaggle Notebook / 自託管 GitHub Runner。
• 邏輯：
• 讀取並反序列化 Hugging Face 上的 Toon 資料集。
• 使用 Rust 進行特徵工程預處理。
• 透過 holyQASM 模擬量子退火或量子特徵對映 (Quantum Feature Map) 以增強模型表現。
• 存儲： 將訓練完成的模型權重上傳至 Hugging Face Model Hub。
階段三：推論服務與智慧告警 (Inference & Alerting)
• 託管： Hugging Face Spaces (Dockerized Runtime, 需開放 HTTP/2 支援)。
• 流程：
• Hugging Face 啟動基於 Rust tonic 框架的 gRPC Server。
• GitHub Action (Cron 2) 作為 gRPC Client，定期透過 HTTP/2 發送推論請求 (RPC Call)。
• Rust 後端執行高效能推論並回傳結果。
• 若預測結果觸發閾值，Cron 2 腳本立即透過 Telegram/Slack Webhook 發送告警。
4. 技術棧 (Tech Stack)
後端運算 (Backend & Processing)
• Rust (Core Logic): 負責資料解析、gRPC 伺服器/客戶端實作與量子電路調度。
• holyQASM (Quantum Logic): 專用於特定領域的量子運算模擬，透過 Rust FFI 進行深度整合。
網路通訊與資料格式 (Communication & Data)
• gRPC & Protocol Buffers: 利用 Rust 的 tonic 與 prost 套件實作。具備強型別合約 (.proto)、二進位傳輸與 HTTP/2 雙向串流優勢，效能遠勝傳統 REST/JSON。
• Toon Format: 作為核心的資料落地與跨節點傳遞格式，確保資料的輕量化與特定應用場景的兼容性。
機器學習與部署 (AI & Infrastructure)
• Hugging Face Hub / Spaces: 資料集 (以 Toon 儲存)、模型權重管理與 Docker 容器託管。
• GitHub Actions: 擔任全系統的調度執行器 (Orchestrator) 與 gRPC 發起端。
5. 安全性實作 (Security)
• 強型別通訊安全： gRPC 的 protobuf 合約確保了 Client 與 Server 之間的資料結構嚴格一致，防範因格式錯誤導致的執行期崩潰 (Runtime Panic)。
• 機敏資料管理： 包含政府 API Keys、gRPC 認證 Token 與 Bot Tokens，均嚴格受控於 GitHub Secrets 中。
