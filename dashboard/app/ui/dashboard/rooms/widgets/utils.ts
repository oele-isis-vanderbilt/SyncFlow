import type { StylesConfig, ClassNamesConfig } from 'react-select';
export const customSelectStyles: StylesConfig = {
  control: (provided) => {
    return {
      ...provided,
      backgroundColor: 'black',
      color: 'white',
    };
  },
  menu: (provided) => {
    return {
      ...provided,
      backgroundColor: 'black',
      color: 'white',
    };
  },
  option: (provided) => {
    return {
      ...provided,
      backgroundColor: 'black',
      color: 'white',
      ':hover': {
        backgroundColor: 'grey',
      },
    };
  },
  multiValue: (provided) => {
    return {
      ...provided,
      color: 'black',
    };
  },
  singleValue: (provided) => {
    return {
      ...provided,
      color: 'white',
    };
  },
};

export const customClassNames: ClassNamesConfig = {
  control: (state) =>
    state.isFocused
      ? 'border-blue-500 dark:border-blue-300 dark:bg-gray-800'
      : 'border-gray-300 dark:border-gray-700 dark:bg-gray-800',
  menu: () => 'bg-white dark:bg-gray-900 text-black dark:text-white',
  option: (state) =>
    state.isFocused
      ? 'bg-gray-100 dark:bg-gray-700 text-black dark:text-white'
      : 'bg-white dark:bg-gray-900 text-black dark:text-white',
  multiValue: () => 'bg-gray-100 dark:bg-black text-black dark:text-white',
  multiValueLabel: () => 'text-black dark:text-white',
  singleValue: () => 'text-black dark:text-white',
};
