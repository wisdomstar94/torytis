import plugin from "tailwindcss/plugin";

const basicPlugin = plugin.withOptions(function () {
  return function ({ matchVariant }) {
    matchVariant("other-hover", (value, extra) => {
      return [`.other\\/${value}:hover ~ * &`, `.other\\/${value}:hover ~ &`];
    });

    matchVariant("other-checked", (value, extra) => {
      return [`.other\\/${value}:checked ~ * &`, `.other\\/${value}:checked ~ &`];
    });

    matchVariant("other-exist", (value, extra) => {
      return [`.other\\/${value} ~ * &`, `.other\\/${value} ~ &`];
    });

    matchVariant("other-has", (value, extra) => {
      return [`.other\\/${extra.modifier}:has(${value}) ~ * &`, `.other\\/${extra.modifier}:has(${value}) ~ &`];
    });
  };
});

export default basicPlugin;
