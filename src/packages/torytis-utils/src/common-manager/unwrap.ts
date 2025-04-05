export function unwrap<T>(value: T | null | undefined, errorMessage: string): T {
  if (value === null) {
    throw new Error(errorMessage);
  }
  if (value === undefined) {
    throw new Error(errorMessage);
  }
  return value;
}
