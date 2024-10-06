import { cn } from "@/utils/common";
import Link from "next/link";

export default function Page() {
  return (
    <>
      <div className="w-full h-full fixed top-0 left-0 flex items-center justify-center bg-blue-200">
        <div className="w-full max-w-[600px] max-h-[600px] m-4 flex flex-col items-center gap-8 relative">
          {/* logo title */}
          <div className="text-4xl font-thin">torytis</div>

          {/* description */}
          <div className="text-base text-center">torytis 는 티스토리 스킨 개발의 편의성을 높여주는 rust 로 만들어진 프레임워크 입니다.</div>

          {/* buttons */}
          <div className="inline-flex gap-6 relative">
            <Link
              href="https://github.com/wisdomstar94/torytis"
              className={cn(
                "inline-flex px-6 py-3 text-base rounded-full relative border cursor-pointer",
                "border-purple-500 text-purple-500",
                "hover:bg-purple-600/10"
                //
              )}
            >
              Github
            </Link>
            <Link
              href={`/intro`}
              className={cn(
                "inline-flex px-6 py-3 text-base rounded-full relative border cursor-pointer",
                "border-purple-500 bg-purple-500 text-white",
                "hover:bg-purple-600"
                //
              )}
            >
              Get started
            </Link>
          </div>
        </div>
      </div>
    </>
  );
}
