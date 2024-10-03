export function getPathname() {
  return decodeURIComponent(location.pathname.trim());
}

export function getCategoryPathnameInfo() {
  const pathname = getPathname();
  let isCategoryPath = pathname.startsWith("/category");

  let categoryType: "all" | "category" | "sub-categpry" | undefined;

  let categoryName: string | undefined;
  let subCategoryName: string | undefined;

  if (isCategoryPath) {
    const urlSplit = pathname.split("/"); // ex) ['', 'category', '1', '2'];
    if (urlSplit.length === 4) {
      categoryType = "sub-categpry";
      categoryName = urlSplit[urlSplit.length - 2];
      subCategoryName = urlSplit[urlSplit.length - 1];
    } else if (urlSplit.length === 3) {
      categoryType = "category";
      categoryName = urlSplit[urlSplit.length - 1];
    } else if (urlSplit.length === 2) {
      categoryType = "all";
    }
  }

  return {
    isCategoryPath,
    categoryType,
    categoryName,
    subCategoryName,
  };
}

export function getCurrentCategoryUrlMatchedMenuElement() {
  const ul = document.querySelector<HTMLElement>("ul.tt_category");
  if (ul === null) return null;

  const { isCategoryPath, categoryType, categoryName, subCategoryName } = getCategoryPathnameInfo();

  if (!isCategoryPath) return null;

  let target: HTMLElement | null = null;

  switch (categoryType) {
    case "all":
      target = ul.querySelector<HTMLElement>(".link_tit");
      break;
    case "category":
      {
        const linkItems = ul.querySelectorAll<HTMLElement>("a.link_item");
        linkItems.forEach((item) => {
          const html = item.innerHTML.replace("\n", "");
          const htmlSplit = html.split('<span class="').map((x) => x.trim());

          if (htmlSplit[0] === categoryName) {
            target = item;
          }
        });
      }
      break;
    case "sub-categpry":
      {
        const linkItems = ul.querySelectorAll<HTMLElement>("a.link_item");
        linkItems.forEach((item) => {
          const html = item.innerHTML.replace("\n", "");
          const htmlSplit = html.split('<span class="').map((x) => x.trim());
          const parentCategoryName = htmlSplit[0];

          const subLinkItems = item.parentElement?.querySelectorAll<HTMLElement>("a.link_sub_item");
          subLinkItems?.forEach((item) => {
            const html = item.innerHTML.replace("\n", "");
            const htmlSplit = html.split('<span class="').map((x) => x.trim());
            const childCategoryName = htmlSplit[0];

            if (parentCategoryName === categoryName && childCategoryName === subCategoryName) {
              target = item;
            }
          });
        });
      }
      break;
  }

  return target;
}

export function isAdmin() {
  if (window.tiara !== undefined) {
    if (window.tiara.customProps?.role === "owner") {
      return true;
    }
  }
  return false;
}
