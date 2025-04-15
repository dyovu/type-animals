## 登録した動物のspellをtypeするとその動物の画像が表示される



## ToDo:
- バック
    - vecdequeの条件を満たしたら、jsonからスペルのkeyに対応するpathを返す

    - 一番最初の文字を押した後に次の文字の判定に行かない
    あ....この関数内でhash map初期化してるからやん
    - startが押された時にスペルと画像pathのJSONを読み込み、keyをカウントするためのhash mapを作成
    - 全てのスペルが押されたら、画像を表示
    - フロント側からの、画像/文字列のCRUDの処理
    - コントロールセンターにメニューを追加する [https://zenn.dev/aidemy/articles/8d8e406967d386](https://zenn.dev/aidemy/articles/8d8e406967d386)
    - エラーハンドリングでpanicさせない
- フロント
    - 設定から文字列と画像の登録、削除、変更


### これから
- 動物が走り抜けるようにしたい、どうやって実装する？
    - モーション作って流す的な？

### 実装概要
- rdevをもちいて実装したkey取得のバイナリを呼び出して、key入力を待受開始、停止
- 起動時にスペルと画像を保持したJSONをアプリケーション固有のフォルダに作成、環境変数を設定
- 受け取ったkeyの判定

