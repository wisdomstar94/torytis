export function Script(props: { html: string }) {
  const { html } = props;
  return <script dangerouslySetInnerHTML={{ __html: html }}></script>;
}
