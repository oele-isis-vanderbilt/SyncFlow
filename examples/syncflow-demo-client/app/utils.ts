import type { StylesConfig } from 'react-select';
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
