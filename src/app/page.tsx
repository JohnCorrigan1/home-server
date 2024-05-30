import { NextUIProvider } from "@nextui-org/react";

import FileUpload from "../components/FileUpload";
import { CPUStatus } from "../components/Status";

export default async function Home() {

    return (
        <NextUIProvider>
            <main className="min-h-screen p-24">
                <div className="w-full flex gap-24 justify-center">
                        <CPUStatus />
                </div>
            </main>
        </NextUIProvider >
    );
}
