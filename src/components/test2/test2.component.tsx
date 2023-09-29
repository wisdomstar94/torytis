import styles from './test2.module.scss';

export function Test2() {
  return (
    <>
      <button tt-onclick="[###]" className={['kkk', styles['box2'], 'w-full text-sm text-slate-700 hover:bg-black'].join(' ')}>
        <tt_html_comment>이건 주석입니다..</tt_html_comment>
        Test Component...!
      </button>
    </>
  );
}