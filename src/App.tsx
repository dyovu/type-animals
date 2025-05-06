import { useState } from "react";
import { invoke } from "@tauri-apps/api/core";

import Settings from "./components/SettingButton.tsx";
import SettingMenue from "./components/SettingMenue.tsx";
import "./App.css";


// 三項演算子でstart画面とsetting画面を分ける
// setting画面の方は、全てをまとめてcomponentにする
function App() {
  const [isSetting, setIsSetting] = useState(true);

  return (
    <main className="container">
      <h3>Type anymals</h3>
      {isSetting ? (
        <div className="button-container">
          <button onClick={() => invoke('start_process')}> Start </button>
          <button onClick={() => invoke('stop_listening')}> Stop </button>
          <Settings isSetting={isSetting} setIsSetting={setIsSetting} />
        </div>
      ):(
        // ここに動物のリストを表示するコンポーネントを持ってくる
        <div>
          <button onClick={() => invoke('my_custom_command', { arg: 'Button 2' })}> Save</button>
          <SettingMenue />
        </div>
      )}
      
    </main>
  );
}

export default App;
