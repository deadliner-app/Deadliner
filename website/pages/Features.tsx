import Image from "next/image";
import React, { FC } from "react";
import Section from "../components/Section";

interface FeatureProps {
  icon: string;
  text: string;
}
const Feature: FC<FeatureProps> = ({ icon, text }) => {
  let size = 45;
  return (
    <div className="flex items-center mb-10">
      <Image src={`/images/${icon}`} width={size} height={size} alt="" />
      <h2 className="ml-3 text-white text-3xl">{text}</h2>
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
    <Section id="features" className="justify-between">
      <div>
        <h1 className="font-semibold text-8xl text-white">Features</h1>
        <div className="mt-14">
          {features.map((e) => (
            <Feature key={e.icon} {...e} />
          ))}
        </div>
      </div>
      <div className="-translate-y-8">
        <Image src="/images/app.png" width={465} height={766} alt="" />
      </div>
    </Section>
  );
};

export default Features;
