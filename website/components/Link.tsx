import { FC } from "react";

interface Props {
  href: string;
  text: string;
}
export const Link: FC<Props> = ({ href, text }) => {
  return (
    <a
      className="text-white lg:text-grey font-medium opacity-90
      hover:underline hover:opacity-75 transition-all duration-100 text-3xl lg:text-xl"
      href={href}
    >
      {text}
    </a>
  );
};
