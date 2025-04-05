import { unwrap } from "@/common-manager";
import { getCategoryInfo } from "./get-category-info";

export function getMenuElementMatchedCategory() {
  const ul = unwrap(document.querySelector<HTMLElement>("ul.tt_category"), "ul.tt_category 요소가 없습니다.");

  const { isCategoryPath, categoryType, categoryName, subCategoryName } = getCategoryInfo();
  if (!isCategoryPath) return;

  if (categoryType === "all") {
    const linkTit = unwrap(ul.querySelector<HTMLElement>("a.link_tit"), "a.link_tit 요소가 없습니다.");
    return linkTit;
  } else if (categoryType === "category") {
    const linkItems = unwrap(ul.querySelectorAll<HTMLElement>("a.link_item"), "a.link_item 요소가 없습니다.");
    for (const item of Array.from(linkItems)) {
      const copyItem = item.cloneNode(true) as HTMLElement;
      copyItem.querySelector<HTMLElement>(".c_cnt")?.remove();
      const renderedCategoryName = copyItem.textContent?.trim() ?? "";

      if (renderedCategoryName.length >= 27) {
        if (categoryName?.startsWith(renderedCategoryName.slice(0, renderedCategoryName.length - 2))) {
          return item;
        }
      } else {
        if (renderedCategoryName === categoryName) {
          return item;
        }
      }
    }
  } else if (categoryType === "sub-categpry") {
    const linkItems = unwrap(ul.querySelectorAll<HTMLElement>("a.link_item"), "a.link_item 요소가 없습니다.");

    for (const item of Array.from(linkItems)) {
      const copyItem = item.cloneNode(true) as HTMLElement;
      copyItem.querySelector<HTMLElement>(".c_cnt")?.remove();
      const renderedParentCategoryName = copyItem.textContent?.trim() ?? "";
      const subLinkItems = unwrap(item.parentElement?.querySelectorAll<HTMLElement>("a.link_sub_item"), "a.link_sub_item 요소가 없습니다.");
      for (const item2 of Array.from(subLinkItems)) {
        const copyItem2 = item2.cloneNode(true) as HTMLElement;
        copyItem2.querySelector<HTMLElement>(".c_cnt")?.remove();
        const renderedChildCategoryName = copyItem2.textContent?.trim() ?? "";
        if (renderedParentCategoryName.length >= 27) {
          if (categoryName?.startsWith(renderedParentCategoryName.slice(0, renderedParentCategoryName.length - 2))) {
            if (renderedChildCategoryName.length >= 27) {
              if (subCategoryName?.startsWith(renderedChildCategoryName.slice(0, renderedChildCategoryName.length - 2))) {
                return item2;
              }
            } else {
              if (renderedChildCategoryName === subCategoryName) {
                return item2;
              }
            }
          }
        } else {
          if (renderedParentCategoryName === categoryName) {
            if (renderedChildCategoryName.length >= 27) {
              if (subCategoryName?.startsWith(renderedChildCategoryName.slice(0, renderedChildCategoryName.length - 2))) {
                return item2;
              }
            } else {
              if (renderedChildCategoryName === subCategoryName) {
                return item2;
              }
            }
          }
        }
      }
    }
  }
}
