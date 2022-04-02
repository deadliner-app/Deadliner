import Image from "next/image";
import React, { FC } from "react";
import Section from "../components/Section";
import Title from "../components/Title";

interface FeatureProps {
  icon: string;
  text: string;
}
const Feature: FC<FeatureProps> = ({ icon, text }) => {
  let size = 45;
  return (
    <div className="flex justify-start items-center mb-4 xl:mb-10">
      <div className="w-7 h-7 xl:w-9 xl:h-9 2xl:w-icon 2xl:h-icon shrink-0">
        <Image src={`/images/${icon}`} width={size} height={size} alt="" />
      </div>
      <div className="flex-grow">
        <h2 className="ml-3 text-white text-[17.8px] lg:text-2xl xl:text-2xl 2xl:text-3xl">
          {text}
        </h2>
      </div>
    </div>
  );
};

let features: FeatureProps[] = [
  { icon: "deadline.png", text: "Setting a Deadline" },
  { icon: "wallpaper.png", text: "Updating wallpaper with the time left" },
  { icon: "happiness.png", text: "A less annoying way for reminding you" },
  { icon: "color-palette.png", text: "Full Customization" },
  { icon: "teamwork.png", text: "Cross Platform" },
  { icon: "energy.png", text: "Runs efficiently in the background" },
];

const Features = () => {
  return (
    <Section id="features" className="justify-between lg:flex-row">
      <div>
        <Title text="Features" />
        <div>
          {features.map((e) => (
            <Feature key={e.icon} {...e} />
          ))}
        </div>
      </div>
      <div className="mt-10 lg:mt-0 -translate-y-0 lg:-translate-y-4 xl:-translate-y-6 2xl:-translate-y-8 w-full h-full lg:w-72 lg:h-7w-72 xl:w-96 xl:h-96 2xl:w-app_screenshot 2xl:h-app_screenshot">
        <Image src="/images/app.png" width={465} height={766} alt="" />
      </div>
    </Section>
  );
};

export default Features;
