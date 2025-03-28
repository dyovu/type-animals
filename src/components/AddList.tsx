import { useState, useEffect } from "react";
import { invoke } from "@tauri-apps/api/core";


const AddList = () => {

    return (
        <div>
            <form>
                <label>
                    Name:
                    <input type="text" name="name" />
                </label>
                    <input type="submit" value="Submit" />
            </form>
        </div>
    );
}

export default AddList;


