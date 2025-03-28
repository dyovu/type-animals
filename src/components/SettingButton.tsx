import { Dispatch, SetStateAction } from "react";
import { invoke } from "@tauri-apps/api/core";

type isSettingProps = {
    isSetting: boolean;
    setIsSetting: Dispatch<SetStateAction<boolean>>;
};

/*
    * invokeで画像サイズの変更をする
*/
const Settings = ({ isSetting, setIsSetting }: isSettingProps) => {
    const handleClick = () => setIsSetting(!isSetting);

    return (
        <button onClick={
            () => {
                handleClick();
                invoke('my_custom_command', { arg: 'Button 3' });
            }
        }>Settings</button>
    );
}

export default Settings;