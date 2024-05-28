import { NextUIProvider } from "@nextui-org/react";
import Status from "../components/Status";

import type { Storage } from "../app/api/storage/route";
import type { CPU } from "../app/api/cpu/route";
import type { Memory } from "../app/api/memory/route";
import FileUpload from "../components/FileUpload";


export default async function Home() {
    

//    const baseUrl = process.env.BASE_URL || "http://localhost:3000";
    //const baseUrl = "http://192.168.86.81:3000";
    const baseUrl = "http://localhost:8000";

    const cpu: number = await fetch(`${baseUrl}/api/cpu`).then((res) => res.json()).cpuUsage;
    const memory: Memory = await fetch(`${baseUrl}/api/memory`).then((res) => res.json());
    const storage: Storage = await fetch(`${baseUrl}/api/storage`).then((res) => res.json());

    return (
        <NextUIProvider>
            <main className="min-h-screen p-24">
                <div className="w-full flex gap-24 justify-center">
                    <Status label="CPU" value={cpu.cpuUsage}
                    />
                    <Status label="Memory" value={memory.memoryUsage} />
                    <Status label="Storage" value={storage.total.usedPercent} />
                </div>
                <div className="w-full flex gap-24 justify-center">
                    <FileUpload />
                </div>
            </main>
        </NextUIProvider >
    );
}
