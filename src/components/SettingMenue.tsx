import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';

interface PathData {
  [key: string]: string;
}

function App() {
  const [data, setData] = useState<PathData>({});
  const [loading, setLoading] = useState<boolean>(true);
  const [error, setError] = useState<string | null>(null);
  
  // バックエンドからデータを取得する関数
  const fetchData = async () => {
    try {
      setLoading(true);
      // バックエンドからJSONデータを取得（Tauriのinvokeを使用）
      const jsonData = await invoke<PathData>('get_json_data');
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
      await invoke('save_json_data', { jsonData: data });
      alert('データが保存されました');
    } catch (err) {
      setError('データの保存に失敗しました: ' + String(err));
      console.error('データ保存エラー:', err);
    }
  };

  // 編集ボタンのクリックハンドラ
  const handleEdit = (key: string) => {
    // 編集機能は後で別ファイルで実装される予定
    console.log(`編集: ${key}, 値: ${data[key]}`);
    // モーダルウィンドウを表示する処理（後で実装）
  };

  // 追加ボタンのクリックハンドラ
  const handleAdd = () => {
    // 追加機能は後で別ファイルで実装される予定
    console.log('データ追加');
    // モーダルウィンドウを表示する処理（後で実装）
  };

  // コンポーネントがマウントされたときにデータを取得
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
    <div className="container mx-auto p-4">
      {/* 保存ボタン */}
      <div className="mb-6">
        <button 
          onClick={saveData}
          className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        >
          保存
        </button>
      </div>

      {/* データ表示 */}
      <div className="mb-6">
        <h2 className="text-xl font-bold mb-4">データ一覧</h2>
        <div className="border rounded divide-y">
          {Object.entries(data).map(([key, value]) => (
            <div key={key} className="p-3 flex justify-between items-center">
              <div className="flex space-x-4">
                <span className="font-medium">{key}:</span>
                <span>{value}</span>
              </div>
              <button 
                onClick={() => handleEdit(key)}
                className="px-3 py-1 bg-gray-200 rounded hover:bg-gray-300"
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
          className="px-4 py-2 bg-green-500 text-white rounded hover:bg-green-600"
        >
          追加
        </button>
      </div>
    </div>
  );
}

export default App;