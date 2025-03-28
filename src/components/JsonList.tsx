import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";

// const MOC_URL = "/Documents/RustProject/tmp/sample.json";

const JsonList= () => {
    // このコンポーネントがレンダリングされた際に、invokeでバックエンドからjsonデータを読み込み
    let data = {"cat": "no_image.png", "dog": "no_image.png"};

    return (
        <div>
        {Object.entries(data).map(([key, value]) => (
            <p key={key}>
            {key}: {JSON.stringify(value)}
            </p>
        ))}
        </div>
    );
};

export default JsonList;
