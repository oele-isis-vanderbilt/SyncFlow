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
        className="mb-3 mt-5 block text-xs font-medium dark:text-white"
        htmlFor={id}
      >
        {label}
      </label>
      <div className="relative">
        <input
          className="peer block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-sm text-gray-900 outline-2 placeholder:text-gray-500"
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
        className="mb-3 mt-5 block text-xs font-medium dark:text-white"
        htmlFor={id}
      >
        {label}
      </label>
      <div className="relative">
        <textarea
          className="peer block w-full rounded-md border border-gray-200 py-[9px] pl-10 text-sm text-gray-900 outline-2 placeholder:text-gray-500"
          id={id}
          name={id}
          placeholder={placeholder}
          required={required}
        />
      </div>
    </div>
  );
}
