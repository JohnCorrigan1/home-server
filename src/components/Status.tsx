"use client";

import { CircularProgress } from '@nextui-org/react';
import { useEffect, useState } from 'react';
import baseUrl from './BaseUrl';

export default function Status({ value, label }: StatusProps) {

    return (
        <CircularProgress aria-label={label} value={value} size="lg"
            classNames={{
                svg: 'w-24 h-24',
                value: "text-lg font-semibold"
            }}
            showValueLabel={true}
            label={label}
            color={value < 50 ? 'success' : value < 80 ? 'warning' : 'danger'}
        />
    );
}

export type StatusProps = {
    value: number;
    label: string;
};

export function CPUStatus() {
    const [value, setValue] = useState<number>(0);
    const [refresh, setRefresh] = useState<number>(0);

    useEffect(() => {
        const callCpu = async () => {
          const cpu = await fetch(`${baseUrl}/api/cpu`).then((res) => res.json());
          setValue(cpu.cpu_usage);
    }
        callCpu();
    }, [refresh]);

    return (
       <div className=" cursor-pointer" onClick={() => setRefresh(refresh + 1)}> 
        <Status value={value} label={"CPU"} />
        </div>
    );
}
