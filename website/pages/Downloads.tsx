import Image from "next/image";
import React, { useEffect, useState } from "react";
import Section from "../components/Section";
import Title from "../components/Title";

const Downloads = () => {
  let [link, setLink] = useState(null);

  useEffect(() => {
    fetch("https://api.github.com/repos/YassinEldeeb/deadliner/releases/latest")
      .then((data) => data.json())
      .then((data) => {
        setLink(data.html_url);
      });
  }, []);

  return (
    <Section id="downloads" className="justify-start">
      <Title text="Downloads" />

      {link ? (
        <a href={link}>
          <div
            className="flex items-center cursor-pointer decoration-white"
            title={link}
          >
            <div className="shrink-0 w-6 h-6 lg:w-10 lg:h-10">
              <Image src="/images/link.png" width={40} height={40} alt="" />
            </div>
            <h2 className="ml-4 text-white font-medium text-md lg:text-3xl underline lg:hover:underline">
              Follow the latest github release for your Operating System.
            </h2>
          </div>
        </a>
      ) : (
        <p className="text-white text-md lg:text-2xl">
          Fetching latest github release...
        </p>
      )}
    </Section>
  );
};

export default Downloads;
