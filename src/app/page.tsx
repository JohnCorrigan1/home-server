import { NextUIProvider } from "@nextui-org/react";

import { CPUStatus, DiskStatus, MemoryStatus } from "@/components/Status";
import FileUpload from "@components/FileUpload";
import UploadStream from "@components/UploadStream";
import ListDirectories from "@components/ListDirectories";

export default async function Home() {
  return (
    <NextUIProvider>
      <main className="min-h-screen p-24 bg-black">
        <div className="w-full flex gap-24 justify-center">
          <CPUStatus />
          <MemoryStatus />
          <DiskStatus />
        </div>
        {/* <FileUpload /> */}
        <UploadStream />
        <ListDirectories />
      </main>
    </NextUIProvider>
  );
}
