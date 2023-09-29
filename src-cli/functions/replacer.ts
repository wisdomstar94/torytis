export function ttAttrReplace(html: string) {
  let convertedHtml = html;

  // tt-onclick
  convertedHtml = convertedHtml.replace(/tt-onclick/gi, 'onclick');

  // tt-onkeypress
  convertedHtml = convertedHtml.replace(/tt-onkeypress/gi, 'onkeypress');

  // tt-onkeydown
  convertedHtml = convertedHtml.replace(/tt-onkeydown/gi, 'onkeydown');

  return convertedHtml;
}

export function ttHtmlCommentReplace(html: string) {
  let convertedHtml = html;
  
  convertedHtml = convertedHtml.replace(/<tt_html_comment>/gi, '<!--');
  convertedHtml = convertedHtml.replace(/<\/tt_html_comment>/gi, '-->');

  return convertedHtml;
}

export function metaTagReplace(html: string) {
  let convertedHtml = html;
  
  convertedHtml = convertedHtml.replace(/<meta charSet/gi, '<meta charset');

  return convertedHtml;
}