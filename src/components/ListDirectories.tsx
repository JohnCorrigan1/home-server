"use client";

import { useEffect, useState } from "react";
import baseUrl from "./BaseUrl";

export default function ListDirectories() {
  const [resData, setResData] = useState<Directory>();

  useEffect(() => {
    getDirs();
  }, []);

  const getDirs = async () => {
    const url = `${baseUrl}/api/directory`;
    const response = await fetch(url);
    const data: Directory = await response.json();
    // setData(await response.json());
    // console.log(data);
    setResData(data);
    console.log(resData);
  };

  //format is file system each directory can have files or more directories top directory
  //is the root directory

  return (
    <div className="text-white">
      <h1>Directories</h1>

      {resData?.directories.map((dir) => (
        <div>
          <div className="flex justify-between">
            <h2 className="text-pink-500">{dir.name}</h2>
            <p>{(dir.total_size / 1024 / 1024 / 1024).toFixed(2)}GB</p>
          </div>
          {dir.files.map((file) => (
            <div className="pl-10 flex justify-between">
              <p>{file.name}</p>
              <p>{(file.size / 1024 / 1024 / 1024).toFixed(2)}GB</p>
            </div>
          ))}
        </div>
      ))}
    </div>
  );
}

type Directory = {
  name: string;
  total_size: number;
  file_count: number;
  directories: Directory[];
  files: File[];
};

type File = {
  name: string;
  size: number;
};
