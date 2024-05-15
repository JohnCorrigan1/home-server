import si from 'systeminformation';


export async function GET() {
    const storage = await si.fsSize();

    const drives: Drive[] = storage.filter(drive => drive.type === "ext4").map(
        (drive) => {
            return {
                size: drive.size / 1000000000,
                used: drive.used / 1000000000,
                usedPercent: drive.use,
                available: drive.available / 1000000000,
                mount: drive.mount,
                name: drive.fs,
            }
        })


    const totalStorage = drives.reduce((acc, drive) => acc + drive.size, 0);
    const totalUsed = drives.reduce((acc, drive) => acc + drive.used, 0);
    const totalAvailable = drives.reduce((acc, drive) => acc + drive.available, 0);
    const totalUsedPercent = (totalUsed / totalStorage * 100);

    const total: Drive = {
        size: totalStorage,
        used: totalUsed,
        available: totalAvailable,
        usedPercent: totalUsedPercent
    }

    return Response.json({ total, drives });

}

export type Drive = {
    size: number;
    used: number;
    usedPercent: number;
    available: number;
    mount?: string;
    name?: string;
}


export type Storage = {
    total: Drive,
    drives: Drive[]
}
