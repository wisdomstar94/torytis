/// <reference types="@wisdomstar94/torytis" />

export function isAdmin() {
  if (window.tiara !== undefined) {
    if (window.tiara.customProps?.role === "owner") {
      return true;
    }
  }
  return false;
}
