import { getPathname } from "@/path-manager";

export type CategoryType = "all" | "category" | "sub-categpry" | undefined;

export function getCategoryInfo() {
  const pathname = getPathname();
  const isCategoryPath = pathname.startsWith("/category");

  let categoryType: CategoryType = undefined;
  let categoryName: string | undefined = undefined;
  let subCategoryName: string | undefined = undefined;

  if (isCategoryPath) {
    const urlSplit = pathname.split("/"); // ex) ['', 'category', '1', '2'];
    if (urlSplit.length === 4) {
      categoryType = "sub-categpry";
      categoryName = decodeURIComponent(urlSplit[urlSplit.length - 2]);
      subCategoryName = decodeURIComponent(urlSplit[urlSplit.length - 1]);
    } else if (urlSplit.length === 3) {
      categoryType = "category";
      categoryName = decodeURIComponent(urlSplit[urlSplit.length - 1]);
    } else if (urlSplit.length === 2) {
      categoryType = "all";
    }
  }

  return {
    categoryType,
    categoryName,
    subCategoryName,
    isCategoryPath,
  };
}
