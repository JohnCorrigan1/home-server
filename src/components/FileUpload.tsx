"use client";
import { useState } from "react";
import { invoke } from "@tauri-apps/api/tauri";
import { open } from "@tauri-apps/api/dialog";
import { fs } from "@tauri-apps/api/fs";
import { dialog } from "@tauri-apps/api";
import { Button } from "@nextui-org/react";

export default function FileUpload() {
    const [file, setFile] = useState<string | null>(null);

    const handleClick = async () => {
        const fileName = await dialog.open({ directory: false, multiple: false, filter: "*" });
        setFile(fileName);
    };

  return (
      <>
    <Button onClick={handleClick}>Upload</Button>
    {file && <p className="text-zin-200">{file}</p>}
    </>
  );
}
