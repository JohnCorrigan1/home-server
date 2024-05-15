import { CircularProgress } from '@nextui-org/react';

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
