"use client";

import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { useState, useEffect } from "react";
import { listen } from "@tauri-apps/api/event";

export default function UploadStream() {
  const [file, setFile] = useState<any>(null);
  const [fileType, setFileType] = useState<number>(1);
  const [show, setShow] = useState<string>("");

  const selectFile = async () => {
    setFile(null);

    const file = await open({
      directory: false,
      multiple: true,
    });
    setFile(file);
  };

  const uploadFile = async () => {
    if (!file) return;
    invoke<string[]>("file_stream", {
      filePaths: file,
      fileType: fileType,
      showName: show,
    }).then((response) => {
      console.log("response", response);
    });
  };

  return (
    <div className="pt-10">
      <div className="flex gap-10">
        <button
          className="py-3 px-2 bg-white text-black rounded-lg"
          onClick={selectFile}
        >
          Select file
        </button>
        {file && (
          <>
            <button
              className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
              onClick={uploadFile}
            >
              Upload
            </button>
            <select
              className="py-3 px-2 bg-white text-black rounded-lg"
              onChange={(e) => setFileType(parseInt(e.target.value))}
              value={fileType}
            >
              <option value="1">Movie</option>
              <option value="2">Show</option>
              <option value="3">Image</option>
              <option value="4">Document</option>
            </select>
            {fileType == 2 && (
              <input
                type="text"
                placeholder="Show Name"
                className="py-3 px-2 bg-white text-black rounded-lg"
                onChange={(e) => setShow(e.target.value)}
              />
            )}
          </>
        )}
      </div>
      {file &&
        file.map((f, index) => (
          <div className="mt-4">
            <FileProgress key={index} index={index} fileName={f} />
          </div>
        ))}
    </div>
  );
}

type FileProgressProps = {
  index: number;
  fileName: string;
};

export function FileProgress({ index, fileName }: FileProgressProps) {
  const [progress, setProgress] = useState<number>(0);
  const [speed, setSpeed] = useState<number>(0);
  const [elapsedTime, setElapsedTime] = useState<number>(0);
  const [timeRemaining, setTimeRemaining] = useState<number>(0);

  useEffect(() => {
    listen("upload-progress-" + index, (event) => {
      setProgress(event.payload[0]);
      setSpeed(event.payload[1]);
      setElapsedTime(event.payload[2]);
      setTimeRemaining(event.payload[3]);
    });
  }, []);
  return (
    <div className="mt-4">
      <>
        <h2>{fileName.split("/").findLast((e) => e != null)}</h2>
        <progress className="w-full" value={progress || 0} max="100"></progress>
        <p>
          {progress?.toFixed(2)}% - {speed?.toFixed(2)} MB/s
        </p>
        <p>Seconds: {elapsedTime?.toFixed(2)}s</p>
        <p>Eta: {timeRemaining?.toFixed(2)}</p>
      </>
    </div>
  );
}
