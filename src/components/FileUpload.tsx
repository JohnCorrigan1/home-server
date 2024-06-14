"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { useState } from "react";

export default function FileUpload() {
  const [file, setFile] = useState<
    string | string[] | null
  >(null);

  const selectFile = async () => {
    const file = await open({
      directory: false,
      multiple: true,
    });
    console.log(file);
    setFile(file);
  };

  const uploadFile = async () => {
    if (!file) return;
    console.log("Uploading file", file);
    invoke<string[]>("upload_file", {
      filePath: file,
    }).then((response) => {
      console.log("response", response);
    });
  };

  return (
    <div>
      <button onClick={selectFile}>Select file</button>
      {file && <p>{file}</p>}

      <button
        className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
        onClick={uploadFile}>
        Upload
      </button>
    </div>
  );
}
