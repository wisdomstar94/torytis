export function allInOneReplace(html: string) {
  let convertedHtml = html;
  convertedHtml = ttAttrReplace(convertedHtml);
  convertedHtml = ttHtmlCommentReplace(convertedHtml);
  convertedHtml = metaTagReplace(convertedHtml);
  return convertedHtml;
}

export function ttAttrReplace(html: string) {
  let convertedHtml = html;

  // tt-onclick
  convertedHtml = convertedHtml.replace(/tt-onclick/gi, 'onclick');

  // tt-onkeypress
  convertedHtml = convertedHtml.replace(/tt-onkeypress/gi, 'onkeypress');

  // tt-onkeydown
  convertedHtml = convertedHtml.replace(/tt-onkeydown/gi, 'onkeydown');

  // tt-value
  convertedHtml = convertedHtml.replace(/tt-value/gi, 'value');

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