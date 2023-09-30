

window.addEventListener('load', () => {
  console.log('index!!!');

  const element = <>
    <div className="this-is">
      <span tt-onclick="...">1</span>
      <span>2</span>
    </div>
  </>;

  console.log('@element', element);
});