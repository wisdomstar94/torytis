import createMDX from "@next/mdx";
import rehypePrettyCode from "rehype-pretty-code";
import remarkGfm from "remark-gfm";

/** @type {import('rehype-pretty-code').Options} */
const options = {
  // See Options section below.
  theme: "rose-pine-dawn",
  // theme: "snazzy-light",
  keepBackground: true,
};

/** @type {import('next').NextConfig} */
const nextConfig = {
  pageExtensions: ["js", "jsx", "md", "mdx", "ts", "tsx"],
  basePath: "/torytis",
  output: "export",
  experimental: {
    mdxRs: false,
  },
  trailingSlash: true,
};

const withMDX = createMDX({
  // Add markdown plugins here, as desired
  options: {
    remarkPlugins: [remarkGfm],
    rehypePlugins: [[rehypePrettyCode, options]],
  },
});

export default withMDX(nextConfig);
