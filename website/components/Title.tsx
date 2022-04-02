import React from "react";

const Title = ({ text }: { text: string }) => {
  return (
    <h1 className="font-semibold text-5xl lg:text-6xl xl:text-7xl 2xl:text-8xl text-white mb-10 lg:mb-12 xl:mb-14">
      {text}
    </h1>
  );
};

export default Title;
