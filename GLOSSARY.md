# 從零開始學 Rust

寫給會 Go 的 Rust 新手的繁體中文 Rust 教學書；本檔統一書籍編排、AI 助教規格與書中 Rust 術語的用法。

## 書籍結構

**部**：
全書最上層的分組；第一部是語言基礎（只用 std），第二部是並行、async、進階主題與實戰（開放 crate）。
_Avoid_: 篇、卷

**章**：
一個主題的集合，例如「第 3 章 所有權」。
_Avoid_: 單元、課

**集**：
一個檔案，只教一個小概念，固定包含「本集目標」「正文」「重點整理」。
_Avoid_: 節、課、lesson

**附錄**：
屬於主線閱讀順序的補充章節；附錄一要在進入第二部之前讀完。

**跟 Go 比一比**：
集裡選用的小節，只在 Rust 跟 Go 不一樣、容易踩坑時出現。
_Avoid_: Go 對照、給 Gopher 的提醒

## 教學規格

**固定句型**：
書中要求讀者先照抄、之後才解釋原理的寫法，例如讀取輸入的 `read_line` 片段。
_Avoid_: 樣板、boilerplate

**固定題庫**：
`EXERCISES.md` 裡預先寫好的練習題，只涵蓋第 3、4 章。

**臨時題**：
AI 助教依讀者目前進度現場出的題目，必須明說不是固定題庫裡的題目。

## Rust 術語

原則：跟 Rust 關鍵字或生態系名稱相同的詞保留英文；其他翻成台灣用語，第一次出現時附英文。

保留英文：`trait`、trait bound、newtype、crate、`struct`、`enum`、`match`、slice、`Future`、`async`、variant、move、drop、adapter、consumer、Cargo 與它的指令名稱。

| 書中用語 | 英文 | 不要用 |
| --- | --- | --- |
| 所有權 | ownership | 擁有權 |
| 擁有者 | owner | 所有者、持有者 |
| 作用域 | scope | 域 |
| 借用 | borrowing | 租借 |
| 參照 | reference | 引用 |
| 可變參照 | mutable reference | 可變引用 |
| 解參照 | dereference | 解引用 |
| 生命週期 | lifetime | 生命周期、壽命 |
| 閉包 | closure | 閉合 |
| 擷取 | capture | 捕獲、捕捉 |
| 組合子 | combinator | 組合器 |
| 惰性 | lazy | 懶惰、延遲 |
| 模式 | pattern | 樣式 |
| 模式比對 | pattern matching | 模式匹配 |
| 解構 | destructuring | 解包、拆包 |
| 綁定 | binding | 繫結、捆綁 |
| 窮盡 | exhaustive | 詳盡、窮舉 |
| 分支（`match` 的） | arm | 臂 |
| 巨集 | macro | 宏 |
| 智慧指標 | smart pointer | 智能指針 |
| 泛型 | generics | 範型 |
| 模組 | module | 模塊 |
| 函式 | function | 函數 |
| 型別 | type | 類型 |
| 變數 | variable | 變量 |
| 不可變 | immutable | 唯讀 |
| 執行緒 | thread | 線程 |
| 並行 | concurrency | 並發 |
| 迭代器 | iterator | 迭代子 |
| 字元 | character | 字符 |
| 位元組 | byte | 字節 |
| 溢位 | overflow | 溢出 |
| 預設 | default | 默認 |
| 複本 | copy | 拷貝 |
| 程式 | program | 程序 |
| 堆疊（stack） | stack | 棧 |
| 堆積（heap） | heap | 堆 |
| 懸置參照 | dangling reference | 懸垂引用、野指標 |
| 內部可變性 | interior mutability | 內部可變 |
| 關聯函式 | associated function | 靜態方法 |
| 關聯型別 | associated type | 相關類型 |
| 預設方法 | default method | 預設實作 |
| 覆寫 | override | 重寫、覆蓋 |
| 型別參數 | type parameter | 類型參數 |
| trait 物件 | trait object | 特徵物件 |
| 孤兒規則 | orphan rule | 孤立規則 |
