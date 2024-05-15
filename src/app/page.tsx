import { NextUIProvider } from "@nextui-org/react";
import Status from "../components/Status";
import type { Storage } from "../app/api/storage/route";
import type { CPU } from "../app/api/cpu/route";
import type { Memory } from "../app/api/memory/route";

export default async function Home() {


    const cpu: CPU = await fetch(`${process.env.BASE_URL}/api/cpu`).then((res) => res.json());
    const memory: Memory = await fetch(`${process.env.BASE_URL}/api/memory`).then((res) => res.json());
    const storage: Storage = await fetch(`${process.env.BASE_URL}/api/storage`).then((res) => res.json());

    return (
        <NextUIProvider>
            <main className="min-h-screen p-24">
                <div className="w-full flex gap-24 justify-center">
                    <Status label="CPU" value={cpu.cpuUsage}
                    />
                    <Status label="Memory" value={memory.totalMemMb / memory.usedMemMb} />
                    <Status label="Storage" value={storage.total.usedPercent} />
                </div>
            </main>
        </NextUIProvider >
    );
}
