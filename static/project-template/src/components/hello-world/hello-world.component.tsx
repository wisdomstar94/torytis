import styles from "./hello-world.module.scss";

export function HelloWorld() {
  return (
    <div id="hello-world" className={styles["text"]}>
      hello world!!!
    </div>
  );
}
