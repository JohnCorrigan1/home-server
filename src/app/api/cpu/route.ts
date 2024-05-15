import { cpu } from 'node-os-utils';


export async function GET() {

    const cpuUsage = await cpu.usage();
    const cpuModel = cpu.model();

    return Response.json({
        cpuUsage,
        cpuModel,
    });
}

export type CPU = {
    cpuUsage: number,
    cpuModel: string
}
