import { useEffect, useState, Dispatch, SetStateAction } from 'react';
import { invoke } from '@tauri-apps/api/core';

import "../styles/settingMenue.css";

interface PathData {
  // 任意の文字列をkeyとして持つ [key: string]
  [key: string]: string;
}

type isSettingProps = {
    isSetting: boolean;
    setIsSetting: Dispatch<SetStateAction<boolean>>;
};

function SettingMenue({ isSetting, setIsSetting }: isSettingProps) {
  const [data, setData] = useState<PathData>({});
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null); // ユニオン型、どちらかの方のみ受け付ける
  
  // バックエンドからデータを取得する関数
  const fetchData = async () => {
    try {
      setLoading(true);
      // invoke関数が返す値がPathData型であることを指定
      const jsonData = await invoke<PathData>('fetch_json_data');
      setData(jsonData);
    } catch (err) {
      setError('データの取得に失敗しました: ' + String(err));
      console.error('データ取得エラー:', err);
    } finally {
      setLoading(false);
    }
  };

  // データを保存する関数
  const saveData = async () => {
    try {
      // データをJSONとしてバックエンドに送信
      await invoke('post_json_data', {jsonData: data});
      setIsSetting(!isSetting);
      alert('データが保存されました');
    } catch (err) {
      setError('データの保存に失敗しました: ' + String(err));
      console.error('データ保存エラー:', err);
    }
  };

  // 編集ボタンのクリックハンドラ
  const handleEdit = (key: string) => {
    console.log(`編集: ${key}, 値: ${data[key]}`);
    // モーダルウィンドウを表示する処理（後で実装）
  };

  // 追加ボタンのクリックハンドラ
  const handleAdd = () => {
    console.log('データ追加');
    // モーダルウィンドウを表示する処理（後で実装）
  };

  // コンポーネントがマウントされたときにjsonデータをfetchする
  useEffect(() => {
    fetchData();
  }, []);

  if (loading) {
    return <div>データを読み込み中...</div>;
  }

  if (error) {
    return <div>エラー: {error}</div>;
  }

  return (
    <div className="wrap">
      {/* 保存ボタン */}
      <div className="top">
        <button 
          onClick={saveData}
          className="btn"
        >
          保存
        </button>
      </div>

      {/* データ表示 */}
      <div className="main">
        <h2 className="title">データ一覧</h2>
        <div className="list">
          {Object.entries(data).map(([key, value]) => (
            <div key={key} className="row">
              <div className="content">
                <span className="key">{key}:</span>
                <span className="val">{value}</span>
              </div>
              <button 
                onClick={() => handleEdit(key)}
                className="btn"
              >
                編集
              </button>
            </div>
          ))}
        </div>
      </div>

      {/* 追加ボタン */}
      <div>
        <button 
          onClick={handleAdd}
          className="btn"
        >
          追加
        </button>
      </div>
    </div>
  );
}

export default SettingMenue;