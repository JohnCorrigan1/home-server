"use client";

export default function Button() {
    const baseUrl = "http://192.168.86.81:8000";
    const callCpu = async () => {
        const cpu: CPU = await fetch(`${baseUrl}/api/cpu`).then((res) => res.json());
        console.log("cpu", cpu.cpu_usage);
    }
    return (
        <button
            className="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded"
            onClick={callCpu}
        >call me
        </button>
    );
}
