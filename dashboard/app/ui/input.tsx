import { useState } from 'react';

export function Input({
  id,
  label,
  type,
  placeholder,
  required,
  children,
  onChange,
}: {
  id: string;
  label: string;
  type: string;
  placeholder: string;
  required: boolean;
  children?: React.ReactNode;
  onChange?: (e: React.ChangeEvent<HTMLInputElement>) => void;
}) {
  return (
    <div>
      <label
        className="mt-5 mb-3 block font-medium text-xs dark:text-white"
        htmlFor={id}
      >
        {label}
      </label>
      <div className="relative">
        <input
          className="peer block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-gray-900 text-sm outline-2 placeholder:text-gray-500"
          id={id}
          type={type}
          name={id}
          placeholder={placeholder}
          required={required}
          onChange={onChange}
        />
        {children}
      </div>
    </div>
  );
}

export function TextArea({
  id,
  label,
  placeholder,
  required,
}: {
  id: string;
  label: string;
  type: string;
  placeholder: string;
  required: boolean;
}) {
  return (
    <div>
      <label
        className="mt-5 mb-3 block font-medium text-xs dark:text-white"
        htmlFor={id}
      >
        {label}
      </label>
      <div className="relative">
        <textarea
          className="peer block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-gray-900 text-sm outline-2 placeholder:text-gray-500"
          id={id}
          name={id}
          placeholder={placeholder}
          required={required}
        />
      </div>
    </div>
  );
}

export function RangeSlider({
  id,
  label,
  min,
  max,
  step,
  defaultValue,
  onChange,
}: {
  id: string;
  label: string;
  min: number;
  max: number;
  defaultValue: number;
  step: number;
  onChange: (x: number) => void;
}) {
  let [value, setValue] = useState(defaultValue);
  return (
    <div>
      <label
        className="mt-6 mb-3 block font-medium text-xs dark:text-white"
        htmlFor={id}
      >
        {label}
      </label>
      <div className="relative">
        <input
          className="peer block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-gray-900 text-sm outline-2 placeholder:text-gray-500"
          id={id}
          name={id}
          type="range"
          min={min}
          max={max}
          step={step}
          value={value}
          onChange={(event) => {
            const newValue = parseInt(event.target.value);
            setValue(newValue);
            onChange(newValue);
          }}
        />
        <div className="flex justify-between text-gray-500 text-xs">
          <span className="dark:text-white">{min}</span>
          <span className="font-bold dark:text-white">{value}</span>
          <span className="dark:text-white">{max}</span>
        </div>
      </div>
    </div>
  );
}

export function Checkbox({
  id,
  label,
  checked,
}: {
  id: string;
  label: string;
  checked: boolean;
}) {
  const [isChecked, setIsChecked] = useState(checked);

  return (
    <div className="mt-2 flex items-center">
      <input
        id={id}
        name={id}
        type="checkbox"
        className="rounded border-gray-300 text-indigo-600 shadow-sm focus:border-indigo-300 focus:ring focus:ring-indigo-200 focus:ring-opacity-50"
        checked={isChecked}
        onChange={(e) => setIsChecked(e.target.checked)}
      />
      <label
        htmlFor={id}
        className="ml-2 block text-gray-900 text-sm dark:text-white"
      >
        {label}
      </label>
    </div>
  );
}
