export function sidebarToggle() {
  if (!document.body.classList.contains("sidebar-toggle-yes")) {
    document.body.classList.add("sidebar-toggle-yes");
    document.body.classList.remove("sidebar-toggle-no");
  } else {
    document.body.classList.add("sidebar-toggle-no");
    document.body.classList.remove("sidebar-toggle-yes");
  }
}
