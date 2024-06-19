"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

export default function FileUpload() {
  const [file, setFile] = useState<
    string | string[] | null
  >(null);
  const [progress, setProgress] = useState<any>(0);
  const [speed, setSpeed] = useState(0);
  const [elapsedTime, setElapsedTime] = useState(0);

  const selectFile = async () => {
    const file = await open({
      directory: false,
      multiple: true,
    });
    setFile(file[0]);
  };

  const uploadFile = async () => {
    if (!file) return;
    const listener = listen("upload-progress", (event) => {
      setProgress(event.payload[0]);
      setSpeed(event.payload[1]);
      setElapsedTime(event.payload[2]);
    });
    invoke<string>("upload_file", {
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
      <div className="mt-4">
        <progress
          className="w-full"
          value={progress}
          max="100"></progress>
        <p>
          {progress.toFixed(2)}% - {speed.toFixed(2)} MB/s
        </p>
        <p>Seconds: {elapsedTime.toFixed(2)}s</p>
      </div>
    </div>
  );
}
