export function Style(props: { html: string }) {
  const { html } = props;
  return <style dangerouslySetInnerHTML={{ __html: html }}></style>;
}
