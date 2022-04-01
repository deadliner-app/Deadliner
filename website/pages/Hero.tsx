import Image from "next/image";
import React from "react";
import Section from "../components/Section";

const Hero = () => {
  return (
    <div id="hero" className="flex justify-center items-center grow">
      <Section className="flex-col items-center">
        <h1 className="mb-20 text-white text-huge font-semibold text-center leading-[7.5rem] h-max">
          Say 👋 to procrastination
          <br />
          when you’re on a
          <br />
          Deadline.
        </h1>
        <div className="w-14 h-14 flex justify-center items-center">
          <Image
            className="animate-pulse"
            src="/scroll.svg"
            width={50}
            height={50}
            alt=""
          />
        </div>
      </Section>
    </div>
  );
};

export default Hero;
