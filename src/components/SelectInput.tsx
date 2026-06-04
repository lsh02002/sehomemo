import React from "react";

export type Option = {
  label: string;
  value: string;
  disabled?: boolean;
};

const SelectInput = ({
  disabled,
  name,
  title,
  value,
  setValue,
  options,
  placeholder,
}: {
  disabled?: boolean;
  name: string;
  title: string;
  value: string;
  setValue: (v: string) => void;
  options: Option[];
  placeholder?: string;
}) => {
  return (
    <div className="w-100 mb-3 bg-transparent">
      <label htmlFor={name} className="form-label fw-semibold">
        {title}
      </label>
      <select
        value={value ?? ""}
        onChange={(e) => setValue(e.target.value)}
        className="form-select"
      >
        <option value="">폴더를 선택하세요</option>

        {options.map((opt) => (
          <option key={opt.value} value={opt.value} disabled={opt.disabled}>
            {opt.label}
          </option>
        ))}
      </select>
    </div>
  );
};

export default SelectInput;
