import { mem } from 'node-os-utils';

export async function GET() {
    const memUsed = await mem.used();

    return Response.json(
        memUsed
    );
}

export type Memory = {
    totalMemory: number,
    usedMemory: number,
    memoryUsage: number
}


