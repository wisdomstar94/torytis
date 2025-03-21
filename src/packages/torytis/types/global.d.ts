/// <reference types="react" />

declare namespace React.JSX {
  interface IntrinsicElements {
    s_t3: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_sidebar: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_sidebar_element: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rctrp_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_search: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rct_notice: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rct_notice_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_article_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_index_article_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_permalink_article_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_article_rep_thumbnail: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_tag_label: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp_container: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp2_container: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp2_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp_input_form: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp_guest: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp_member: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_notice_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_notice_rep_thumbnail: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_article_protected: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest_container: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest_reply_container: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest_reply_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest_input_form: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest_form: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_guest_member: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_tag: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_tag_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_paging: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_paging_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_ad_div: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_article_prev: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_article_prev_thumbnail: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_article_next: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_article_next_thumbnail: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rp_count: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;

    s_cover_group: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_cover_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_cover: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_cover_item: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_cover_item_thumbnail: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_cover_item_article_info: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_cover_item_not_article_info: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_cover_url: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;

    s_list: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_list_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_list_empty: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;

    s_rctps_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
    s_rctps_rep_thumbnail: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;

    s_page_rep: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;

    tt_html_comment: React.DetailedHTMLProps<React.HTMLAttributes<HTMLElement>, HTMLElement>;
  }
}

declare namespace React {
  interface HTMLAttributes<T> {
    "tt-value"?: string;
    "tt-onclick"?: string;
    "tt-onmouseover"?: string;
    "tt-onmouseout"?: string;
    "tt-onmouseenter"?: string;
    "tt-onmouseleave"?: string;
    "tt-onkeypress"?: string;
    "tt-onkeydown"?: string;
    "tt-onload"?: string;
    "tt-onerror"?: string;
    "tt-onlyattr"?: string;

    name?: string;
  }
}

interface CustomProps {
  blogId: string;
  entryId: string;
  filterTarget: boolean;
  role: "user" | "owner";
  trackPage: string;
  userId: string;
}

interface Tiara {
  appUserId: string;
  customProps: CustomProps;
  entry: unknown;
  kakaoAppKey: string;
  key: string;
  page: string;
  section: string;
  svcDomain: string;
  trackPage: string;
}

declare interface Window {
  tiara: Tiara | undefined;
}
