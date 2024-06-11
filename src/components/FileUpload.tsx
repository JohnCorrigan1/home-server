"use client";
import { useState } from "react";
/* "use client";
 * import { useState } from "react";
 * import { invoke } from "@tauri-apps/api/tauri";
 * import { open } from "@tauri-apps/api/dialog";
 * import { dialog } from "@tauri-apps/api";
 * import { Button } from "@nextui-org/react";
 *
 * export default function FileUpload() {
 *     const [file, setFile] = useState<string | string[] | null>(null);
 *
 *     const handleClick = async () => {
 *         const fileName = await dialog.open({
 *             directory: false,
 *             multiple: false,
 *             defaultPath: "~/Downloads",
 *             filters:  [
 *                 { name: 'Images', extensions: ['jpg', 'png', 'gif'] },
 *                 { name: 'Movies', extensions: ['mkv', 'avi', 'mp4'] },
 *             ]
 *
 *     });
 *         setFile(fileName);
 *     };
 *
 *     return (
 *         <>
 *             <Button onClick={handleClick}>Upload</Button>
 *             {file && <p className="text-zin-200">{file}</p>}
 *         </>
 *     );
 * } */
/* import baseUrl from "./BaseUrl";
 *
 * export default function FileUpload() {
 *     return (
 *         <form action={`${baseUrl}/api/upload`} method="post" encType="multipart/form-data">
 *             <input type="file" name="image" />
 *             <button type="submit">Upload Image</button>
 *         </form>
 *
 *     )
 * } */
import baseUrl from "./BaseUrl";

export default function FileUpload() {
  const [progress, setProgress] = useState(0);
  const [speed, setSpeed] = useState("");
  const [loaded, setLoaded] = useState(0);
  const [total, setTotal] = useState(0);

  const submitHandler = async (
    e: React.FormEvent<HTMLFormElement>
  ) => {
    e.preventDefault();
    const xhr = new XMLHttpRequest();
    const formData = new FormData();
    const file = (e.target as HTMLFormElement).image
      .files[0];
    formData.append("image", file);
    xhr.open("POST", `${baseUrl}/api/upload`, true);
    xhr.upload.onprogress = (e) => {
      setProgress((e.loaded / e.total) * 100);
      const speed =
        e.loaded / 1024 ** 2 / (e.timeStamp / 1000);
      setSpeed(`${speed.toFixed(2)} MB/s`);
      setLoaded(e.loaded);
      setTotal(e.total);
    };
    xhr.send(formData);
  };

  return (
    <div className="flex flex-col gap-10">
      <form onSubmit={submitHandler}>
        <input type="file" name="image" />
        <button type="submit">Upload Image</button>
      </form>
      <h1 className="text-zinc-200 text-xl font-semibold">
        Speed: {speed}
      </h1>
      <h1 className="text-zinc-200 text-xl font-semibold">
        Progress: {progress}%
      </h1>
      <h1 className="text-zinc-200 text-xl font-semibold">
        Loaded: {loaded / 1024 ** 2} MB
      </h1>
      <h1 className="text-zinc-200 text-xl font-semibold">
        loaded: {loaded}
      </h1>
      <h1 className="text-zinc-200 text-xl font-semibold">
        Total: {total / 1024 ** 2} MB
      </h1>
      <h1 className="text-zinc-200 text-xl font-semibold">
        Total: {total}
      </h1>
    </div>
  );
}
