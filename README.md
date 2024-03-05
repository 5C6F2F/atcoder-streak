# AtCoder Streak

AtCoderのStreakを切らさないようにするためのアプリです。

Streakが更新されていない場合、設定した時刻にLINEで通知します。

---

PowerShell 7 以降のpwsh.exeが必要です。インストールしてパスを通しておいてください。

https://learn.microsoft.com/ja-jp/powershell/scripting/whats-new/migrating-from-windows-powershell-51-to-powershell-7

通知のためにLINE Notifyを利用します。トークンを取得しておいてください。

https://notify-bot.line.me/my/


## 使い方

ファイルを適当な場所に置き`config.toml`の次の2箇所を編集してください。

+ `[load_last_ac]`の`user_name`にAtCoderの名前を入れる
+ `[line_notifier]`の`token`にLINE Notifyのトークンを入れる

次に`registerer.exe`を起動すると、自動でタスクスケジューラに`atcoderstreak.exe`のタスクが登録されます。

タスクの名前・説明・実行時間は`config.toml`の`[registerer]`から変更できます。
