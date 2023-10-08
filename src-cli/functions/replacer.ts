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

  // tt-onerror
  convertedHtml = convertedHtml.replace(/tt-onerror/gi, 'onerror');

  // tt-onlyattr
  convertedHtml = convertedHtml.replace(/tt-onlyattr="[^"]*"/gi, ((matchedString: string) => {
    const temp1 = matchedString.replace("tt-onlyattr=\"", "");
    const temp2 = temp1.slice(0, temp1.length - 1);
    return temp2;
  }) as any);

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