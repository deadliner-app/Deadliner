import React, { FC, HTMLAttributes, ReactNode } from "react";

interface Props {
  id?: string;
  children: ReactNode;
  className?: string;
}

const Section: FC<Props> = ({ children, className, id }) => {
  return (
    <div id={id || ""} className="flex justify-center">
      <div
        className={`w-full py-4 lg:py-6 max-w-screen-2xl px-8 lg:px-16 flex justify-center ${
          className ?? ""
        }`}
      >
        {children}
      </div>
    </div>
  );
};

export default Section;
