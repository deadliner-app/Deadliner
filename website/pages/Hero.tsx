import Image from "next/image";
import React from "react";
import Section from "../components/Section";

const Hero = () => {
  return (
    <div id="hero" className="flex justify-center items-center grow">
      <Section className="flex-col items-center">
        <h1 className="mb-20 text-white text-[34px] md:text-[50px] lg:text-[65px] lg:leading-[5.5rem] xl:text-[80px] xl:leading-[7.5rem] 2xl:text-huge font-semibold text-center h-max">
          Say 👋 to procrastination
          <br />
          when you’re on a
          <br />
          Deadline.
        </h1>
        <div className="2xl:w-14 2xl:h-14 xl:w-10 xl:h-10 w-8 h-8 flex justify-center items-center">
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
