# AtCoder Streak

AtCoderのStreakを切らさないようにするためのアプリです。

Streakが更新されていない場合、設定した時刻にDiscordで通知します。

---

PowerShell 7 以降のpwsh.exeが必要です。インストールしてパスを通しておいてください。

[Windows PowerShell 5.1 から PowerShell 7 への移行](https://learn.microsoft.com/ja-jp/powershell/scripting/whats-new/migrating-from-windows-powershell-51-to-powershell-7)

通知のためにDiscord APIを利用します。トークンを取得しておいてください。

[Discord開発者向けページ](https://discord.com/developers/applications)

## 使い方

ファイルを適当な場所に置き`config.toml`の次の2箇所を編集してください。

+ `[load_last_ac]`の`user_name`にAtCoderの名前を入力
+ `[discord_notifier]`の`token`にDiscord APIのトークンを入力

次に`registerer.exe`を起動すると、自動でタスクスケジューラに`atcoderstreak.exe`のタスクが登録されます。

タスクの名前・説明・実行時間は`config.toml`の`[registerer]`から変更できます。
