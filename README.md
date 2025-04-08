# 動物のspellをtypeすると其胴部tの画像が表示される

## 現状
- Keyの読み取り開始、停止


# ToDo:
- バック
    - 押したkeyをカウントして保存していく(判定するために、取り出しやすい構造にする)
        - jsonの文字列から書く文字列を文字に分割したhash map作って、その文字ごとにcountを載せていく？
        - 考慮事項 : 
            hash mapのkey検索は遅い？
            文字列が多くなると探索が膨大になる？
    - 全てのスペルが押されたら、画像を表示
    - フロント側からの、画像/文字列のCRUDの処理
    - コントロールセンターにメニューを追加する [https://zenn.dev/aidemy/articles/8d8e406967d386](https://zenn.dev/aidemy/articles/8d8e406967d386)
    - エラーハンドリングでpanicさせない
- フロント
    - 設定から文字列と画像の登録、削除、変更


## これから
- splellの順序は考慮するか
- 動物が走り抜けるようにしたい、どうやって実装する？

